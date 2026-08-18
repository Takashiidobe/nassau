use super::modules::{Sig, SigItem, StructEnv};
use super::{Infer, Scheme, Type, TypeEntry};
use crate::parser::{Decl, DeclKind, FixityKind, FunctorParameter, SigExpKind};

fn parameters(count: usize) -> String {
    let names: Vec<_> = (0..count)
        .map(|index| format!("'{}", super::var_name(index)))
        .collect();
    match names.as_slice() {
        [] => String::new(),
        [name] => format!("{name} "),
        _ => format!("({}) ", names.join(", ")),
    }
}

impl Infer {
    fn constructor_echo(&self, name: &str, scheme: &Scheme) -> String {
        match self.prune(&scheme.ty) {
            Type::Arrow(argument, _) => format!("{name} of {}", self.resolve(&argument)),
            _ => name.to_string(),
        }
    }

    fn datatype_echo(&self, name: &str, stamp: usize, arity: usize) -> String {
        let mut constructors: Vec<_> = self
            .datatypes
            .get(&stamp)
            .into_iter()
            .flat_map(|info| &info.constructors)
            .map(|(name, scheme)| self.constructor_echo(name, scheme))
            .collect();
        constructors.sort();
        format!(
            "datatype {}{name} = {}",
            parameters(arity),
            constructors.join(" | ")
        )
    }

    fn type_echo(&self, name: &str, entry: &TypeEntry, definition: bool) -> String {
        match entry {
            TypeEntry::Alias(alias) if definition => format!(
                "type {}{name} = {}",
                parameters(alias.params.len()),
                self.resolve(&alias.body)
            ),
            TypeEntry::Alias(alias) => {
                let mut infer = self.clone();
                let equality = infer.require_equality(&alias.body).is_ok();
                format!(
                    "{} {}{name}",
                    if equality { "eqtype" } else { "type" },
                    parameters(alias.params.len())
                )
            }
            TypeEntry::Data { tycon, arity } => {
                if let Some(info) = self.datatypes.get(&tycon.stamp)
                    && !info.constructors.is_empty()
                    && !self.abstract_datatypes.contains(&tycon.stamp)
                {
                    self.datatype_echo(name, tycon.stamp, *arity)
                } else {
                    format!("type {}{name}", parameters(*arity))
                }
            }
        }
    }

    pub(super) fn structure_items(&self, env: &StructEnv) -> Vec<(String, String)> {
        let mut items = Vec::new();
        for (name, entry) in &env.types {
            items.push((name.clone(), self.type_echo(name, entry, false)));
        }
        for entry in &env.values {
            if entry.constructor {
                let ty = self.prune(&entry.scheme.ty);
                let result = match &ty {
                    Type::Arrow(_, result) => self.prune(result),
                    ty => ty.clone(),
                };
                if matches!(result, Type::Con(ref con, _) if con.name == "exn" && con.stamp == 0) {
                    items.push((
                        entry.name.clone(),
                        format!(
                            "exception {}",
                            self.constructor_echo(&entry.name, &entry.scheme)
                        ),
                    ));
                }
            } else {
                items.push((
                    entry.name.clone(),
                    format!("val {}: {}", entry.name, self.resolve(&entry.scheme.ty)),
                ));
            }
        }
        for (name, nested) in &env.structs {
            items.push((
                name.clone(),
                format!("structure {name}: {}", self.structure_echo(nested)),
            ));
        }
        items.sort_by(|a, b| a.0.cmp(&b.0));
        items.dedup_by(|a, b| a.0 == b.0);
        items
    }

    fn structure_echo(&self, env: &StructEnv) -> String {
        format!(
            "sig {} end",
            self.structure_items(env)
                .into_iter()
                .map(|(_, echo)| echo)
                .collect::<Vec<_>>()
                .join(" ")
        )
    }

    fn signature_echo(&self, sig: &Sig) -> String {
        let mut items = Vec::new();
        for item in &sig.items {
            let (name, echo) = match item {
                SigItem::Val { name, scheme } => {
                    (name, format!("val {name}: {}", self.resolve(&scheme.ty)))
                }
                SigItem::Type {
                    name,
                    params,
                    equality,
                    definition,
                    ..
                } => {
                    let suffix = definition
                        .as_ref()
                        .map(|ty| format!(" = {}", self.resolve(ty)))
                        .unwrap_or_default();
                    (
                        name,
                        format!(
                            "{} {}{name}{suffix}",
                            if *equality { "eqtype" } else { "type" },
                            parameters(params.len())
                        ),
                    )
                }
                SigItem::Datatype {
                    name, tycon, arity, ..
                } => (name, self.datatype_echo(name, tycon.stamp, *arity)),
                SigItem::Exception { name, argument } => (
                    name,
                    format!(
                        "exception {name}{}",
                        argument
                            .as_ref()
                            .map(|ty| format!(" of {}", self.resolve(ty)))
                            .unwrap_or_default()
                    ),
                ),
                SigItem::Struct { name, sig } => (
                    name,
                    format!("structure {name}: {}", self.signature_echo(sig)),
                ),
            };
            items.push((name, echo));
        }
        items.sort_by(|a, b| a.0.cmp(b.0));
        format!(
            "sig {} end",
            items
                .into_iter()
                .map(|(_, echo)| echo)
                .collect::<Vec<_>>()
                .join(" ")
        )
    }

    pub(super) fn declaration_echo(&self, decl: &Decl) -> Vec<String> {
        match &decl.value {
            DeclKind::Type(bindings) => bindings
                .iter()
                .filter_map(|binding| {
                    self.lookup_type(&binding.name)
                        .map(|entry| self.type_echo(&binding.name, &entry, true))
                })
                .collect(),
            DeclKind::Datatype { bindings, withtype } => {
                let mut echoes: Vec<_> = bindings
                    .iter()
                    .filter_map(|binding| {
                        self.lookup_type(&binding.name)
                            .map(|entry| self.type_echo(&binding.name, &entry, true))
                    })
                    .collect();
                echoes.extend(withtype.iter().filter_map(|binding| {
                    self.lookup_type(&binding.name)
                        .map(|entry| self.type_echo(&binding.name, &entry, true))
                }));
                echoes
            }
            DeclKind::DatatypeCopy { name, .. } => self
                .lookup_type(name)
                .map(|entry| vec![self.type_echo(name, &entry, true)])
                .unwrap_or_default(),
            DeclKind::Exception(bindings) => bindings
                .iter()
                .filter_map(|binding| {
                    self.lookup(&binding.name).map(|entry| {
                        format!(
                            "exception {}",
                            self.constructor_echo(&binding.name, &entry.scheme)
                        )
                    })
                })
                .collect(),
            DeclKind::Structure(bindings) => bindings
                .iter()
                .filter_map(|binding| {
                    self.lookup_struct(&binding.name).map(|env| {
                        format!("structure {}: {}", binding.name, self.structure_echo(env))
                    })
                })
                .collect(),
            DeclKind::Signature(bindings) => bindings
                .iter()
                .filter_map(|binding| {
                    self.signatures
                        .iter()
                        .rev()
                        .find(|(name, _)| *name == binding.name)
                        .map(|(_, sig)| {
                            format!("signature {} = {}", binding.name, self.signature_echo(sig))
                        })
                })
                .collect(),
            DeclKind::Functor(bindings) => bindings
                .iter()
                .filter_map(|binding| {
                    self.functors
                        .iter()
                        .rev()
                        .find(|(name, _)| *name == binding.name)
                        .map(|(_, functor)| {
                            let signature = match &binding.parameter {
                                FunctorParameter::Named(_, sig) | FunctorParameter::Specs(sig) => {
                                    sig
                                }
                            };
                            let signature = match &signature.value {
                                SigExpKind::Name(name) => name.clone(),
                                _ => self.signature_echo(&functor.sig),
                            };
                            let parameter = functor
                                .parameter
                                .as_ref()
                                .map(|name| format!("{name}: {signature}"))
                                .unwrap_or(signature);
                            format!(
                                "functor {} ({parameter}): {}",
                                binding.name,
                                self.structure_echo(&functor.result)
                            )
                        })
                })
                .collect(),
            DeclKind::Open(paths) => {
                let mut echoes = Vec::new();
                for path in paths {
                    if let Some(env) = self.lookup_struct(path) {
                        echoes.extend(
                            env.types
                                .iter()
                                .map(|(name, entry)| self.type_echo(name, entry, false)),
                        );
                        echoes.extend(env.structs.iter().map(|(name, env)| {
                            format!("structure {name}: {}", self.structure_echo(env))
                        }));
                    }
                }
                echoes
            }
            DeclKind::Fixity {
                kind,
                precedence,
                names,
            } => {
                let kind = match kind {
                    FixityKind::Infix => "infix",
                    FixityKind::Infixr => "infixr",
                    FixityKind::Nonfix => "nonfix",
                };
                let precedence = if kind == "nonfix" {
                    String::new()
                } else {
                    format!(" {precedence}")
                };
                vec![format!("{kind}{precedence} {}", names.join(" "))]
            }
            DeclKind::Abstype { bindings, body, .. } => {
                let mut echoes: Vec<_> = bindings
                    .iter()
                    .map(|binding| {
                        format!(
                            "type {}{}",
                            parameters(binding.parameters.len()),
                            binding.name
                        )
                    })
                    .collect();
                echoes.extend(body.iter().flat_map(|decl| self.declaration_echo(decl)));
                echoes
            }
            DeclKind::Local(_, public) => public
                .iter()
                .flat_map(|decl| self.declaration_echo(decl))
                .collect(),
            DeclKind::Val { .. } | DeclKind::Fun(_) => Vec::new(),
        }
    }
}
