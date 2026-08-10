//! Structures, signatures and signature matching.
//!
//! A structure elaborates to a `StructEnv`: the values, types and
//! substructures it exports. A signature elaborates to a `Sig`, in which each
//! abstract type is a "flexible" type constructor with a stamp of its own.
//! Matching a structure against a signature finds what each flexible
//! constructor stands for (the realization), checks every specification
//! against it, and builds the restricted structure. Transparent ascription
//! keeps the identity of the structure's types; opaque ascription makes a
//! fresh type for each abstract one, so nothing outside can see through it.

use std::collections::HashMap;
use std::rc::Rc;

use miette::SourceSpan;

use super::{
    Alias, DataInfo, Entry, Infer, Mark, Res, Scheme, TyCon, Type, TypeEntry, TypeError, arrow, con,
};
use crate::parser::{
    FunctorBinding, FunctorParameter, SigBinding, SigExp, SigExpKind, Spec, SpecKind, StrExp,
    StrExpKind, StructBinding, WhereType,
};

/// What a structure exports.
#[derive(Clone, Default)]
pub(super) struct StructEnv {
    pub(super) values: Vec<Entry>,
    pub(super) types: Vec<(String, TypeEntry)>,
    pub(super) structs: Vec<(String, StructEnv)>,
}

#[derive(Clone)]
pub(super) struct Sig {
    items: Vec<SigItem>,
}

#[derive(Clone)]
enum SigItem {
    Val {
        name: String,
        scheme: Scheme,
    },
    /// `type t`, `eqtype t` (flexible: no definition) or `type t = ty`.
    Type {
        name: String,
        params: Vec<usize>,
        equality: bool,
        tycon: TyCon,
        definition: Option<Type>,
    },
    /// A `datatype` specification, or a replication of one (`copy`), whose
    /// type constructor is not flexible.
    Datatype {
        name: String,
        tycon: TyCon,
        arity: usize,
        constructors: Vec<(String, Scheme)>,
        copy: bool,
    },
    Exception {
        name: String,
        argument: Option<Type>,
    },
    Struct {
        name: String,
        sig: Sig,
    },
}

/// A functor: its parameter, its body, and the environment it was declared
/// in. Each application elaborates the body again in that environment, with
/// the parameter bound to the argument, so datatypes the body declares are
/// new types each time.
pub(super) struct Functor {
    /// `None` when the parameter is a list of specifications, whose names
    /// the body sees unqualified.
    parameter: Option<String>,
    sig: Sig,
    body: StrExp,
    scope: Scope,
}

/// A copy of the environment a functor was declared in.
struct Scope {
    values: Vec<Entry>,
    types: Vec<(String, TypeEntry)>,
    structs: Vec<(String, StructEnv)>,
    signatures: Vec<(String, Sig)>,
    functors: Vec<(String, Rc<Functor>)>,
}

/// What each flexible type constructor (by stamp) stands for: a type over
/// the given parameters.
type Realization = HashMap<usize, (Vec<usize>, Type)>;

impl Infer {
    // --- declarations ------------------------------------------------------

    pub(super) fn infer_structures(&mut self, bindings: &[StructBinding]) -> Res<()> {
        let mut built = Vec::new();
        for binding in bindings {
            let start = self.next_stamp;
            let env = self.elab_strexp(&binding.body)?;
            self.assign_paths(&env, &binding.name, start);
            built.push((binding.name.clone(), env));
        }
        self.structs.extend(built);
        Ok(())
    }

    pub(super) fn infer_functors(&mut self, bindings: &[FunctorBinding]) -> Res<()> {
        let mut built = Vec::new();
        for binding in bindings {
            let (parameter, signature) = match &binding.parameter {
                FunctorParameter::Named(name, signature) => (Some(name.clone()), signature),
                FunctorParameter::Specs(signature) => (None, signature),
            };
            let sig = self.elab_sigexp(signature)?;
            // The body must make sense for any argument that matches, so it
            // is checked once with the parameter's abstract types.
            let mark = self.mark();
            let env = self.sig_env(&sig.items);
            // Errors in the body name the parameter's types `X.t`.
            if let Some(name) = &parameter {
                self.assign_paths(&env, name, 0);
            }
            self.bind_parameter(parameter.as_deref(), env);
            let checked = self.elab_strexp(&binding.body);
            self.release(mark);
            checked?;
            let scope = Scope {
                values: self.values.clone(),
                types: self.types.clone(),
                structs: self.structs.clone(),
                signatures: self.signatures.clone(),
                functors: self.functors.clone(),
            };
            built.push((
                binding.name.clone(),
                Rc::new(Functor {
                    parameter,
                    sig,
                    body: binding.body.clone(),
                    scope,
                }),
            ));
        }
        self.functors.extend(built);
        Ok(())
    }

    fn bind_parameter(&mut self, parameter: Option<&str>, env: StructEnv) {
        match parameter {
            Some(name) => self.structs.push((name.to_string(), env)),
            None => self.extend_env(&env),
        }
    }

    /// Replaces the environment with `scope`, returning the old one.
    fn swap_scope(&mut self, scope: Scope) -> Scope {
        Scope {
            values: std::mem::replace(&mut self.values, scope.values),
            types: std::mem::replace(&mut self.types, scope.types),
            structs: std::mem::replace(&mut self.structs, scope.structs),
            signatures: std::mem::replace(&mut self.signatures, scope.signatures),
            functors: std::mem::replace(&mut self.functors, scope.functors),
        }
    }

    pub(super) fn infer_signatures(&mut self, bindings: &[SigBinding]) -> Res<()> {
        let mut built = Vec::new();
        for binding in bindings {
            built.push((binding.name.clone(), self.elab_sigexp(&binding.body)?));
        }
        self.signatures.extend(built);
        Ok(())
    }

    pub(super) fn open_structures(&mut self, paths: &[String], span: SourceSpan) -> Res<()> {
        for path in paths {
            let Some(env) = self.lookup_struct(path).cloned() else {
                return Err((TypeError::UnboundStructure(path.clone()), span));
            };
            self.extend_env(&env);
        }
        Ok(())
    }

    fn extend_env(&mut self, env: &StructEnv) {
        self.values.extend(env.values.iter().cloned());
        self.types.extend(env.types.iter().cloned());
        self.structs.extend(env.structs.iter().cloned());
    }

    /// Remembers the qualified name of each type constructor the structure
    /// created, so it can be printed as `S.t`.
    fn assign_paths(&mut self, env: &StructEnv, prefix: &str, start: usize) {
        for (name, entry) in &env.types {
            if let TypeEntry::Data { tycon, .. } = entry
                && tycon.stamp > start
            {
                self.paths.insert(tycon.stamp, format!("{prefix}.{name}"));
            }
        }
        for (name, inner) in &env.structs {
            self.assign_paths(inner, &format!("{prefix}.{name}"), start);
        }
    }

    // --- structure expressions ---------------------------------------------

    fn elab_strexp(&mut self, exp: &StrExp) -> Res<StructEnv> {
        let mut application = None;
        let mut application_parameter = None;
        let env = match &exp.value {
            StrExpKind::Struct(declarations) => {
                let mark = self.mark();
                for declaration in declarations {
                    self.infer_decl(declaration)?;
                }
                let env = self.collect_since(mark);
                self.release(mark);
                Ok(env)
            }
            StrExpKind::Apply(name, argument) => {
                let Some(functor) = self
                    .functors
                    .iter()
                    .rev()
                    .find(|(known, _)| known == name)
                    .map(|(_, functor)| functor.clone())
                else {
                    return Err((TypeError::UnboundFunctor(name.clone()), exp.source_span()));
                };
                let actual = self.elab_strexp(argument)?;
                let parameter =
                    self.ascribe(&actual, &functor.sig, false, argument.source_span())?;
                application_parameter = Some(Box::new(exports(self, &parameter)));
                let saved = self.swap_scope(Scope {
                    values: functor.scope.values.clone(),
                    types: functor.scope.types.clone(),
                    structs: functor.scope.structs.clone(),
                    signatures: functor.scope.signatures.clone(),
                    functors: functor.scope.functors.clone(),
                });
                self.bind_parameter(functor.parameter.as_deref(), parameter);
                let body = Rc::new(functor.body.clone());
                let result = self.elab_strexp(&body);
                application = Some(body);
                self.swap_scope(saved);
                result
            }
            StrExpKind::Name(path) => self
                .lookup_struct(path)
                .cloned()
                .ok_or_else(|| (TypeError::UnboundStructure(path.clone()), exp.source_span())),
            StrExpKind::Ascribed {
                body,
                signature,
                opaque,
            } => {
                let env = self.elab_strexp(body)?;
                let sig = self.elab_sigexp(signature)?;
                self.ascribe(&env, &sig, *opaque, exp.source_span())
            }
            StrExpKind::Let(declarations, inner) => {
                let mark = self.mark();
                for declaration in declarations {
                    self.infer_decl(declaration)?;
                }
                let env = self.elab_strexp(inner)?;
                self.release(mark);
                Ok(env)
            }
        }?;
        fn exports(infer: &Infer, env: &StructEnv) -> super::StructureInfo {
            super::StructureInfo {
                values: env
                    .values
                    .iter()
                    .map(|entry| super::Export {
                        name: entry.name.clone(),
                        constructor: entry.constructor,
                        carries: matches!(infer.prune(&entry.scheme.ty), Type::Arrow(..)),
                    })
                    .collect(),
                structures: env
                    .structs
                    .iter()
                    .map(|(name, env)| (name.clone(), exports(infer, env)))
                    .collect(),
                application: None,
                parameter: None,
            }
        }
        let mut info = exports(self, &env);
        info.application = application;
        info.parameter = application_parameter;
        self.structure_info.insert(exp as *const _ as usize, info);
        Ok(env)
    }

    /// What was bound since `mark`; a later binding of a name hides earlier ones.
    fn collect_since(&self, mark: Mark) -> StructEnv {
        fn latest<T>(items: &[T], name: impl Fn(&T) -> &str) -> Vec<T>
        where
            T: Clone,
        {
            let mut seen: Vec<&str> = Vec::new();
            let mut kept = Vec::new();
            for item in items.iter().rev() {
                if !seen.contains(&name(item)) {
                    seen.push(name(item));
                    kept.push(item.clone());
                }
            }
            kept.reverse();
            kept
        }
        StructEnv {
            values: latest(&self.values[mark.values..], |entry| &entry.name),
            types: latest(&self.types[mark.types..], |(name, _)| name),
            structs: latest(&self.structs[mark.structs..], |(name, _)| name),
        }
    }

    // --- signature expressions ---------------------------------------------

    fn elab_sigexp(&mut self, sig: &SigExp) -> Res<Sig> {
        match &sig.value {
            SigExpKind::Name(name) => self
                .signatures
                .iter()
                .rev()
                .find(|(known, _)| known == name)
                .map(|(_, sig)| sig.clone())
                .ok_or_else(|| (TypeError::UnboundSignature(name.clone()), sig.source_span())),
            SigExpKind::Sig(specs) => {
                let base = self.mark();
                let mut items = Vec::new();
                let result = self.elab_specs(specs, base, &mut items);
                self.release(base);
                result?;
                Ok(Sig { items })
            }
            SigExpKind::Where(inner, refinements) => {
                let mut sig_inner = self.elab_sigexp(inner)?;
                for refinement in refinements {
                    self.refine(&mut sig_inner, refinement, sig.source_span())?;
                }
                Ok(sig_inner)
            }
        }
    }

    /// The specifications see the ones before them, so after each the
    /// environment is rebuilt from the items so far.
    fn elab_specs(&mut self, specs: &[Spec], base: Mark, items: &mut Vec<SigItem>) -> Res<()> {
        for spec in specs {
            self.elab_spec(spec, items)?;
            self.release(base);
            let env = self.sig_env(items);
            self.extend_env(&env);
        }
        Ok(())
    }

    fn elab_spec(&mut self, spec: &Spec, items: &mut Vec<SigItem>) -> Res<()> {
        match &spec.value {
            SpecKind::Val(values) => {
                for (name, ty) in values {
                    self.level += 1;
                    let converted = self.convert(ty, &mut HashMap::new());
                    self.level -= 1;
                    let scheme = self.generalize(&converted?);
                    items.push(SigItem::Val {
                        name: name.clone(),
                        scheme,
                    });
                }
            }
            SpecKind::Type(specs) => {
                for spec in specs {
                    let mut variables = HashMap::new();
                    let mut params = Vec::new();
                    for parameter in &spec.parameters {
                        let ty = self.fresh();
                        if let Type::Var(id) = &ty {
                            params.push(*id);
                        }
                        variables.insert(parameter.clone(), ty);
                    }
                    let definition = match &spec.definition {
                        Some(ty) => {
                            let body = self.convert(ty, &mut variables)?;
                            self.check_closed(&variables, params.len(), &spec.parameters, ty)?;
                            Some(body)
                        }
                        None => None,
                    };
                    self.next_stamp += 1;
                    let tycon = TyCon {
                        name: spec.name.clone(),
                        stamp: self.next_stamp,
                    };
                    self.datatypes.insert(
                        tycon.stamp,
                        DataInfo {
                            constructors: Vec::new(),
                            never_equal: !spec.equality,
                            equal_params: vec![true; params.len()],
                        },
                    );
                    items.push(SigItem::Type {
                        name: spec.name.clone(),
                        params,
                        equality: spec.equality,
                        tycon,
                        definition,
                    });
                }
            }
            SpecKind::Datatype(bindings) => {
                let tycons = self.infer_datatypes(bindings, &[])?;
                for (binding, tycon) in bindings.iter().zip(tycons) {
                    let constructors = self.datatypes[&tycon.stamp].constructors.clone();
                    items.push(SigItem::Datatype {
                        name: binding.name.clone(),
                        tycon,
                        arity: binding.parameters.len(),
                        constructors,
                        copy: false,
                    });
                }
            }
            SpecKind::DatatypeCopy { name, original } => {
                let Some(TypeEntry::Data { tycon, arity }) = self.lookup_type(original) else {
                    return Err((TypeError::UnboundType(original.clone()), spec.source_span()));
                };
                let constructors = self
                    .datatypes
                    .get(&tycon.stamp)
                    .map(|info| info.constructors.clone())
                    .unwrap_or_default();
                items.push(SigItem::Datatype {
                    name: name.clone(),
                    tycon,
                    arity,
                    constructors,
                    copy: true,
                });
            }
            SpecKind::Exception(exceptions) => {
                for (name, argument) in exceptions {
                    let argument = match argument {
                        Some(ty) => Some(self.convert(ty, &mut HashMap::new())?),
                        None => None,
                    };
                    items.push(SigItem::Exception {
                        name: name.clone(),
                        argument,
                    });
                }
            }
            SpecKind::Structure(structures) => {
                for (name, signature) in structures {
                    let sig = self.elab_sigexp(signature)?;
                    items.push(SigItem::Struct {
                        name: name.clone(),
                        sig,
                    });
                }
            }
            SpecKind::Include(signature) => {
                let sig = self.elab_sigexp(signature)?;
                items.extend(sig.items);
            }
            SpecKind::Sharing(names) => self.share(items, names, spec.source_span())?,
            SpecKind::SharingStructures(paths) => {
                self.share_structures(items, paths, spec.source_span())?;
            }
        }
        Ok(())
    }

    /// The environment a signature's specifications bring into scope.
    fn sig_env(&self, items: &[SigItem]) -> StructEnv {
        let mut env = StructEnv::default();
        for item in items {
            match item {
                SigItem::Val { name, scheme } => env.values.push(Entry {
                    name: name.clone(),
                    scheme: scheme.clone(),
                    constructor: false,
                }),
                SigItem::Type {
                    name,
                    params,
                    tycon,
                    definition,
                    ..
                } => env.types.push((
                    name.clone(),
                    match definition {
                        Some(body) => TypeEntry::Alias(Alias {
                            params: params.clone(),
                            body: body.clone(),
                        }),
                        None => TypeEntry::Data {
                            tycon: tycon.clone(),
                            arity: params.len(),
                        },
                    },
                )),
                SigItem::Datatype {
                    name,
                    tycon,
                    arity,
                    constructors,
                    ..
                } => {
                    env.types.push((
                        name.clone(),
                        TypeEntry::Data {
                            tycon: tycon.clone(),
                            arity: *arity,
                        },
                    ));
                    for (constructor, scheme) in constructors {
                        env.values.push(Entry {
                            name: constructor.clone(),
                            scheme: scheme.clone(),
                            constructor: true,
                        });
                    }
                }
                SigItem::Exception { name, argument } => env.values.push(Entry {
                    name: name.clone(),
                    scheme: exception_scheme(argument),
                    constructor: true,
                }),
                SigItem::Struct { name, sig } => {
                    env.structs.push((name.clone(), self.sig_env(&sig.items)));
                }
            }
        }
        env
    }

    // --- where type and sharing --------------------------------------------

    /// The type a specification of `path` stands for, as a closure over its
    /// parameters, and its stamp if it is still flexible.
    fn type_closure(
        &mut self,
        items: &[SigItem],
        path: &str,
    ) -> Option<(Vec<usize>, Type, Option<usize>, bool)> {
        let (prefix, base) = match path.rsplit_once('.') {
            Some((prefix, base)) => (Some(prefix), base),
            None => (None, path),
        };
        let mut scope = items;
        if let Some(prefix) = prefix {
            for part in prefix.split('.') {
                scope = scope.iter().rev().find_map(|item| match item {
                    SigItem::Struct { name, sig } if name == part => Some(sig.items.as_slice()),
                    _ => None,
                })?;
            }
        }
        let found = scope.iter().rev().find(|item| {
            matches!(item, SigItem::Type { name, .. } | SigItem::Datatype { name, .. } if name == base)
        })?;
        match found.clone() {
            SigItem::Type {
                params,
                tycon,
                equality,
                definition,
                ..
            } => match definition {
                Some(body) => Some((params, body, None, equality)),
                None => {
                    let args = params.iter().map(|id| Type::Var(*id)).collect();
                    Some((
                        params,
                        Type::Con(tycon.clone(), args),
                        Some(tycon.stamp),
                        equality,
                    ))
                }
            },
            SigItem::Datatype { tycon, arity, .. } => {
                let params: Vec<usize> = (0..arity)
                    .map(|_| match self.fresh() {
                        Type::Var(id) => id,
                        _ => unreachable!("fresh returns a variable"),
                    })
                    .collect();
                let args = params.iter().map(|id| Type::Var(*id)).collect();
                Some((params, Type::Con(tycon, args), None, false))
            }
            _ => None,
        }
    }

    /// `sig ... end where type path = ty`.
    fn refine(&mut self, sig: &mut Sig, refinement: &WhereType, span: SourceSpan) -> Res<()> {
        let bad = |reason: &str| {
            (
                TypeError::BadRefinement(refinement.name.clone(), reason.to_string()),
                span,
            )
        };
        let Some((params, _, Some(stamp), equality)) =
            self.type_closure(&sig.items, &refinement.name)
        else {
            return Err(bad("it is not an abstract type of the signature"));
        };
        if params.len() != refinement.parameters.len() {
            return Err(bad("the number of type parameters differs"));
        }
        let mut variables = HashMap::new();
        let mut ids = Vec::new();
        for parameter in &refinement.parameters {
            let ty = self.fresh();
            if let Type::Var(id) = &ty {
                ids.push(*id);
            }
            variables.insert(parameter.clone(), ty);
        }
        let body = self.convert(&refinement.ty, &mut variables)?;
        self.check_closed(
            &variables,
            ids.len(),
            &refinement.parameters,
            &refinement.ty,
        )?;
        if equality {
            let instance = self.without_parameters(&ids, &body);
            if self.require_equality(&instance).is_err() {
                return Err(bad("the type does not admit equality"));
            }
        }
        let mut real = Realization::new();
        real.insert(stamp, (ids, body));
        self.apply_realization(&mut sig.items, &real);
        Ok(())
    }

    /// `sharing type t1 = t2 = ...`: the types become one.
    fn share(&mut self, items: &mut [SigItem], paths: &[String], span: SourceSpan) -> Res<()> {
        let mut closures = Vec::new();
        for path in paths {
            let Some(closure) = self.type_closure(items, path) else {
                return Err((
                    TypeError::BadRefinement(path.clone(), "it is not a type specification".into()),
                    span,
                ));
            };
            closures.push(closure);
        }
        // A type with a definition (or a datatype) is the one the others
        // become; otherwise the first.
        let canonical = closures
            .iter()
            .position(|(_, _, stamp, _)| stamp.is_none())
            .unwrap_or(0);
        let (canon_params, canon_body, ..) = closures[canonical].clone();
        let mut real = Realization::new();
        for (index, (params, body, stamp, _)) in closures.iter().enumerate() {
            if params.len() != canon_params.len() {
                return Err((
                    TypeError::BadRefinement(
                        paths[index].clone(),
                        "the types take different numbers of parameters".into(),
                    ),
                    span,
                ));
            }
            if index == canonical {
                continue;
            }
            match stamp {
                Some(stamp) => {
                    real.insert(*stamp, (canon_params.clone(), canon_body.clone()));
                }
                None => {
                    let same = self.same_type(
                        &(params.clone(), body.clone()),
                        &(canon_params.clone(), canon_body.clone()),
                    );
                    if !same {
                        return Err((
                            TypeError::BadRefinement(
                                paths[index].clone(),
                                "it is already a different type".into(),
                            ),
                            span,
                        ));
                    }
                }
            }
        }
        self.apply_realization(items, &real);
        Ok(())
    }

    /// `sharing A = B`: each type both structures specify becomes one type.
    fn share_structures(
        &mut self,
        items: &mut [SigItem],
        paths: &[String],
        span: SourceSpan,
    ) -> Res<()> {
        let mut specified = Vec::new();
        for path in paths {
            let Some(found) = struct_items(items, path) else {
                return Err((
                    TypeError::BadRefinement(
                        path.clone(),
                        "it is not a structure specification".into(),
                    ),
                    span,
                ));
            };
            let mut names = Vec::new();
            type_paths(found, "", &mut names);
            specified.push(names);
        }
        let common: Vec<String> = specified[0]
            .iter()
            .filter(|name| specified[1..].iter().all(|other| other.contains(name)))
            .cloned()
            .collect();
        for name in common {
            let shared: Vec<String> = paths.iter().map(|path| format!("{path}.{name}")).collect();
            self.share(items, &shared, span)?;
        }
        Ok(())
    }

    /// Substitutes the realization into a signature. A flexible type that is
    /// realized becomes a definition.
    fn apply_realization(&self, items: &mut [SigItem], real: &Realization) {
        for item in items {
            match item {
                SigItem::Val { scheme, .. } => *scheme = self.realize_scheme(scheme, real),
                SigItem::Type {
                    params,
                    tycon,
                    definition,
                    ..
                } => match definition {
                    Some(body) => *body = self.realize(body, real),
                    None => {
                        if let Some((given, body)) = real.get(&tycon.stamp) {
                            let map: HashMap<usize, Type> = given
                                .iter()
                                .copied()
                                .zip(params.iter().map(|id| Type::Var(*id)))
                                .collect();
                            *definition = Some(self.substitute(body, &map));
                        }
                    }
                },
                SigItem::Datatype { constructors, .. } => {
                    for (_, scheme) in constructors {
                        *scheme = self.realize_scheme(scheme, real);
                    }
                }
                SigItem::Exception { argument, .. } => {
                    if let Some(argument) = argument {
                        *argument = self.realize(argument, real);
                    }
                }
                SigItem::Struct { sig, .. } => self.apply_realization(&mut sig.items, real),
            }
        }
    }

    fn realize(&self, ty: &Type, real: &Realization) -> Type {
        match self.prune(ty) {
            Type::Var(id) => Type::Var(id),
            Type::Con(tycon, args) => {
                let args: Vec<Type> = args.iter().map(|arg| self.realize(arg, real)).collect();
                match real.get(&tycon.stamp) {
                    Some((params, body)) if tycon.stamp != 0 => {
                        let map: HashMap<usize, Type> = params.iter().copied().zip(args).collect();
                        self.substitute(body, &map)
                    }
                    _ => Type::Con(tycon, args),
                }
            }
            Type::Arrow(from, to) => arrow(self.realize(&from, real), self.realize(&to, real)),
            Type::Record(fields) => Type::Record(
                fields
                    .iter()
                    .map(|(label, ty)| (label.clone(), self.realize(ty, real)))
                    .collect(),
            ),
        }
    }

    fn realize_scheme(&self, scheme: &Scheme, real: &Realization) -> Scheme {
        Scheme {
            vars: scheme.vars.clone(),
            ty: self.realize(&scheme.ty, real),
        }
    }

    // --- type helpers --------------------------------------------------------

    /// A type constant that only equals itself, standing for a bound variable.
    fn skolem(&mut self, equality: bool) -> Type {
        self.next_stamp += 1;
        let tycon = TyCon {
            name: "'a".into(),
            stamp: self.next_stamp,
        };
        self.datatypes.insert(
            tycon.stamp,
            DataInfo {
                constructors: Vec::new(),
                never_equal: !equality,
                equal_params: Vec::new(),
            },
        );
        Type::Con(tycon, Vec::new())
    }

    /// `body` with each parameter replaced by a type admitting equality, for
    /// asking whether the rest of it does.
    fn without_parameters(&mut self, params: &[usize], body: &Type) -> Type {
        let map: HashMap<usize, Type> = params.iter().map(|id| (*id, self.skolem(true))).collect();
        self.substitute(body, &map)
    }

    /// Whether two parameterised types are the same type.
    fn same_type(&mut self, a: &(Vec<usize>, Type), b: &(Vec<usize>, Type)) -> bool {
        if a.0.len() != b.0.len() {
            return false;
        }
        let mut left = HashMap::new();
        let mut right = HashMap::new();
        for (x, y) in a.0.iter().zip(&b.0) {
            let shared = self.skolem(false);
            left.insert(*x, shared.clone());
            right.insert(*y, shared);
        }
        let (left, right) = (self.substitute(&a.1, &left), self.substitute(&b.1, &right));
        self.unify(&left, &right).is_ok()
    }

    /// The actual type a structure gives `name`, as a closure.
    fn actual_type(&mut self, entry: &TypeEntry) -> (Vec<usize>, Type) {
        match entry {
            TypeEntry::Alias(alias) => (alias.params.clone(), alias.body.clone()),
            TypeEntry::Data { tycon, arity } => {
                let params: Vec<usize> = (0..*arity)
                    .map(|_| match self.fresh() {
                        Type::Var(id) => id,
                        _ => unreachable!("fresh returns a variable"),
                    })
                    .collect();
                let args = params.iter().map(|id| Type::Var(*id)).collect();
                (params, Type::Con(tycon.clone(), args))
            }
        }
    }

    // --- signature matching --------------------------------------------------

    fn ascribe(
        &mut self,
        env: &StructEnv,
        sig: &Sig,
        opaque: bool,
        span: SourceSpan,
    ) -> Res<StructEnv> {
        let mut real = Realization::new();
        self.realize_types(env, &sig.items, &mut real, span)?;
        let mut sealed = real.clone();
        if opaque {
            self.seal(&sig.items, &real, &mut sealed);
        }
        self.conform(env, &sig.items, &real, &sealed, opaque, span)
    }

    /// Finds what each flexible type of the specifications is in `env`, and
    /// checks the definitions and datatypes agree.
    fn realize_types(
        &mut self,
        env: &StructEnv,
        items: &[SigItem],
        real: &mut Realization,
        span: SourceSpan,
    ) -> Res<()> {
        for item in items {
            match item {
                SigItem::Type {
                    name,
                    params,
                    equality,
                    tycon,
                    definition,
                    ..
                } => {
                    let Some(entry) = latest_type(env, name) else {
                        return Err((
                            TypeError::MissingSpecification(format!("type {name}")),
                            span,
                        ));
                    };
                    let (actual_params, actual_body) = self.actual_type(&entry);
                    if actual_params.len() != params.len() {
                        return Err(mismatch(
                            format!("type {name}"),
                            "the number of type parameters differs",
                            span,
                        ));
                    }
                    match definition {
                        None => {
                            if *equality {
                                let instance =
                                    self.without_parameters(&actual_params, &actual_body);
                                if self.require_equality(&instance).is_err() {
                                    return Err(mismatch(
                                        format!("type {name}"),
                                        "it is specified with eqtype but does not admit equality",
                                        span,
                                    ));
                                }
                            }
                            real.insert(tycon.stamp, (actual_params, actual_body));
                        }
                        Some(body) => {
                            let wanted = self.realize(body, real);
                            if !self
                                .same_type(&(params.clone(), wanted), &(actual_params, actual_body))
                            {
                                return Err(mismatch(
                                    format!("type {name}"),
                                    "the definition differs from the one in the signature",
                                    span,
                                ));
                            }
                        }
                    }
                }
                SigItem::Datatype {
                    name,
                    tycon,
                    arity,
                    copy,
                    ..
                } => {
                    let Some(TypeEntry::Data {
                        tycon: actual,
                        arity: actual_arity,
                    }) = latest_type(env, name)
                    else {
                        return Err((
                            TypeError::MissingSpecification(format!("datatype {name}")),
                            span,
                        ));
                    };
                    if actual_arity != *arity {
                        return Err(mismatch(
                            format!("datatype {name}"),
                            "the number of type parameters differs",
                            span,
                        ));
                    }
                    let params: Vec<usize> = (0..*arity)
                        .map(|_| match self.fresh() {
                            Type::Var(id) => id,
                            _ => unreachable!("fresh returns a variable"),
                        })
                        .collect();
                    let args: Vec<Type> = params.iter().map(|id| Type::Var(*id)).collect();
                    let body = Type::Con(actual, args.clone());
                    if *copy {
                        let wanted = self.realize(&Type::Con(tycon.clone(), args), real);
                        if !self.same_type(&(params.clone(), wanted), &(params, body)) {
                            return Err(mismatch(
                                format!("datatype {name}"),
                                "it is not a replication of the specified datatype",
                                span,
                            ));
                        }
                    } else {
                        real.insert(tycon.stamp, (params, body));
                    }
                }
                SigItem::Struct { name, sig } => {
                    let Some(inner) = latest_struct(env, name) else {
                        return Err((
                            TypeError::MissingSpecification(format!("structure {name}")),
                            span,
                        ));
                    };
                    self.realize_types(&inner, &sig.items, real, span)?;
                }
                SigItem::Val { .. } | SigItem::Exception { .. } => {}
            }
        }
        Ok(())
    }

    /// For opaque ascription: a fresh type constructor for each abstract type
    /// and datatype, which is what the sealed structure exports.
    fn seal(&mut self, items: &[SigItem], real: &Realization, sealed: &mut Realization) {
        for item in items {
            let flexible = match item {
                SigItem::Type {
                    tycon,
                    definition: None,
                    ..
                }
                | SigItem::Datatype {
                    tycon, copy: false, ..
                } => Some(tycon),
                SigItem::Struct { sig, .. } => {
                    self.seal(&sig.items, real, sealed);
                    None
                }
                _ => None,
            };
            let Some(tycon) = flexible else { continue };
            let arity = real[&tycon.stamp].0.len();
            self.next_stamp += 1;
            let fresh = TyCon {
                name: tycon.name.clone(),
                stamp: self.next_stamp,
            };
            let vars: Vec<usize> = (0..arity)
                .map(|_| match self.fresh() {
                    Type::Var(id) => id,
                    _ => unreachable!("fresh returns a variable"),
                })
                .collect();
            let args = vars.iter().map(|id| Type::Var(*id)).collect();
            sealed.insert(tycon.stamp, (vars, Type::Con(fresh, args)));
        }
    }

    /// Checks each specification against `env` and builds the restricted
    /// structure. `real` says what the flexible types are for the checks;
    /// `sealed` is what they are for the exported types.
    fn conform(
        &mut self,
        env: &StructEnv,
        items: &[SigItem],
        real: &Realization,
        sealed: &Realization,
        opaque: bool,
        span: SourceSpan,
    ) -> Res<StructEnv> {
        let mut out = StructEnv::default();
        for item in items {
            match item {
                SigItem::Val { name, scheme } => {
                    let Some(actual) = latest_value(env, name) else {
                        return Err((
                            TypeError::MissingSpecification(format!("value {name}")),
                            span,
                        ));
                    };
                    let wanted = self.realize_scheme(scheme, real);
                    self.check_generality(&actual.scheme, &wanted, &format!("value {name}"), span)?;
                    out.values.push(Entry {
                        name: name.clone(),
                        scheme: self.realize_scheme(scheme, sealed),
                        constructor: false,
                    });
                }
                SigItem::Type {
                    name,
                    params,
                    tycon,
                    definition,
                    ..
                } => {
                    let entry = match definition {
                        Some(body) => TypeEntry::Alias(Alias {
                            params: params.clone(),
                            body: self.realize(body, sealed),
                        }),
                        None if opaque => {
                            let fresh = sealed_tycon(sealed, tycon.stamp);
                            let equality = matches!(
                                self.datatypes.get(&tycon.stamp),
                                Some(info) if !info.never_equal
                            );
                            self.datatypes.insert(
                                fresh.stamp,
                                DataInfo {
                                    constructors: Vec::new(),
                                    never_equal: !equality,
                                    equal_params: vec![true; params.len()],
                                },
                            );
                            TypeEntry::Data {
                                tycon: fresh,
                                arity: params.len(),
                            }
                        }
                        None => latest_type(env, name).expect("checked when realizing"),
                    };
                    out.types.push((name.clone(), entry));
                }
                SigItem::Datatype {
                    name,
                    tycon,
                    arity,
                    constructors,
                    copy,
                } => {
                    let Some(TypeEntry::Data { tycon: actual, .. }) = latest_type(env, name) else {
                        unreachable!("checked when realizing");
                    };
                    let info = self.datatypes.get(&actual.stamp);
                    let actual_constructors = info
                        .map(|info| info.constructors.clone())
                        .unwrap_or_default();
                    let (never_equal, equal_params) = info
                        .map(|info| (info.never_equal, info.equal_params.clone()))
                        .unwrap_or((false, Vec::new()));
                    if actual_constructors.len() != constructors.len() {
                        return Err(mismatch(
                            format!("datatype {name}"),
                            "its constructors differ from the specification",
                            span,
                        ));
                    }
                    let mut exported = Vec::new();
                    for (constructor, scheme) in constructors {
                        let Some((_, found)) = actual_constructors
                            .iter()
                            .find(|(known, _)| known == constructor)
                        else {
                            return Err(mismatch(
                                format!("datatype {name}"),
                                &format!("constructor {constructor} is missing"),
                                span,
                            ));
                        };
                        let wanted = self.realize_scheme(scheme, real);
                        self.check_generality(
                            found,
                            &wanted,
                            &format!("constructor {constructor}"),
                            span,
                        )?;
                        exported.push((constructor.clone(), self.realize_scheme(scheme, sealed)));
                    }
                    let exported_tycon = if opaque && !copy {
                        let fresh = sealed_tycon(sealed, tycon.stamp);
                        self.datatypes.insert(
                            fresh.stamp,
                            DataInfo {
                                constructors: exported.clone(),
                                never_equal,
                                equal_params,
                            },
                        );
                        fresh
                    } else {
                        actual
                    };
                    out.types.push((
                        name.clone(),
                        TypeEntry::Data {
                            tycon: exported_tycon,
                            arity: *arity,
                        },
                    ));
                    for (constructor, scheme) in exported {
                        out.values.push(Entry {
                            name: constructor,
                            scheme,
                            constructor: true,
                        });
                    }
                }
                SigItem::Exception { name, argument } => {
                    let Some(actual) = latest_value(env, name).filter(|entry| entry.constructor)
                    else {
                        return Err((
                            TypeError::MissingSpecification(format!("exception {name}")),
                            span,
                        ));
                    };
                    let wanted =
                        exception_scheme(&argument.as_ref().map(|ty| self.realize(ty, real)));
                    self.check_generality(
                        &actual.scheme,
                        &wanted,
                        &format!("exception {name}"),
                        span,
                    )?;
                    out.values.push(Entry {
                        name: name.clone(),
                        scheme: exception_scheme(
                            &argument.as_ref().map(|ty| self.realize(ty, sealed)),
                        ),
                        constructor: true,
                    });
                }
                SigItem::Struct { name, sig } => {
                    let inner = latest_struct(env, name).expect("checked when realizing");
                    let restricted =
                        self.conform(&inner, &sig.items, real, sealed, opaque, span)?;
                    out.structs.push((name.clone(), restricted));
                }
            }
        }
        Ok(out)
    }

    /// The actual scheme must be at least as general as the specified one:
    /// the specified type's variables are held fixed while the actual type's
    /// are instantiated freely.
    fn check_generality(
        &mut self,
        actual: &Scheme,
        wanted: &Scheme,
        what: &str,
        span: SourceSpan,
    ) -> Res<()> {
        let mut map = HashMap::new();
        for &id in &wanted.vars {
            let equality = self.vars[id].equality;
            map.insert(id, self.skolem(equality));
        }
        let fixed = self.substitute(&wanted.ty, &map);
        let instance = self.instantiate(actual);
        if self.unify(&instance, &fixed).is_err() {
            let reason = format!(
                "the structure has type {} but the signature specifies {}",
                self.show_scheme(actual),
                self.show_scheme(wanted)
            );
            return Err(mismatch(what.to_string(), &reason, span));
        }
        Ok(())
    }
}

fn mismatch(name: String, reason: &str, span: SourceSpan) -> (TypeError, SourceSpan) {
    (
        TypeError::SpecificationMismatch {
            name,
            reason: reason.to_string(),
        },
        span,
    )
}

fn exception_scheme(argument: &Option<Type>) -> Scheme {
    Scheme {
        vars: Vec::new(),
        ty: match argument {
            Some(argument) => arrow(argument.clone(), con("exn")),
            None => con("exn"),
        },
    }
}

fn latest_type(env: &StructEnv, name: &str) -> Option<TypeEntry> {
    env.types
        .iter()
        .rev()
        .find(|(known, _)| known == name)
        .map(|(_, entry)| entry.clone())
}

fn latest_value(env: &StructEnv, name: &str) -> Option<Entry> {
    env.values
        .iter()
        .rev()
        .find(|entry| entry.name == name)
        .cloned()
}

fn latest_struct(env: &StructEnv, name: &str) -> Option<StructEnv> {
    env.structs
        .iter()
        .rev()
        .find(|(known, _)| known == name)
        .map(|(_, inner)| inner.clone())
}

/// The fresh type constructor sealing made for the flexible one with `stamp`.
fn sealed_tycon(sealed: &Realization, stamp: usize) -> TyCon {
    match &sealed[&stamp].1 {
        Type::Con(tycon, _) => tycon.clone(),
        _ => unreachable!("sealing maps to a type constructor"),
    }
}

/// The specifications of the structure at `path` in a signature.
fn struct_items<'a>(items: &'a [SigItem], path: &str) -> Option<&'a [SigItem]> {
    let mut scope = items;
    for part in path.split('.') {
        scope = scope.iter().rev().find_map(|item| match item {
            SigItem::Struct { name, sig } if name == part => Some(sig.items.as_slice()),
            _ => None,
        })?;
    }
    Some(scope)
}

/// The paths, relative to `items`, of every type they specify.
fn type_paths(items: &[SigItem], prefix: &str, out: &mut Vec<String>) {
    for item in items {
        match item {
            SigItem::Type { name, .. } | SigItem::Datatype { name, .. } => {
                out.push(format!("{prefix}{name}"));
            }
            SigItem::Struct { name, sig } => {
                type_paths(&sig.items, &format!("{prefix}{name}."), out)
            }
            SigItem::Val { .. } | SigItem::Exception { .. } => {}
        }
    }
}
