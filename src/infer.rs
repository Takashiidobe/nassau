//! Hindley–Milner type inference with SML's value restriction, equality types
//! and overloaded arithmetic.
//!
//! Types are built from unification variables; a variable's `level` says how
//! deeply nested the binding it was created under is, so generalisation only
//! quantifies variables that are deeper than the current declaration. Overloaded
//! operators (`+`, `<`, ...) create a variable restricted to a set of
//! candidate types; it is never generalised and defaults to the first
//! candidate (`int`) when the enclosing top-level declaration ends.

use std::collections::{HashMap, HashSet};
use std::fmt;

use miette::SourceSpan;

mod modules;
mod printing;

use crate::error::ThisError;
use crate::parser::{
    DataBinding, Decl, DeclKind, ExceptionKind, Expr, ExprKind, Pat, PatKind, Program, Rule,
    StmtKind, Ty as SyntaxTy, TyKind,
};
use crate::span::Loc;
use crate::value;
use modules::{Functor, Sig, StructEnv};

#[derive(Debug, ThisError)]
pub enum TypeError {
    #[error("unbound variable '{0}'")]
    UnboundVariable(String),
    #[error("unbound constructor '{0}'")]
    UnboundConstructor(String),
    #[error("unbound type '{0}'")]
    UnboundType(String),
    #[error("expected {expected}, found {found}")]
    Mismatch { expected: String, found: String },
    #[error("cannot build the infinite type {var} = {ty}")]
    Circular { var: String, ty: String },
    #[error("type {0} does not admit equality")]
    NotEquality(String),
    #[error("operator is not defined for type {0}")]
    NotOverloaded(String),
    #[error("cannot apply a value of type {0}, which is not a function")]
    NotAFunction(String),
    #[error("constructor '{0}' requires an argument")]
    ConstructorNeedsArgument(String),
    #[error("constructor '{0}' does not take an argument")]
    ConstructorTakesNoArgument(String),
    #[error("type '{name}' expects {expected} type argument(s), found {found}")]
    TypeArity {
        name: String,
        expected: usize,
        found: usize,
    },
    #[error("'{0}' is not an exception")]
    NotAnException(String),
    #[error("unbound type variable {0} in type declaration")]
    UnboundTypeVariable(String),
    #[error("unbound structure '{0}'")]
    UnboundStructure(String),
    #[error("unbound signature '{0}'")]
    UnboundSignature(String),
    #[error("the structure does not provide {0}, which the signature specifies")]
    MissingSpecification(String),
    #[error("{name} does not match its specification: {reason}")]
    SpecificationMismatch { name: String, reason: String },
    #[error("unbound functor '{0}'")]
    UnboundFunctor(String),
    #[error("cannot refine {0}: {1}")]
    BadRefinement(String, String),
    #[error("int constant too large")]
    IntegerOutOfRange,
    #[error("unresolved flex record (can't tell what fields there are besides {0})")]
    UnresolvedFlexRecord(String),
    #[error("{0} are not supported yet")]
    Unsupported(&'static str),
}

pub type Failure = (TypeError, SourceSpan);
type Res<T> = Result<T, Failure>;

/// A name a top-level declaration bound, with its printed type.
pub struct Binding {
    pub name: String,
    pub ty: String,
    /// The same type, for code that needs its structure.
    pub resolved: Ty,
}

/// A resolved type, as the backend sees it: overloads are defaulted and weak
/// type variables of top-level bindings are frozen, but polymorphic
/// variables remain.
#[derive(Clone, Debug, PartialEq)]
pub enum Ty {
    Var {
        id: usize,
        equality: bool,
    },
    /// A type constructor. Built-in ones (`int`, `list`, ...) have stamp 0.
    Con {
        name: String,
        stamp: usize,
        args: Vec<Ty>,
    },
    Arrow(Box<Ty>, Box<Ty>),
    /// Labels are sorted as SML sorts them; tuples use `1`, `2`, ...
    Record(Vec<(String, Ty)>),
}

impl Ty {
    /// Whether this is the built-in type constructor `name`.
    pub fn is(&self, name: &str) -> bool {
        matches!(self, Ty::Con { name: found, stamp: 0, .. } if found == name)
    }
}

impl fmt::Display for Ty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.show(&mut Vec::new(), 0))
    }
}

impl Ty {
    /// Shows the type, naming its variables consistently with earlier calls
    /// that shared `names`.
    pub fn show_named(&self, names: &mut Vec<usize>) -> String {
        self.show(names, 0)
    }

    /// Shows the type as SML does, naming its variables `'a`, `'b`, ... in
    /// order of appearance.
    fn show(&self, names: &mut Vec<usize>, precedence: u8) -> String {
        match self {
            Ty::Var { id, equality } => {
                let index = names
                    .iter()
                    .position(|known| known == id)
                    .unwrap_or_else(|| {
                        names.push(*id);
                        names.len() - 1
                    });
                let quote = if *equality { "''" } else { "'" };
                format!("{quote}{}", var_name(index))
            }
            Ty::Con { name, args, .. } => match args.as_slice() {
                [] => name.clone(),
                [single] => format!("{} {name}", single.show(names, 2)),
                many => {
                    let args: Vec<String> = many.iter().map(|arg| arg.show(names, 0)).collect();
                    format!("({}) {name}", args.join(","))
                }
            },
            Ty::Arrow(from, to) => {
                let text = format!("{} -> {}", from.show(names, 1), to.show(names, 0));
                if precedence >= 1 {
                    format!("({text})")
                } else {
                    text
                }
            }
            Ty::Record(fields) => {
                let is_tuple = fields.len() > 1
                    && fields
                        .iter()
                        .enumerate()
                        .all(|(index, (label, _))| *label == (index + 1).to_string());
                if is_tuple {
                    let items: Vec<String> =
                        fields.iter().map(|(_, ty)| ty.show(names, 2)).collect();
                    let text = items.join(" * ");
                    if precedence >= 2 {
                        format!("({text})")
                    } else {
                        text
                    }
                } else if fields.is_empty() {
                    "unit".to_string()
                } else {
                    let items: Vec<String> = fields
                        .iter()
                        .map(|(label, ty)| format!("{label}:{}", ty.show(names, 0)))
                        .collect();
                    format!("{{{}}}", items.join(", "))
                }
            }
        }
    }
}

/// `a`, `b`, ..., `z`, `a1`, ...
fn var_name(index: usize) -> String {
    let letter = (b'a' + (index % 26) as u8) as char;
    match index / 26 {
        0 => letter.to_string(),
        round => format!("{letter}{round}"),
    }
}

/// The inferred type of one expression or pattern.
#[derive(Clone)]
pub struct Node<T> {
    /// The node's address: the syntax tree is not moved between type
    /// checking and code generation.
    address: usize,
    pub pattern: bool,
    pub start: Loc,
    pub end: Loc,
    pub ty: T,
}

/// The inferred type of every expression and pattern of a program.
#[derive(Default)]
pub struct TypeTable {
    nodes: Vec<Node<Ty>>,
    index: HashMap<(usize, bool), usize>,
    /// Each datatype's constructors in declaration order, by stamp, with
    /// whether each takes an argument.
    datatypes: HashMap<usize, Vec<(String, bool)>>,
    structures: HashMap<usize, StructureInfo>,
    declaration_constructors: HashMap<usize, Vec<(String, usize)>>,
    exception_arguments: HashMap<usize, Vec<Option<Ty>>>,
}

#[derive(Clone, Default)]
pub struct StructureInfo {
    pub values: Vec<Export>,
    pub structures: Vec<(String, StructureInfo)>,
    pub application: Option<std::rc::Rc<crate::parser::StrExp>>,
    pub parameter: Option<Box<StructureInfo>>,
}

#[derive(Clone)]
pub struct Export {
    pub name: String,
    pub constructor: bool,
    pub carries: bool,
}

impl TypeTable {
    pub fn structure(&self, exp: &crate::parser::StrExp) -> &StructureInfo {
        &self.structures[&(exp as *const _ as usize)]
    }

    pub fn exception_argument(&self, decl: &Decl, index: usize) -> Option<&Ty> {
        self.exception_arguments
            .get(&(decl as *const _ as usize))?
            .get(index)?
            .as_ref()
    }

    pub fn declaration_constructors(&self, decl: &Decl) -> &[(String, usize)] {
        self.declaration_constructors
            .get(&(decl as *const _ as usize))
            .map_or(&[], Vec::as_slice)
    }
    /// The constructors of the datatype with `stamp`.
    pub fn constructors(&self, stamp: usize) -> Option<&[(String, bool)]> {
        self.datatypes.get(&stamp).map(Vec::as_slice)
    }

    pub fn expr(&self, expr: &Expr) -> Option<&Ty> {
        self.get(expr as *const Expr as usize, false)
    }

    pub fn pat(&self, pat: &Pat) -> Option<&Ty> {
        self.get(pat as *const Pat as usize, true)
    }

    fn get(&self, address: usize, pattern: bool) -> Option<&Ty> {
        let index = self.index.get(&(address, pattern))?;
        Some(&self.nodes[*index].ty)
    }

    /// Every typed node once, in source order, outer nodes first.
    pub fn nodes(&self) -> Vec<&Node<Ty>> {
        let mut indices: Vec<usize> = self.index.values().copied().collect();
        // Nodes are recorded after their children, so among nodes with the
        // same span the later one encloses the earlier.
        indices.sort_by_key(|index| {
            let node = &self.nodes[*index];
            (
                node.start.offset,
                std::cmp::Reverse(node.end.offset),
                std::cmp::Reverse(*index),
            )
        });
        indices
            .into_iter()
            .map(|index| &self.nodes[index])
            .collect()
    }
}

type Echoes = Vec<(usize, String)>;

/// What checking a program produced.
pub struct Checked {
    pub bindings: Vec<Binding>,
    pub echoes: Echoes,
    pub types: TypeTable,
}

/// A type constructor. Built-in ones have stamp 0; every `datatype` gets its
/// own stamp, so two datatypes with the same name are different types.
#[derive(Clone, Debug, PartialEq)]
struct TyCon {
    name: String,
    stamp: usize,
}

impl TyCon {
    fn builtin(name: &str) -> Self {
        Self {
            name: name.to_string(),
            stamp: 0,
        }
    }

    fn is(&self, name: &str) -> bool {
        self.stamp == 0 && self.name == name
    }
}

#[derive(Clone, Debug)]
enum Type {
    Var(usize),
    Con(TyCon, Vec<Type>),
    Arrow(Box<Type>, Box<Type>),
    /// Labels are kept sorted; tuples use the labels `1`, `2`, ...
    Record(Vec<(String, Type)>),
}

#[derive(Clone)]
struct VarInfo {
    link: Option<Type>,
    level: usize,
    equality: bool,
    overload: Option<Vec<&'static str>>,
    /// A flexible record (`{a, ...}` or the argument of `#a`): the variable
    /// stands for a record with at least these fields, sorted by label.
    fields: Option<Vec<(String, Type)>>,
}

#[derive(Clone, Debug)]
struct Scheme {
    vars: Vec<usize>,
    ty: Type,
}

#[derive(Clone)]
struct Entry {
    name: String,
    scheme: Scheme,
    constructor: bool,
}

#[derive(Clone)]
struct Alias {
    params: Vec<usize>,
    body: Type,
}

#[derive(Clone)]
enum TypeEntry {
    Alias(Alias),
    Data { tycon: TyCon, arity: usize },
}

/// What is known about a datatype once it is declared.
#[derive(Clone)]
struct DataInfo {
    constructors: Vec<(String, Scheme)>,
    /// Whether some constructor holds a function, `real` or `exn`.
    never_equal: bool,
    /// Which parameters must admit equality for the datatype to.
    equal_params: Vec<bool>,
}

enum UnifyError {
    Mismatch,
    Circular(usize, Type),
    NotEquality(Type),
    NotOverloaded(Type),
}

const NUMERIC: &[&str] = &["int", "word", "real"];
const ORDERED: &[&str] = &["int", "word", "real", "string", "char"];
const INTEGRAL: &[&str] = &["int", "word"];
const TYPE_CONSTRUCTORS: &[(&str, usize)] = &[
    ("int", 0),
    ("real", 0),
    ("string", 0),
    ("bool", 0),
    ("unit", 0),
    ("char", 0),
    ("word", 0),
    ("exn", 0),
    ("order", 0),
    ("list", 1),
    ("option", 1),
    ("ref", 1),
];

fn con(name: &str) -> Type {
    Type::Con(TyCon::builtin(name), Vec::new())
}

fn arrow(from: Type, to: Type) -> Type {
    Type::Arrow(Box::new(from), Box::new(to))
}

fn list(element: Type) -> Type {
    Type::Con(TyCon::builtin("list"), vec![element])
}

fn tuple(items: Vec<Type>) -> Type {
    Type::Record(
        items
            .into_iter()
            .enumerate()
            .map(|(index, ty)| ((index + 1).to_string(), ty))
            .collect(),
    )
}

fn label_key(label: &str) -> (u8, usize, String) {
    match label.parse::<usize>() {
        Ok(number) => (0, number, String::new()),
        Err(_) => (1, 0, label.to_string()),
    }
}

fn sort_labels<T>(fields: &mut [(String, T)]) {
    fields.sort_by_key(|(label, _)| label_key(label));
}

/// Lengths of the environment's scopes, to return to when a scope ends.
#[derive(Clone, Copy)]
struct Mark {
    values: usize,
    types: usize,
    structs: usize,
    signatures: usize,
    functors: usize,
}

#[derive(Clone)]
struct Infer {
    vars: Vec<VarInfo>,
    level: usize,
    values: Vec<Entry>,
    types: Vec<(String, TypeEntry)>,
    datatypes: HashMap<usize, DataInfo>,
    structs: Vec<(String, StructEnv)>,
    signatures: Vec<(String, Sig)>,
    functors: Vec<(String, std::rc::Rc<Functor>)>,
    /// The qualified name (`S.t`) of each type constructor a structure declares.
    paths: HashMap<usize, String>,
    next_stamp: usize,
    overloaded: Vec<usize>,
    /// Flexible records not yet known to be resolved.
    flexible: Vec<usize>,
    /// Type variables written in annotations, shared within a top-level declaration.
    tyvars: HashMap<String, Type>,
    /// The type inferred for each expression and pattern, until resolved.
    node_types: Vec<Node<Type>>,
    structure_info: HashMap<usize, StructureInfo>,
    declaration_constructors: HashMap<usize, Vec<(String, usize)>>,
    exception_arguments: HashMap<usize, Vec<Option<Ty>>>,
    abstract_datatypes: HashSet<usize>,
}

impl Infer {
    fn new() -> Self {
        let mut infer = Self {
            vars: Vec::new(),
            level: 0,
            values: Vec::new(),
            types: Vec::new(),
            datatypes: HashMap::new(),
            structs: Vec::new(),
            signatures: Vec::new(),
            functors: Vec::new(),
            paths: HashMap::new(),
            next_stamp: 0,
            overloaded: Vec::new(),
            flexible: Vec::new(),
            tyvars: HashMap::new(),
            node_types: Vec::new(),
            structure_info: HashMap::new(),
            declaration_constructors: HashMap::new(),
            exception_arguments: HashMap::new(),
            abstract_datatypes: HashSet::new(),
        };
        infer.install_builtins();
        infer
    }

    fn install_builtins(&mut self) {
        let int = || con("int");
        let string = || con("string");
        let boolean = || con("bool");
        let unit = || con("unit");
        let real = || con("real");
        let option = |ty: Type| Type::Con(TyCon::builtin("option"), vec![ty]);
        let reference = |ty: Type| Type::Con(TyCon::builtin("ref"), vec![ty]);
        self.builtin("nil", true, 1, |v| list(v[0].clone()));
        self.builtin("::", true, 1, |v| {
            arrow(
                tuple(vec![v[0].clone(), list(v[0].clone())]),
                list(v[0].clone()),
            )
        });
        self.builtin("SOME", true, 1, |v| {
            arrow(v[0].clone(), option(v[0].clone()))
        });
        self.builtin("NONE", true, 1, |v| option(v[0].clone()));
        for name in ["LESS", "EQUAL", "GREATER"] {
            self.builtin(name, true, 0, |_| con("order"));
        }
        for name in ["Div", "Overflow", "Match", "Bind", "Empty", "Subscript"] {
            self.builtin(name, true, 0, |_| con("exn"));
        }
        self.builtin("Fail", true, 0, |_| arrow(string(), con("exn")));
        self.builtin("not", false, 0, |_| arrow(boolean(), boolean()));
        self.builtin("print", false, 0, |_| arrow(string(), unit()));
        self.builtin("size", false, 0, |_| arrow(string(), int()));
        self.builtin("^", false, 0, |_| {
            arrow(tuple(vec![string(), string()]), string())
        });
        self.builtin("@", false, 1, |v| {
            let l = list(v[0].clone());
            arrow(tuple(vec![l.clone(), l.clone()]), l)
        });
        self.builtin("hd", false, 1, |v| arrow(list(v[0].clone()), v[0].clone()));
        self.builtin("tl", false, 1, |v| {
            arrow(list(v[0].clone()), list(v[0].clone()))
        });
        self.builtin("null", false, 1, |v| arrow(list(v[0].clone()), boolean()));
        self.builtin("length", false, 1, |v| arrow(list(v[0].clone()), int()));
        self.builtin("rev", false, 1, |v| {
            arrow(list(v[0].clone()), list(v[0].clone()))
        });
        self.builtin("map", false, 2, |v| {
            arrow(
                arrow(v[0].clone(), v[1].clone()),
                arrow(list(v[0].clone()), list(v[1].clone())),
            )
        });
        for name in ["foldl", "foldr"] {
            self.builtin(name, false, 2, |v| {
                arrow(
                    arrow(tuple(vec![v[0].clone(), v[1].clone()]), v[1].clone()),
                    arrow(v[1].clone(), arrow(list(v[0].clone()), v[1].clone())),
                )
            });
        }
        self.builtin("o", false, 3, |v| {
            arrow(
                tuple(vec![
                    arrow(v[1].clone(), v[2].clone()),
                    arrow(v[0].clone(), v[1].clone()),
                ]),
                arrow(v[0].clone(), v[2].clone()),
            )
        });
        self.builtin("before", false, 1, |v| {
            arrow(tuple(vec![v[0].clone(), unit()]), v[0].clone())
        });
        self.builtin("ignore", false, 1, |v| arrow(v[0].clone(), unit()));
        self.builtin("ref", true, 1, |v| {
            arrow(v[0].clone(), reference(v[0].clone()))
        });
        self.builtin("!", false, 1, |v| {
            arrow(reference(v[0].clone()), v[0].clone())
        });
        self.builtin(":=", false, 1, |v| {
            arrow(tuple(vec![reference(v[0].clone()), v[0].clone()]), unit())
        });
        // `op =` and `op <>` as values; applied infix, they are operators.
        for name in ["=", "<>"] {
            self.builtin(name, false, 1, |v| {
                arrow(tuple(vec![v[0].clone(), v[0].clone()]), boolean())
            });
            if let Some(Entry { scheme, .. }) = self.values.last()
                && let [var] = scheme.vars[..]
            {
                self.vars[var].equality = true;
            }
        }
        self.builtin("real", false, 0, |_| arrow(int(), real()));
        self.builtin("floor", false, 0, |_| arrow(real(), int()));
        self.builtin("ord", false, 0, |_| arrow(con("char"), int()));
        self.builtin("chr", false, 0, |_| arrow(int(), con("char")));
        self.builtin("str", false, 0, |_| arrow(con("char"), string()));
        self.builtin("explode", false, 0, |_| arrow(string(), list(con("char"))));
        self.builtin("implode", false, 0, |_| arrow(list(con("char")), string()));
        self.builtin("concat", false, 0, |_| arrow(list(string()), string()));
        self.builtin("Int.toString", false, 0, |_| arrow(int(), string()));
        self.builtin("Word8.fromInt", false, 0, |_| arrow(int(), con("word8")));
        self.builtin("Posix.Process.exit", false, 1, |v| {
            arrow(con("word8"), v[0].clone())
        });
    }

    fn builtin(
        &mut self,
        name: &str,
        constructor: bool,
        arity: usize,
        build: impl FnOnce(&[Type]) -> Type,
    ) {
        self.level = 1;
        let variables: Vec<Type> = (0..arity).map(|_| self.fresh()).collect();
        self.level = 0;
        let vars = variables
            .iter()
            .map(|ty| match ty {
                Type::Var(id) => *id,
                _ => unreachable!(),
            })
            .collect();
        let ty = build(&variables);
        self.values.push(Entry {
            name: name.to_string(),
            scheme: Scheme { vars, ty },
            constructor,
        });
    }

    fn fresh(&mut self) -> Type {
        self.vars.push(VarInfo {
            link: None,
            level: self.level,
            equality: false,
            overload: None,
            fields: None,
        });
        Type::Var(self.vars.len() - 1)
    }

    /// A record with at least `fields`, whose other fields are not known yet.
    fn flexible_record(&mut self, mut fields: Vec<(String, Type)>) -> Type {
        sort_labels(&mut fields);
        let ty = self.fresh();
        if let Type::Var(id) = &ty {
            self.vars[*id].fields = Some(fields);
            self.flexible.push(*id);
        }
        ty
    }

    /// SML/NJ requires every flexible record to be resolved to a known
    /// record type by the end of the declaration that would generalise it.
    /// At the end of a top-level declaration (`all`), every flexible record
    /// must be resolved.
    fn check_flexible(&mut self, span: SourceSpan, all: bool) -> Res<()> {
        let mut pending = Vec::new();
        for id in std::mem::take(&mut self.flexible) {
            let Type::Var(root) = self.prune(&Type::Var(id)) else {
                continue;
            };
            let Some(fields) = &self.vars[root].fields else {
                continue;
            };
            if all || self.vars[root].level > self.level {
                let labels: Vec<String> = fields
                    .iter()
                    .map(|(label, _)| format!("#{label}"))
                    .collect();
                return Err((TypeError::UnresolvedFlexRecord(labels.join(" ")), span));
            }
            pending.push(id);
        }
        self.flexible = pending;
        Ok(())
    }

    fn fresh_with(&mut self, equality: bool) -> Type {
        let ty = self.fresh();
        if let Type::Var(id) = &ty {
            self.vars[*id].equality = equality;
        }
        ty
    }

    fn overloaded_var(&mut self, candidates: &'static [&'static str]) -> Type {
        let ty = self.fresh_with(false);
        if let Type::Var(id) = &ty {
            self.vars[*id].overload = Some(candidates.to_vec());
            self.overloaded.push(*id);
        }
        ty
    }

    fn prune(&self, ty: &Type) -> Type {
        let mut ty = ty.clone();
        while let Type::Var(id) = &ty {
            match &self.vars[*id].link {
                Some(next) => ty = next.clone(),
                None => break,
            }
        }
        ty
    }

    // --- unification -----------------------------------------------------

    fn unify(&mut self, lhs: &Type, rhs: &Type) -> Result<(), UnifyError> {
        let (lhs, rhs) = (self.prune(lhs), self.prune(rhs));
        match (&lhs, &rhs) {
            (Type::Var(a), Type::Var(b)) if a == b => Ok(()),
            (Type::Var(a), Type::Var(b)) => self.merge(*a, *b),
            (Type::Var(id), other) | (other, Type::Var(id)) => self.bind(*id, other),
            (Type::Con(a, xs), Type::Con(b, ys)) if a == b && xs.len() == ys.len() => {
                for (x, y) in xs.iter().zip(ys) {
                    self.unify(x, y)?;
                }
                Ok(())
            }
            (Type::Arrow(a, b), Type::Arrow(c, d)) => {
                self.unify(a, c)?;
                self.unify(b, d)
            }
            (Type::Record(xs), Type::Record(ys))
                if xs.len() == ys.len() && xs.iter().zip(ys).all(|(x, y)| x.0 == y.0) =>
            {
                for ((_, x), (_, y)) in xs.iter().zip(ys) {
                    self.unify(x, y)?;
                }
                Ok(())
            }
            _ => Err(UnifyError::Mismatch),
        }
    }

    /// Link `a` to `b`, keeping the constraints of both.
    fn merge(&mut self, a: usize, b: usize) -> Result<(), UnifyError> {
        let equality = self.vars[a].equality || self.vars[b].equality;
        let level = self.vars[a].level.min(self.vars[b].level);
        let overload = match (self.vars[a].overload.clone(), self.vars[b].overload.clone()) {
            (Some(x), Some(y)) => Some(x.into_iter().filter(|c| y.contains(c)).collect()),
            (x, y) => x.or(y),
        };
        let overload = overload.map(|candidates: Vec<&'static str>| {
            if equality {
                candidates.into_iter().filter(|c| *c != "real").collect()
            } else {
                candidates
            }
        });
        if overload.as_ref().is_some_and(Vec::is_empty) {
            return Err(UnifyError::NotOverloaded(Type::Var(b)));
        }
        let fields = match (self.vars[a].fields.clone(), self.vars[b].fields.clone()) {
            (Some(xs), Some(ys)) => {
                let mut fields = ys.clone();
                for (label, x) in xs {
                    match ys.iter().find(|(known, _)| *known == label) {
                        Some((_, y)) => self.unify(&x, y)?,
                        None => fields.push((label, x)),
                    }
                }
                sort_labels(&mut fields);
                Some(fields)
            }
            (x, y) => x.or(y),
        };
        if fields.is_some() && overload.is_some() {
            return Err(UnifyError::Mismatch);
        }
        self.vars[b].fields = fields;
        self.vars[b].equality = equality;
        self.vars[b].level = level;
        self.vars[b].overload = overload;
        self.vars[a].link = Some(Type::Var(b));
        Ok(())
    }

    fn bind(&mut self, id: usize, ty: &Type) -> Result<(), UnifyError> {
        self.check_occurs(id, ty).map_err(|error| match error {
            UnifyError::Circular(var, _) => UnifyError::Circular(var, ty.clone()),
            other => other,
        })?;
        if self.vars[id].equality {
            self.require_equality(ty)?;
        }
        if let Some(fields) = self.vars[id].fields.clone() {
            let Type::Record(found) = self.prune(ty) else {
                return Err(UnifyError::Mismatch);
            };
            for (label, field) in &fields {
                let Some((_, other)) = found.iter().find(|(known, _)| known == label) else {
                    return Err(UnifyError::Mismatch);
                };
                self.unify(field, other)?;
            }
        }
        if let Some(candidates) = &self.vars[id].overload
            && !matches!(ty, Type::Con(name, args) if args.is_empty() && name.stamp == 0 && candidates.contains(&name.name.as_str()))
        {
            return Err(UnifyError::NotOverloaded(ty.clone()));
        }
        self.vars[id].link = Some(ty.clone());
        Ok(())
    }

    fn check_occurs(&mut self, id: usize, ty: &Type) -> Result<(), UnifyError> {
        match self.prune(ty) {
            Type::Var(other) if other == id => Err(UnifyError::Circular(id, ty.clone())),
            Type::Var(other) => {
                self.vars[other].level = self.vars[other].level.min(self.vars[id].level);
                match self.vars[other].fields.clone() {
                    Some(fields) => fields
                        .iter()
                        .try_for_each(|(_, field)| self.check_occurs(id, field)),
                    None => Ok(()),
                }
            }
            Type::Con(_, args) => args.iter().try_for_each(|arg| self.check_occurs(id, arg)),
            Type::Arrow(from, to) => {
                self.check_occurs(id, &from)?;
                self.check_occurs(id, &to)
            }
            Type::Record(fields) => fields
                .iter()
                .try_for_each(|(_, field)| self.check_occurs(id, field)),
        }
    }

    fn require_equality(&mut self, ty: &Type) -> Result<(), UnifyError> {
        match self.prune(ty) {
            Type::Var(id) => {
                self.vars[id].equality = true;
                if let Some(candidates) = &mut self.vars[id].overload {
                    candidates.retain(|c| *c != "real");
                }
                Ok(())
            }
            Type::Con(name, _) if name.is("real") || name.is("exn") => {
                Err(UnifyError::NotEquality(ty.clone()))
            }
            Type::Con(name, _) if name.is("ref") => Ok(()),
            Type::Con(name, args) if name.stamp != 0 => {
                let Some(info) = self.datatypes.get(&name.stamp) else {
                    return Ok(());
                };
                if info.never_equal {
                    return Err(UnifyError::NotEquality(ty.clone()));
                }
                let needed = info.equal_params.clone();
                args.iter()
                    .zip(needed)
                    .filter(|(_, needed)| *needed)
                    .try_for_each(|(arg, _)| self.require_equality(arg))
            }
            Type::Con(_, args) => args.iter().try_for_each(|arg| self.require_equality(arg)),
            Type::Arrow(..) => Err(UnifyError::NotEquality(ty.clone())),
            Type::Record(fields) => fields
                .iter()
                .try_for_each(|(_, field)| self.require_equality(field)),
        }
    }

    fn unify_at(&mut self, expected: &Type, found: &Type, span: SourceSpan) -> Res<()> {
        match self.unify(expected, found) {
            Ok(()) => Ok(()),
            Err(error) => Err((self.explain(error, expected, found), span)),
        }
    }

    fn explain(&self, error: UnifyError, expected: &Type, found: &Type) -> TypeError {
        match error {
            UnifyError::Mismatch => {
                let mut names = Namer::new();
                self.hide_inaccessible(expected, &mut names);
                self.hide_inaccessible(found, &mut names);
                let expected = self.show(expected, &mut names, 0);
                let found = self.show(found, &mut names, 0);
                TypeError::Mismatch { expected, found }
            }
            UnifyError::Circular(id, ty) => {
                let mut names = Namer::new();
                let var = self.show(&Type::Var(id), &mut names, 0);
                let ty = self.show(&ty, &mut names, 0);
                TypeError::Circular { var, ty }
            }
            UnifyError::NotEquality(ty) => {
                let mut names = Namer::new();
                self.hide_inaccessible(&ty, &mut names);
                TypeError::NotEquality(self.show(&ty, &mut names, 0))
            }
            UnifyError::NotOverloaded(ty) => {
                let mut names = Namer::new();
                self.hide_inaccessible(&ty, &mut names);
                TypeError::NotOverloaded(self.show(&ty, &mut names, 0))
            }
        }
    }

    // --- generalisation --------------------------------------------------

    fn free_vars(&self, ty: &Type, out: &mut Vec<usize>) {
        match self.prune(ty) {
            Type::Var(id) => {
                if !out.contains(&id) {
                    out.push(id);
                    if let Some(fields) = &self.vars[id].fields {
                        for (_, field) in fields {
                            self.free_vars(field, out);
                        }
                    }
                }
            }
            Type::Con(_, args) => args.iter().for_each(|arg| self.free_vars(arg, out)),
            Type::Arrow(from, to) => {
                self.free_vars(&from, out);
                self.free_vars(&to, out);
            }
            Type::Record(fields) => fields
                .iter()
                .for_each(|(_, field)| self.free_vars(field, out)),
        }
    }

    fn generalize(&mut self, ty: &Type) -> Scheme {
        let mut free = Vec::new();
        self.free_vars(ty, &mut free);
        let mut vars = Vec::new();
        for id in free {
            if self.vars[id].level > self.level {
                if self.vars[id].overload.is_some() || self.vars[id].fields.is_some() {
                    self.vars[id].level = self.level;
                } else {
                    vars.push(id);
                }
            }
        }
        Scheme {
            vars,
            ty: ty.clone(),
        }
    }

    /// The binding is not generalised (value restriction): its variables now
    /// belong to the enclosing level.
    fn monomorphic(&mut self, ty: &Type) -> Scheme {
        let mut free = Vec::new();
        self.free_vars(ty, &mut free);
        for id in free {
            self.vars[id].level = self.vars[id].level.min(self.level);
        }
        Scheme {
            vars: Vec::new(),
            ty: ty.clone(),
        }
    }

    fn instantiate(&mut self, scheme: &Scheme) -> Type {
        if scheme.vars.is_empty() {
            return scheme.ty.clone();
        }
        let mut map = HashMap::new();
        for &id in &scheme.vars {
            let equality = self.vars[id].equality;
            map.insert(id, self.fresh_with(equality));
        }
        self.substitute(&scheme.ty, &map)
    }

    fn substitute(&self, ty: &Type, map: &HashMap<usize, Type>) -> Type {
        match self.prune(ty) {
            Type::Var(id) => map.get(&id).cloned().unwrap_or(Type::Var(id)),
            Type::Con(name, args) => Type::Con(
                name,
                args.iter().map(|arg| self.substitute(arg, map)).collect(),
            ),
            Type::Arrow(from, to) => arrow(self.substitute(&from, map), self.substitute(&to, map)),
            Type::Record(fields) => Type::Record(
                fields
                    .iter()
                    .map(|(label, ty)| (label.clone(), self.substitute(ty, map)))
                    .collect(),
            ),
        }
    }

    /// Overloaded variables that nothing pinned down become `int`.
    fn default_overloads(&mut self) {
        for id in std::mem::take(&mut self.overloaded) {
            if let Type::Var(root) = self.prune(&Type::Var(id))
                && let Some(candidates) = &self.vars[root].overload
                && let Some(first) = candidates.first()
            {
                self.vars[root].link = Some(con(first));
            }
        }
    }

    /// SML/NJ freezes the type variables a value binding could not generalise
    /// as dummy types (`?.X1`, ...) when the top-level declaration ends.
    fn instantiate_dummies(&mut self, schemes: &[(String, Scheme)]) {
        let mut number = 0;
        for (_, scheme) in schemes {
            let mut free = Vec::new();
            self.free_vars(&scheme.ty, &mut free);
            for id in free {
                if !scheme.vars.contains(&id) {
                    number += 1;
                    self.vars[id].link = Some(con(&format!("?.X{number}")));
                }
            }
        }
    }

    // --- environment -----------------------------------------------------

    fn mark(&self) -> Mark {
        Mark {
            values: self.values.len(),
            types: self.types.len(),
            structs: self.structs.len(),
            signatures: self.signatures.len(),
            functors: self.functors.len(),
        }
    }

    fn release(&mut self, mark: Mark) {
        self.values.truncate(mark.values);
        self.types.truncate(mark.types);
        self.structs.truncate(mark.structs);
        self.signatures.truncate(mark.signatures);
        self.functors.truncate(mark.functors);
    }

    fn push_monomorphic(&mut self, name: String, ty: Type) {
        self.values.push(Entry {
            name,
            scheme: Scheme {
                vars: Vec::new(),
                ty,
            },
            constructor: false,
        });
    }

    /// A value or constructor, possibly qualified by structure names.
    fn lookup(&self, name: &str) -> Option<Entry> {
        match name.rsplit_once('.') {
            None => self.values.iter().rev().find(|entry| entry.name == name),
            Some((path, base)) => match self.lookup_struct(path) {
                Some(found) => {
                    return found
                        .values
                        .iter()
                        .find(|entry| entry.name == base)
                        .cloned();
                }
                // Basis functions such as `Int.toString` are built in under
                // their qualified names, until the basis has structures.
                None => self.values.iter().rev().find(|entry| entry.name == name),
            },
        }
        .cloned()
    }

    /// A type name, possibly qualified.
    fn lookup_type(&self, name: &str) -> Option<TypeEntry> {
        match name.rsplit_once('.') {
            None => self
                .types
                .iter()
                .rev()
                .find(|(known, _)| known == name)
                .map(|(_, entry)| entry.clone()),
            Some((path, base)) => self
                .lookup_struct(path)?
                .types
                .iter()
                .find(|(known, _)| known == base)
                .map(|(_, entry)| entry.clone()),
        }
    }

    /// A structure, by name or qualified path.
    fn lookup_struct(&self, path: &str) -> Option<&StructEnv> {
        let mut parts = path.split('.');
        let first = parts.next()?;
        let mut found = self
            .structs
            .iter()
            .rev()
            .find(|(known, _)| known == first)
            .map(|(_, env)| env)?;
        for part in parts {
            found = found
                .structs
                .iter()
                .find(|(known, _)| known == part)
                .map(|(_, env)| env)?;
        }
        Some(found)
    }

    fn lookup_constructor(&self, name: &str) -> Option<Entry> {
        self.lookup(name).filter(|entry| entry.constructor)
    }

    // --- printing --------------------------------------------------------

    fn show(&self, ty: &Type, names: &mut Namer, precedence: u8) -> String {
        match self.prune(ty) {
            Type::Var(id) => names.name(id, self.vars[id].equality),
            Type::Con(tycon, args) => {
                let name = names.tycon(&tycon);
                match args.as_slice() {
                    [] => name,
                    [single] => format!("{} {name}", self.show(single, names, 2)),
                    many => {
                        let args: Vec<String> =
                            many.iter().map(|arg| self.show(arg, names, 0)).collect();
                        format!("({}) {name}", args.join(","))
                    }
                }
            }
            Type::Arrow(from, to) => {
                let text = format!(
                    "{} -> {}",
                    self.show(&from, names, 1),
                    self.show(&to, names, 0)
                );
                if precedence >= 1 {
                    format!("({text})")
                } else {
                    text
                }
            }
            Type::Record(fields) => {
                let is_tuple = !fields.is_empty()
                    && fields
                        .iter()
                        .enumerate()
                        .all(|(index, (label, _))| *label == (index + 1).to_string());
                if is_tuple && fields.len() > 1 {
                    let items: Vec<String> = fields
                        .iter()
                        .map(|(_, ty)| self.show(ty, names, 2))
                        .collect();
                    let text = items.join(" * ");
                    if precedence >= 2 {
                        format!("({text})")
                    } else {
                        text
                    }
                } else {
                    let items: Vec<String> = fields
                        .iter()
                        .map(|(label, ty)| format!("{label}:{}", self.show(ty, names, 0)))
                        .collect();
                    format!("{{{}}}", items.join(", "))
                }
            }
        }
    }

    /// The type of a top-level binding. Like SML/NJ, a datatype that is no
    /// longer in scope, because its declaration was local, is shown as `?.t`.
    fn show_scheme(&self, scheme: &Scheme) -> String {
        let mut namer = Namer::new();
        self.hide_inaccessible(&scheme.ty, &mut namer);
        self.show(&scheme.ty, &mut namer, 0)
    }

    fn hide_inaccessible(&self, ty: &Type, namer: &mut Namer) {
        match self.prune(ty) {
            Type::Con(tycon, args) => {
                let accessible = tycon.stamp == 0
                    || self
                        .types
                        .iter()
                        .rev()
                        .find(|(name, _)| *name == tycon.name)
                        .is_some_and(|(_, entry)| {
                            matches!(entry, TypeEntry::Data { tycon: found, .. } if *found == tycon)
                        });
                if !accessible {
                    match self.paths.get(&tycon.stamp) {
                        Some(path) => {
                            namer.qualified.insert(tycon.stamp, path.clone());
                        }
                        None => namer.hidden.push(tycon.stamp),
                    }
                }
                for arg in &args {
                    self.hide_inaccessible(arg, namer);
                }
            }
            Type::Arrow(from, to) => {
                self.hide_inaccessible(&from, namer);
                self.hide_inaccessible(&to, namer);
            }
            Type::Record(fields) => {
                for (_, field) in &fields {
                    self.hide_inaccessible(field, namer);
                }
            }
            Type::Var(_) => {}
        }
    }
}

/// Names variables `'a`, `'b`, ... (`''a` for equality variables) in order of
/// appearance.
struct Namer {
    names: HashMap<usize, String>,
    letters: usize,
    /// Stamps seen for each type name, so distinct types of one name can be
    /// told apart as `t` and `t/2`.
    tycons: HashMap<String, Vec<usize>>,
    /// Stamps of datatypes that are out of scope.
    hidden: Vec<usize>,
    /// Type constructors of structures, shown by their qualified names.
    qualified: HashMap<usize, String>,
}

impl Namer {
    fn new() -> Self {
        Self {
            names: HashMap::new(),
            letters: 0,
            tycons: HashMap::new(),
            hidden: Vec::new(),
            qualified: HashMap::new(),
        }
    }

    fn tycon(&mut self, tycon: &TyCon) -> String {
        if self.hidden.contains(&tycon.stamp) {
            return format!("?.{}", tycon.name);
        }
        if let Some(path) = self.qualified.get(&tycon.stamp) {
            return path.clone();
        }
        let stamps = self.tycons.entry(tycon.name.clone()).or_default();
        let index = match stamps.iter().position(|stamp| *stamp == tycon.stamp) {
            Some(index) => index,
            None => {
                stamps.push(tycon.stamp);
                stamps.len() - 1
            }
        };
        if index == 0 {
            tycon.name.clone()
        } else {
            format!("{}/{}", tycon.name, index + 1)
        }
    }

    fn name(&mut self, id: usize, equality: bool) -> String {
        if let Some(name) = self.names.get(&id) {
            return name.clone();
        }
        let index = self.letters;
        self.letters += 1;
        let letter = char::from(b'a' + (index % 26) as u8);
        let suffix = if index >= 26 {
            (index / 26).to_string()
        } else {
            String::new()
        };
        let name = format!("{}{letter}{suffix}", if equality { "''" } else { "'" });
        self.names.insert(id, name.clone());
        name
    }
}

// --- inference over the syntax tree -------------------------------------

impl Infer {
    fn convert(&mut self, ty: &SyntaxTy, variables: &mut HashMap<String, Type>) -> Res<Type> {
        match &ty.value {
            TyKind::Variable(name) => {
                if let Some(existing) = variables.get(name) {
                    return Ok(existing.clone());
                }
                let created = self.fresh_with(name.starts_with("''"));
                variables.insert(name.clone(), created.clone());
                Ok(created)
            }
            TyKind::Constructor(name, arguments) => {
                let mut converted = Vec::new();
                for argument in arguments {
                    converted.push(self.convert(argument, variables)?);
                }
                let arity_error = |expected: usize, found: usize| {
                    (
                        TypeError::TypeArity {
                            name: name.clone(),
                            expected,
                            found,
                        },
                        ty.source_span(),
                    )
                };
                match self.lookup_type(name) {
                    Some(TypeEntry::Alias(alias)) => {
                        if alias.params.len() != converted.len() {
                            return Err(arity_error(alias.params.len(), converted.len()));
                        }
                        let map: HashMap<usize, Type> =
                            alias.params.iter().copied().zip(converted).collect();
                        return Ok(self.substitute(&alias.body, &map));
                    }
                    Some(TypeEntry::Data { tycon, arity }) => {
                        if arity != converted.len() {
                            return Err(arity_error(arity, converted.len()));
                        }
                        return Ok(Type::Con(tycon, converted));
                    }
                    None => {}
                }
                if let Some((path, _)) = name.rsplit_once('.') {
                    return Err((
                        if self.lookup_struct(path).is_some() {
                            TypeError::UnboundType(name.clone())
                        } else {
                            TypeError::UnboundStructure(path.to_string())
                        },
                        ty.source_span(),
                    ));
                }
                match TYPE_CONSTRUCTORS.iter().find(|(known, _)| known == name) {
                    Some((_, arity)) if *arity == converted.len() => {
                        Ok(Type::Con(TyCon::builtin(name), converted))
                    }
                    Some((_, arity)) => Err((
                        TypeError::TypeArity {
                            name: name.clone(),
                            expected: *arity,
                            found: converted.len(),
                        },
                        ty.source_span(),
                    )),
                    None => Err((TypeError::UnboundType(name.clone()), ty.source_span())),
                }
            }
            TyKind::Tuple(items) => {
                let mut converted = Vec::new();
                for item in items {
                    converted.push(self.convert(item, variables)?);
                }
                Ok(tuple(converted))
            }
            TyKind::Arrow(from, to) => {
                let from = self.convert(from, variables)?;
                let to = self.convert(to, variables)?;
                Ok(arrow(from, to))
            }
            TyKind::Record(fields) => {
                let mut converted = Vec::new();
                for (label, field) in fields {
                    converted.push((label.clone(), self.convert(field, variables)?));
                }
                sort_labels(&mut converted);
                Ok(Type::Record(converted))
            }
        }
    }

    fn annotation(&mut self, ty: &SyntaxTy) -> Res<Type> {
        let mut variables = std::mem::take(&mut self.tyvars);
        let result = self.convert(ty, &mut variables);
        self.tyvars = variables;
        result
    }

    fn infer_pat(&mut self, pat: &Pat, binds: &mut Vec<(String, Type)>) -> Res<Type> {
        let ty = self.infer_pat_kind(pat, binds)?;
        self.node_types.push(Node {
            address: pat as *const Pat as usize,
            pattern: true,
            start: pat.start.clone(),
            end: pat.end.clone(),
            ty: ty.clone(),
        });
        Ok(ty)
    }

    fn infer_pat_kind(&mut self, pat: &Pat, binds: &mut Vec<(String, Type)>) -> Res<Type> {
        let span = pat.source_span();
        Ok(match &pat.value {
            PatKind::Wildcard => self.fresh(),
            PatKind::Variable(name) => {
                if name.contains('.') && self.lookup_constructor(name).is_none() {
                    return Err((TypeError::UnboundConstructor(name.clone()), span));
                }
                if let Some(entry) = self.lookup_constructor(name) {
                    let ty = self.instantiate(&entry.scheme);
                    if matches!(self.prune(&ty), Type::Arrow(..)) {
                        return Err((TypeError::ConstructorNeedsArgument(name.clone()), span));
                    }
                    ty
                } else {
                    let ty = self.fresh();
                    binds.push((name.clone(), ty.clone()));
                    ty
                }
            }
            PatKind::Integer(value) if !value::int_fits(*value) => {
                return Err((TypeError::IntegerOutOfRange, span));
            }
            PatKind::Integer(_) => con("int"),
            PatKind::Word(_) => con("word"),
            PatKind::String(_) => con("string"),
            PatKind::Character(_) => con("char"),
            PatKind::Boolean(_) => con("bool"),
            PatKind::Unit => con("unit"),
            PatKind::Tuple(items) => {
                let mut types = Vec::new();
                for item in items {
                    types.push(self.infer_pat(item, binds)?);
                }
                tuple(types)
            }
            PatKind::List(items) => {
                let element = self.fresh();
                for item in items {
                    let found = self.infer_pat(item, binds)?;
                    self.unify_at(&element, &found, item.source_span())?;
                }
                list(element)
            }
            PatKind::Record(fields, true) => {
                let mut types = Vec::new();
                for (label, field) in fields {
                    types.push((label.clone(), self.infer_pat(field, binds)?));
                }
                self.flexible_record(types)
            }
            PatKind::Record(fields, false) => {
                let mut sorted: Vec<&(String, Pat)> = fields.iter().collect();
                sorted.sort_by_key(|(label, _)| label_key(label));
                let mut types = Vec::new();
                for (label, field) in sorted {
                    types.push((label.clone(), self.infer_pat(field, binds)?));
                }
                Type::Record(types)
            }
            PatKind::Constructor(name, argument) => {
                let Some(entry) = self.lookup_constructor(name) else {
                    return Err((TypeError::UnboundConstructor(name.clone()), span));
                };
                let ty = self.instantiate(&entry.scheme);
                let Type::Arrow(parameter, result) = self.prune(&ty) else {
                    return Err((TypeError::ConstructorTakesNoArgument(name.clone()), span));
                };
                let found = self.infer_pat(argument, binds)?;
                self.unify_at(&parameter, &found, argument.source_span())?;
                *result
            }
            PatKind::Cons(head, tail) => {
                let head_type = self.infer_pat(head, binds)?;
                let tail_type = self.infer_pat(tail, binds)?;
                let expected = list(head_type);
                self.unify_at(&expected, &tail_type, tail.source_span())?;
                expected
            }
            PatKind::Layered(name, annotation, inner) => {
                let position = binds.len();
                let ty = self.infer_pat(inner, binds)?;
                if let Some(annotation) = annotation {
                    let annotated = self.annotation(annotation)?;
                    self.unify_at(&annotated, &ty, span)?;
                }
                binds.insert(position, (name.clone(), ty.clone()));
                ty
            }
            PatKind::Typed(inner, annotation) => {
                let ty = self.infer_pat(inner, binds)?;
                let annotated = self.annotation(annotation)?;
                self.unify_at(&annotated, &ty, span)?;
                annotated
            }
        })
    }

    fn apply(
        &mut self,
        function: &Type,
        argument: &Type,
        function_span: SourceSpan,
        argument_span: SourceSpan,
    ) -> Res<Type> {
        match self.prune(function) {
            Type::Arrow(parameter, result) => {
                self.unify_at(&parameter, argument, argument_span)?;
                Ok(*result)
            }
            Type::Var(_) => {
                let result = self.fresh();
                let expected = arrow(argument.clone(), result.clone());
                self.unify_at(function, &expected, function_span)?;
                Ok(result)
            }
            other => {
                let shown = self.show(&other, &mut Namer::new(), 0);
                Err((TypeError::NotAFunction(shown), function_span))
            }
        }
    }

    fn overloaded_operator(
        &mut self,
        candidates: &'static [&'static str],
        lhs: &Expr,
        rhs: &Expr,
        result: Option<Type>,
    ) -> Res<Type> {
        let operand = self.overloaded_var(candidates);
        let left = self.infer_expr(lhs)?;
        self.unify_at(&operand, &left, lhs.source_span())?;
        let right = self.infer_expr(rhs)?;
        self.unify_at(&operand, &right, rhs.source_span())?;
        Ok(result.unwrap_or(operand))
    }

    fn infer_expr(&mut self, expr: &Expr) -> Res<Type> {
        let ty = self.infer_expr_kind(expr)?;
        self.node_types.push(Node {
            address: expr as *const Expr as usize,
            pattern: false,
            start: expr.start.clone(),
            end: expr.end.clone(),
            ty: ty.clone(),
        });
        Ok(ty)
    }

    fn infer_expr_kind(&mut self, expr: &Expr) -> Res<Type> {
        let span = expr.source_span();
        match &expr.value {
            ExprKind::Integer(value) if !value::int_fits(*value) => {
                Err((TypeError::IntegerOutOfRange, span))
            }
            ExprKind::Integer(_) => Ok(con("int")),
            ExprKind::Real(_) => Ok(con("real")),
            ExprKind::Boolean(_) => Ok(con("bool")),
            ExprKind::String(_) => Ok(con("string")),
            ExprKind::Character(_) => Ok(con("char")),
            ExprKind::Word(_) => Ok(con("word")),
            ExprKind::Unit => Ok(con("unit")),
            ExprKind::Variable(name) => self.variable(name, span),
            ExprKind::Add(lhs, rhs)
            | ExprKind::Subtract(lhs, rhs)
            | ExprKind::Multiply(lhs, rhs) => self.overloaded_operator(NUMERIC, lhs, rhs, None),
            ExprKind::IntDivide(lhs, rhs) => self.overloaded_operator(INTEGRAL, lhs, rhs, None),
            ExprKind::Divide(lhs, rhs) => {
                let real = con("real");
                let left = self.infer_expr(lhs)?;
                self.unify_at(&real, &left, lhs.source_span())?;
                let right = self.infer_expr(rhs)?;
                self.unify_at(&real, &right, rhs.source_span())?;
                Ok(real)
            }
            ExprKind::Greater(lhs, rhs)
            | ExprKind::GreaterEqual(lhs, rhs)
            | ExprKind::Less(lhs, rhs)
            | ExprKind::LessEqual(lhs, rhs) => {
                self.overloaded_operator(ORDERED, lhs, rhs, Some(con("bool")))
            }
            ExprKind::Equal(lhs, rhs) | ExprKind::NotEqual(lhs, rhs) => {
                let operand = self.fresh_with(true);
                let left = self.infer_expr(lhs)?;
                self.unify_at(&operand, &left, lhs.source_span())?;
                let right = self.infer_expr(rhs)?;
                self.unify_at(&operand, &right, rhs.source_span())?;
                Ok(con("bool"))
            }
            ExprKind::AndAlso(lhs, rhs) | ExprKind::OrElse(lhs, rhs) => {
                for side in [lhs, rhs] {
                    let found = self.infer_expr(side)?;
                    self.unify_at(&con("bool"), &found, side.source_span())?;
                }
                Ok(con("bool"))
            }
            ExprKind::If(condition, consequent, alternative) => {
                let found = self.infer_expr(condition)?;
                self.unify_at(&con("bool"), &found, condition.source_span())?;
                let then_type = self.infer_expr(consequent)?;
                let else_type = self.infer_expr(alternative)?;
                self.unify_at(&then_type, &else_type, alternative.source_span())?;
                Ok(then_type)
            }
            ExprKind::List(items) => {
                let element = self.fresh();
                for item in items {
                    let found = self.infer_expr(item)?;
                    self.unify_at(&element, &found, item.source_span())?;
                }
                Ok(list(element))
            }
            ExprKind::Tuple(items) => {
                let mut types = Vec::new();
                for item in items {
                    types.push(self.infer_expr(item)?);
                }
                Ok(tuple(types))
            }
            ExprKind::Sequence(items) => {
                let mut last = con("unit");
                for item in items {
                    last = self.infer_expr(item)?;
                }
                Ok(last)
            }
            ExprKind::Record(fields) => {
                let mut types = Vec::new();
                for (label, value) in fields {
                    types.push((label.clone(), self.infer_expr(value)?));
                }
                sort_labels(&mut types);
                Ok(Type::Record(types))
            }
            ExprKind::Selector(label) => {
                let field = self.fresh();
                let record = self.flexible_record(vec![(label.clone(), field.clone())]);
                Ok(arrow(record, field))
            }
            ExprKind::Apply(function, argument) => {
                let function_type = self.infer_expr(function)?;
                let argument_type = self.infer_expr(argument)?;
                self.apply(
                    &function_type,
                    &argument_type,
                    function.source_span(),
                    argument.source_span(),
                )
            }
            ExprKind::Infix(name, lhs, rhs) => {
                let function_type = self.variable(name, span)?;
                let left = self.infer_expr(lhs)?;
                let right = self.infer_expr(rhs)?;
                self.apply(&function_type, &tuple(vec![left, right]), span, span)
            }
            ExprKind::Let(declarations, body) => {
                let mark = self.mark();
                for declaration in declarations {
                    self.infer_decl(declaration)?;
                }
                let result = self.infer_expr(body);
                self.release(mark);
                result
            }
            ExprKind::Case(scrutinee, rules) => {
                let scrutinee_type = self.infer_expr(scrutinee)?;
                let result = self.fresh();
                self.infer_rules(rules, &scrutinee_type, &result)?;
                Ok(result)
            }
            ExprKind::Fn(rules) => {
                let argument = self.fresh();
                let result = self.fresh();
                self.infer_rules(rules, &argument, &result)?;
                Ok(arrow(argument, result))
            }
            ExprKind::While(condition, body) => {
                let found = self.infer_expr(condition)?;
                self.unify_at(&con("bool"), &found, condition.source_span())?;
                self.infer_expr(body)?;
                Ok(con("unit"))
            }
            ExprKind::Raise(inner) => {
                let found = self.infer_expr(inner)?;
                self.unify_at(&con("exn"), &found, inner.source_span())?;
                Ok(self.fresh())
            }
            ExprKind::Handle(body, rules) => {
                let result = self.infer_expr(body)?;
                self.infer_rules(rules, &con("exn"), &result)?;
                Ok(result)
            }
            ExprKind::Typed(inner, annotation) => {
                let found = self.infer_expr(inner)?;
                let annotated = self.annotation(annotation)?;
                self.unify_at(&annotated, &found, span)?;
                Ok(annotated)
            }
            ExprKind::Word8FromInt(inner) => {
                let found = self.infer_expr(inner)?;
                self.unify_at(&con("int"), &found, inner.source_span())?;
                Ok(con("word8"))
            }
            ExprKind::PosixExit(inner) => {
                let found = self.infer_expr(inner)?;
                self.unify_at(&con("word8"), &found, inner.source_span())?;
                Ok(self.fresh())
            }
        }
    }

    fn variable(&mut self, name: &str, span: SourceSpan) -> Res<Type> {
        if let Some(entry) = self.lookup(name) {
            return Ok(self.instantiate(&entry.scheme));
        }
        if let Some((path, _)) = name.rsplit_once('.')
            && self.lookup_struct(path).is_none()
        {
            return Err((TypeError::UnboundStructure(path.to_string()), span));
        }
        if name == "mod" {
            let operand = self.overloaded_var(INTEGRAL);
            return Ok(arrow(
                tuple(vec![operand.clone(), operand.clone()]),
                operand,
            ));
        }
        Err((TypeError::UnboundVariable(name.to_string()), span))
    }

    fn infer_rules(&mut self, rules: &[Rule], argument: &Type, result: &Type) -> Res<()> {
        for (pattern, body) in rules {
            let mark = self.mark();
            let mut binds = Vec::new();
            let found = self.infer_pat(pattern, &mut binds)?;
            self.unify_at(argument, &found, pattern.source_span())?;
            for (name, ty) in binds {
                self.push_monomorphic(name, ty);
            }
            let body_type = self.infer_expr(body)?;
            self.unify_at(result, &body_type, body.source_span())?;
            self.release(mark);
        }
        Ok(())
    }

    /// Only these expressions may have their type generalised (value restriction).
    fn nonexpansive(&self, expr: &Expr) -> bool {
        match &expr.value {
            ExprKind::Integer(_)
            | ExprKind::Real(_)
            | ExprKind::Boolean(_)
            | ExprKind::String(_)
            | ExprKind::Character(_)
            | ExprKind::Word(_)
            | ExprKind::Unit
            | ExprKind::Variable(_)
            | ExprKind::Fn(_)
            | ExprKind::Selector(_) => true,
            ExprKind::Tuple(items) | ExprKind::List(items) => {
                items.iter().all(|item| self.nonexpansive(item))
            }
            ExprKind::Record(fields) => fields.iter().all(|(_, value)| self.nonexpansive(value)),
            ExprKind::Typed(inner, _) => self.nonexpansive(inner),
            ExprKind::Apply(function, argument) => {
                // `ref` is a constructor, but allocating a cell is an effect, so
                // `ref e` is never generalised.
                matches!(&function.value, ExprKind::Variable(name) if name != "ref" && self.lookup_constructor(name).is_some())
                    && self.nonexpansive(argument)
            }
            ExprKind::Infix(name, lhs, rhs) => {
                name == "::" && self.nonexpansive(lhs) && self.nonexpansive(rhs)
            }
            _ => false,
        }
    }

    fn scheme_for(&mut self, ty: &Type, generalise: bool) -> Scheme {
        if generalise {
            self.generalize(ty)
        } else {
            self.monomorphic(ty)
        }
    }

    fn define(&mut self, name: String, scheme: Scheme) -> (String, Scheme) {
        self.values.push(Entry {
            name: name.clone(),
            scheme: scheme.clone(),
            constructor: false,
        });
        (name, scheme)
    }

    /// `val name = expr`, the simple form the backend already handles.
    fn infer_simple_val(&mut self, name: &str, expr: &Expr) -> Res<Vec<(String, Scheme)>> {
        self.level += 1;
        let inferred = self.infer_expr(expr);
        self.level -= 1;
        let ty = inferred?;
        self.check_flexible(expr.source_span(), false)?;
        let scheme = self.scheme_for(&ty, self.nonexpansive(expr));
        Ok(vec![self.define(name.to_string(), scheme)])
    }

    /// Infers a declaration, extends the environment with what it binds and
    /// returns those value bindings in order.
    fn infer_decl(&mut self, decl: &Decl) -> Res<Vec<(String, Scheme)>> {
        match &decl.value {
            DeclKind::Val {
                recursive: false,
                bindings,
            } => {
                let mut pending = Vec::new();
                for (pattern, expr) in bindings {
                    self.level += 1;
                    let result = self.infer_val_binding(pattern, expr);
                    self.level -= 1;
                    let binds = result?;
                    self.check_flexible(decl.source_span(), false)?;
                    let general = self.nonexpansive(expr);
                    for (name, ty) in binds {
                        pending.push((name, self.scheme_for(&ty, general)));
                    }
                }
                Ok(pending
                    .into_iter()
                    .map(|(name, scheme)| self.define(name, scheme))
                    .collect())
            }
            DeclKind::Val {
                recursive: true,
                bindings,
            } => {
                self.level += 1;
                let result = self.infer_val_rec(bindings);
                self.level -= 1;
                let binds = result?;
                self.check_flexible(decl.source_span(), false)?;
                Ok(binds
                    .into_iter()
                    .map(|(name, ty)| {
                        let scheme = self.generalize(&ty);
                        self.define(name, scheme)
                    })
                    .collect())
            }
            DeclKind::Fun(bindings) => {
                self.level += 1;
                let result = self.infer_functions(bindings);
                self.level -= 1;
                let functions = result?;
                self.check_flexible(decl.source_span(), false)?;
                Ok(functions
                    .into_iter()
                    .map(|(name, ty)| {
                        let scheme = self.generalize(&ty);
                        self.define(name, scheme)
                    })
                    .collect())
            }
            DeclKind::Type(bindings) => {
                let aliases = self.convert_aliases(bindings)?;
                self.types.extend(aliases);
                Ok(Vec::new())
            }
            DeclKind::Datatype { bindings, withtype } => {
                let stamps = self.infer_datatypes(bindings, withtype)?;
                self.declaration_constructors.insert(
                    decl as *const _ as usize,
                    bindings
                        .iter()
                        .zip(stamps)
                        .flat_map(|(binding, tycon)| {
                            binding.constructors.iter().map(move |constructor| {
                                (constructor.value.name.clone(), tycon.stamp)
                            })
                        })
                        .collect(),
                );
                Ok(Vec::new())
            }
            DeclKind::Exception(bindings) => {
                self.infer_exceptions(bindings, decl.source_span())?;
                let arguments = bindings
                    .iter()
                    .map(|binding| {
                        let entry = self.lookup(&binding.name).expect("checked exception");
                        match self.resolve(&entry.scheme.ty) {
                            Ty::Arrow(argument, _) => Some(*argument),
                            _ => None,
                        }
                    })
                    .collect();
                self.exception_arguments
                    .insert(decl as *const _ as usize, arguments);
                Ok(Vec::new())
            }
            DeclKind::DatatypeCopy { name, original } => {
                let before = self.values.len();
                self.copy_datatype(name, original, decl.source_span())?;
                self.declaration_constructors.insert(
                    decl as *const _ as usize,
                    self.values[before..]
                        .iter()
                        .map(|entry| {
                            let ty = self.prune(&entry.scheme.ty);
                            let ty = match ty {
                                Type::Arrow(_, result) => self.prune(&result),
                                ty => ty,
                            };
                            let Type::Con(tycon, _) = ty else {
                                unreachable!("datatype constructor result")
                            };
                            (entry.name.clone(), tycon.stamp)
                        })
                        .collect(),
                );
                Ok(Vec::new())
            }
            DeclKind::Abstype {
                bindings,
                withtype,
                body,
            } => {
                let before = self.values.len();
                let tycons = self.infer_datatypes(bindings, withtype)?;
                self.declaration_constructors.insert(
                    decl as *const _ as usize,
                    bindings
                        .iter()
                        .zip(&tycons)
                        .flat_map(|(binding, tycon)| {
                            binding.constructors.iter().map(move |constructor| {
                                (constructor.value.name.clone(), tycon.stamp)
                            })
                        })
                        .collect(),
                );
                let after = self.values.len();
                let mut bound = Vec::new();
                for declaration in body {
                    bound.extend(self.infer_decl(declaration)?);
                }
                // The constructors are private to the body, and outside it
                // the type is abstract: no equality.
                self.values.drain(before..after);
                for tycon in tycons {
                    self.abstract_datatypes.insert(tycon.stamp);
                    if let Some(info) = self.datatypes.get_mut(&tycon.stamp) {
                        info.never_equal = true;
                    }
                }
                Ok(bound)
            }
            DeclKind::Local(private, public) => {
                let mark = self.mark();
                for declaration in private {
                    self.infer_decl(declaration)?;
                }
                let visible = self.mark();
                let mut bound = Vec::new();
                for declaration in public {
                    bound.extend(self.infer_decl(declaration)?);
                }
                let values: Vec<Entry> = self.values.drain(visible.values..).collect();
                let types: Vec<(String, TypeEntry)> = self.types.drain(visible.types..).collect();
                let structs: Vec<(String, StructEnv)> =
                    self.structs.drain(visible.structs..).collect();
                let signatures: Vec<(String, Sig)> =
                    self.signatures.drain(visible.signatures..).collect();
                let functors: Vec<(String, std::rc::Rc<Functor>)> =
                    self.functors.drain(visible.functors..).collect();
                self.release(mark);
                self.values.extend(values);
                self.types.extend(types);
                self.structs.extend(structs);
                self.signatures.extend(signatures);
                self.functors.extend(functors);
                Ok(bound)
            }
            DeclKind::Structure(bindings) => {
                self.infer_structures(bindings)?;
                Ok(Vec::new())
            }
            DeclKind::Signature(bindings) => {
                self.infer_signatures(bindings)?;
                Ok(Vec::new())
            }
            DeclKind::Functor(bindings) => {
                self.infer_functors(bindings)?;
                Ok(Vec::new())
            }
            DeclKind::Open(paths) => {
                self.open_structures(paths, decl.source_span())?;
                let mut bound = Vec::new();
                for path in paths {
                    if let Some(env) = self.lookup_struct(path) {
                        bound.extend(
                            env.values
                                .iter()
                                .filter(|entry| !entry.constructor)
                                .map(|entry| (entry.name.clone(), entry.scheme.clone())),
                        );
                    }
                }
                Ok(bound)
            }
            DeclKind::Fixity { .. } => Ok(Vec::new()),
        }
    }

    /// The abbreviations of a `type` (or `withtype`) declaration; they cannot
    /// see one another.
    fn convert_aliases(
        &mut self,
        bindings: &[crate::parser::TypeBinding],
    ) -> Res<Vec<(String, TypeEntry)>> {
        let mut aliases = Vec::new();
        for binding in bindings {
            let mut variables = HashMap::new();
            let mut params = Vec::new();
            for parameter in &binding.parameters {
                let ty = self.fresh();
                if let Type::Var(id) = &ty {
                    params.push(*id);
                }
                variables.insert(parameter.clone(), ty);
            }
            let body = self.convert(&binding.ty, &mut variables)?;
            self.check_closed(
                &variables,
                binding.parameters.len(),
                &binding.parameters,
                &binding.ty,
            )?;
            aliases.push((
                binding.name.clone(),
                TypeEntry::Alias(Alias { params, body }),
            ));
        }
        Ok(aliases)
    }

    /// A declaration's types may only mention the type variables it binds.
    fn check_closed(
        &self,
        variables: &HashMap<String, Type>,
        bound: usize,
        parameters: &[String],
        ty: &SyntaxTy,
    ) -> Res<()> {
        if variables.len() > bound
            && let Some(name) = variables.keys().find(|name| !parameters.contains(name))
        {
            return Err((
                TypeError::UnboundTypeVariable(name.clone()),
                ty.source_span(),
            ));
        }
        Ok(())
    }

    /// Declares the types and constructors of a `datatype ... and ...` group
    /// and returns the new type constructors.
    fn infer_datatypes(
        &mut self,
        bindings: &[DataBinding],
        withtype: &[crate::parser::TypeBinding],
    ) -> Res<Vec<TyCon>> {
        // The names come first: constructors and abbreviations may refer to
        // any datatype of the group.
        let mut tycons = Vec::new();
        for binding in bindings {
            self.next_stamp += 1;
            let tycon = TyCon {
                name: binding.name.clone(),
                stamp: self.next_stamp,
            };
            self.types.push((
                binding.name.clone(),
                TypeEntry::Data {
                    tycon: tycon.clone(),
                    arity: binding.parameters.len(),
                },
            ));
            tycons.push(tycon);
        }
        let aliases = self.convert_aliases(withtype)?;
        self.types.extend(aliases);

        let mut declared = Vec::new();
        let mut parameter_vars = Vec::new();
        let mut argument_types = Vec::new();
        for (binding, tycon) in bindings.iter().zip(&tycons) {
            let mut variables = HashMap::new();
            let mut params = Vec::new();
            let mut parameter_types = Vec::new();
            for parameter in &binding.parameters {
                let ty = self.fresh();
                if let Type::Var(id) = &ty {
                    params.push(*id);
                }
                variables.insert(parameter.clone(), ty.clone());
                parameter_types.push(ty);
            }
            let result = Type::Con(tycon.clone(), parameter_types);
            let mut constructors = Vec::new();
            let mut arguments = Vec::new();
            for constructor in &binding.constructors {
                let argument = match &constructor.value.argument {
                    Some(ty) => {
                        let converted = self.convert(ty, &mut variables)?;
                        self.check_closed(&variables, params.len(), &binding.parameters, ty)?;
                        Some(converted)
                    }
                    None => None,
                };
                let ty = match &argument {
                    Some(argument) => arrow(argument.clone(), result.clone()),
                    None => result.clone(),
                };
                constructors.push((
                    constructor.value.name.clone(),
                    Scheme {
                        vars: params.clone(),
                        ty,
                    },
                ));
                arguments.push(argument);
            }
            declared.push(constructors);
            parameter_vars.push(params);
            argument_types.push(arguments);
        }

        let equality = self.datatype_equality(&tycons, &parameter_vars, &argument_types);
        for (((tycon, constructors), (never_equal, equal_params)), _) in tycons
            .iter()
            .zip(declared)
            .zip(equality)
            .zip(&parameter_vars)
        {
            for (name, scheme) in &constructors {
                self.values.push(Entry {
                    name: name.clone(),
                    scheme: scheme.clone(),
                    constructor: true,
                });
            }
            self.datatypes.insert(
                tycon.stamp,
                DataInfo {
                    constructors,
                    never_equal,
                    equal_params,
                },
            );
        }
        Ok(tycons)
    }

    /// For each datatype of a group: whether it never admits equality, and
    /// which of its parameters it needs to. Datatypes of the group refer to
    /// each other, so this iterates to a fixed point.
    fn datatype_equality(
        &self,
        tycons: &[TyCon],
        parameters: &[Vec<usize>],
        arguments: &[Vec<Option<Type>>],
    ) -> Vec<(bool, Vec<bool>)> {
        let mut state: Vec<(bool, Vec<bool>)> = parameters
            .iter()
            .map(|params| (false, vec![false; params.len()]))
            .collect();
        loop {
            let before = state.clone();
            for (index, arguments) in arguments.iter().enumerate() {
                for argument in arguments.iter().flatten() {
                    self.equality_walk(argument, index, tycons, parameters, &mut state);
                }
            }
            if state == before {
                return state;
            }
        }
    }

    fn equality_walk(
        &self,
        ty: &Type,
        index: usize,
        tycons: &[TyCon],
        parameters: &[Vec<usize>],
        state: &mut Vec<(bool, Vec<bool>)>,
    ) {
        match self.prune(ty) {
            Type::Var(id) => {
                if let Some(position) = parameters[index].iter().position(|param| *param == id) {
                    state[index].1[position] = true;
                }
            }
            Type::Arrow(..) => state[index].0 = true,
            Type::Record(fields) => {
                for (_, field) in &fields {
                    self.equality_walk(field, index, tycons, parameters, state);
                }
            }
            Type::Con(name, _) if name.is("real") || name.is("exn") => state[index].0 = true,
            Type::Con(name, _) if name.is("ref") => {}
            Type::Con(name, args) => {
                let (never, needed) = match tycons.iter().position(|tycon| *tycon == name) {
                    Some(other) => state[other].clone(),
                    None => match self.datatypes.get(&name.stamp) {
                        Some(info) => (info.never_equal, info.equal_params.clone()),
                        None => (false, vec![false; args.len()]),
                    },
                };
                if never {
                    state[index].0 = true;
                }
                for (arg, needed) in args.iter().zip(needed) {
                    if needed {
                        self.equality_walk(arg, index, tycons, parameters, state);
                    }
                }
            }
        }
    }

    /// Exception constructors are constructors of `exn`; each declaration makes
    /// new ones, and a replication reuses the type of an existing one.
    fn infer_exceptions(
        &mut self,
        bindings: &[crate::parser::ExceptionBinding],
        span: SourceSpan,
    ) -> Res<()> {
        let mut declared = Vec::new();
        for binding in bindings {
            let scheme = match &binding.kind {
                ExceptionKind::Fresh(None) => Scheme {
                    vars: Vec::new(),
                    ty: con("exn"),
                },
                ExceptionKind::Fresh(Some(ty)) => {
                    let mut variables = HashMap::new();
                    let argument = self.convert(ty, &mut variables)?;
                    if let Some(name) = variables.keys().next() {
                        return Err((
                            TypeError::UnboundTypeVariable(name.clone()),
                            ty.source_span(),
                        ));
                    }
                    Scheme {
                        vars: Vec::new(),
                        ty: arrow(argument, con("exn")),
                    }
                }
                ExceptionKind::Copy(original) => {
                    let found = self.lookup_constructor(original).filter(|entry| {
                        let ty = self.prune(&entry.scheme.ty);
                        let result = match ty {
                            Type::Arrow(_, result) => self.prune(&result),
                            other => other,
                        };
                        matches!(result, Type::Con(name, _) if name.is("exn"))
                    });
                    let Some(entry) = found else {
                        return Err((TypeError::NotAnException(original.clone()), span));
                    };
                    entry.scheme
                }
            };
            declared.push(Entry {
                name: binding.name.clone(),
                scheme,
                constructor: true,
            });
        }
        // Like `and`, the bindings cannot see each other.
        self.values.extend(declared);
        Ok(())
    }

    /// `datatype t = datatype u`.
    fn copy_datatype(&mut self, name: &str, original: &str, span: SourceSpan) -> Res<()> {
        let entry = self.lookup_type(original);
        let Some(TypeEntry::Data { tycon, arity }) = entry else {
            let error = match TYPE_CONSTRUCTORS
                .iter()
                .any(|(known, _)| *known == original)
            {
                true => TypeError::Unsupported("replications of built-in datatypes"),
                false => TypeError::UnboundType(original.to_string()),
            };
            return Err((error, span));
        };
        let constructors = self
            .datatypes
            .get(&tycon.stamp)
            .map(|info| info.constructors.clone())
            .unwrap_or_default();
        self.types
            .push((name.to_string(), TypeEntry::Data { tycon, arity }));
        for (name, scheme) in constructors {
            self.values.push(Entry {
                name,
                scheme,
                constructor: true,
            });
        }
        Ok(())
    }

    fn infer_val_binding(&mut self, pattern: &Pat, expr: &Expr) -> Res<Vec<(String, Type)>> {
        let expr_type = self.infer_expr(expr)?;
        let mut binds = Vec::new();
        let pattern_type = self.infer_pat(pattern, &mut binds)?;
        self.unify_at(&pattern_type, &expr_type, expr.source_span())?;
        Ok(binds)
    }

    fn infer_val_rec(&mut self, bindings: &[(Pat, Expr)]) -> Res<Vec<(String, Type)>> {
        let mut all = Vec::new();
        let mut pattern_types = Vec::new();
        for (pattern, _) in bindings {
            pattern_types.push(self.infer_pat(pattern, &mut all)?);
        }
        let mark = self.mark();
        for (name, ty) in &all {
            self.push_monomorphic(name.clone(), ty.clone());
        }
        for ((_, expr), pattern_type) in bindings.iter().zip(&pattern_types) {
            let found = self.infer_expr(expr)?;
            self.unify_at(pattern_type, &found, expr.source_span())?;
        }
        self.release(mark);
        Ok(all)
    }

    fn infer_functions(
        &mut self,
        bindings: &[crate::parser::FunBinding],
    ) -> Res<Vec<(String, Type)>> {
        let mark = self.mark();
        let mut function_types = Vec::new();
        for binding in bindings {
            let ty = self.fresh();
            function_types.push(ty.clone());
            self.push_monomorphic(binding.name.clone(), ty);
        }
        for (binding, function_type) in bindings.iter().zip(&function_types) {
            for clause in &binding.clauses {
                let inner = self.mark();
                let mut binds = Vec::new();
                let mut parameters = Vec::new();
                for parameter in &clause.parameters {
                    parameters.push(self.infer_pat(parameter, &mut binds)?);
                }
                for (name, ty) in binds {
                    self.push_monomorphic(name, ty);
                }
                let mut ty = self.infer_expr(&clause.body)?;
                for parameter in parameters.into_iter().rev() {
                    ty = arrow(parameter, ty);
                }
                self.unify_at(function_type, &ty, clause.body.source_span())?;
                self.release(inner);
            }
        }
        self.release(mark);
        Ok(bindings
            .iter()
            .map(|binding| binding.name.clone())
            .zip(function_types)
            .collect())
    }
}

/// Infers every top-level declaration and returns what each bound, or the
/// first type error.
pub fn check_program(program: &Program) -> Result<Checked, Failure> {
    Session::new().check(program)
}

/// A type checker that keeps its environment from one program to the next,
/// as the REPL needs.
#[derive(Clone)]
pub struct Session {
    infer: Infer,
}

impl Default for Session {
    fn default() -> Self {
        Self::new()
    }
}

impl Session {
    pub fn new() -> Self {
        Self {
            infer: Infer::new(),
        }
    }

    pub fn binding_type(&self, ty: &Ty) -> Ty {
        match ty {
            Ty::Con { name, stamp, args } => {
                let mut shown = name.clone();
                if *stamp != 0 {
                    let mut seen = HashSet::new();
                    if let Some((name, _)) = self.infer.types.iter().rev().find(|(name, entry)| {
                        seen.insert(name.clone()) && matches!(entry, TypeEntry::Data { tycon, .. } if tycon.stamp == *stamp)
                    }) { shown = name.clone(); }
                    else if let Some(path) = self.infer.paths.get(stamp) { shown = path.clone(); }
                    else { shown = name.clone(); }
                }
                Ty::Con {
                    name: shown,
                    stamp: *stamp,
                    args: args.iter().map(|ty| self.binding_type(ty)).collect(),
                }
            }
            Ty::Arrow(from, to) => Ty::Arrow(
                Box::new(self.binding_type(from)),
                Box::new(self.binding_type(to)),
            ),
            Ty::Record(fields) => Ty::Record(
                fields
                    .iter()
                    .map(|(name, ty)| (name.clone(), self.binding_type(ty)))
                    .collect(),
            ),
            ty => ty.clone(),
        }
    }

    pub fn constructors(&self, ty: &Ty) -> Option<Vec<(String, Option<Ty>)>> {
        let Ty::Con { stamp, args, .. } = ty else {
            return None;
        };
        let info = self.infer.datatypes.get(stamp)?;
        if self.infer.abstract_datatypes.contains(stamp) {
            return None;
        }
        Some(
            info.constructors
                .iter()
                .map(|(name, scheme)| {
                    let argument = match self.infer.resolve(&scheme.ty) {
                        Ty::Arrow(argument, _) => Some(substitute(&argument, &scheme.vars, args)),
                        _ => None,
                    };
                    (name.clone(), argument)
                })
                .collect(),
        )
    }

    /// Checks `program` in the environment left by earlier programs. On an
    /// error the environment is left as it was.
    pub fn check(&mut self, program: &Program) -> Result<Checked, Failure> {
        let mut infer = self.infer.clone();
        let (bindings, echoes) = infer.check_statements(program)?;
        let nodes: Vec<Node<Ty>> = std::mem::take(&mut infer.node_types)
            .into_iter()
            .map(|node| Node {
                ty: infer.resolve(&node.ty),
                address: node.address,
                pattern: node.pattern,
                start: node.start,
                end: node.end,
            })
            .collect();
        // A functor body is checked again at each application; the last
        // check of a node wins.
        let index = nodes
            .iter()
            .enumerate()
            .map(|(index, node)| ((node.address, node.pattern), index))
            .collect();
        let datatypes = infer
            .datatypes
            .iter()
            .map(|(stamp, info)| {
                let constructors = info
                    .constructors
                    .iter()
                    .map(|(name, scheme)| {
                        let carries = matches!(infer.prune(&scheme.ty), Type::Arrow(..));
                        (name.clone(), carries)
                    })
                    .collect();
                (*stamp, constructors)
            })
            .collect();
        let types = TypeTable {
            nodes,
            index,
            datatypes,
            structures: std::mem::take(&mut infer.structure_info),
            declaration_constructors: std::mem::take(&mut infer.declaration_constructors),
            exception_arguments: std::mem::take(&mut infer.exception_arguments),
        };
        self.infer = infer;
        Ok(Checked {
            bindings,
            types,
            echoes,
        })
    }
}

impl Infer {
    fn check_statements(&mut self, program: &Program) -> Result<(Vec<Binding>, Echoes), Failure> {
        let mut bindings = Vec::new();
        let mut echoes = Vec::new();
        for statement in &program.statements {
            self.tyvars.clear();
            let bound = match &statement.value {
                StmtKind::Val(name, expr) => self.infer_simple_val(name, expr)?,
                StmtKind::Print(expr) => {
                    let found = self.infer_expr(expr)?;
                    self.unify_at(&con("string"), &found, expr.source_span())?;
                    Vec::new()
                }
                StmtKind::Exit(expr) => {
                    self.infer_expr(expr)?;
                    Vec::new()
                }
                StmtKind::Declaration(declaration) => self.infer_decl(declaration)?,
            };
            self.check_flexible(statement.source_span(), true)?;
            self.default_overloads();
            self.instantiate_dummies(&bound);
            if let StmtKind::Declaration(decl) = &statement.value {
                echoes.extend(
                    self.declaration_echo(decl)
                        .into_iter()
                        .map(|echo| (bindings.len(), echo)),
                );
            }
            for (name, scheme) in bound {
                bindings.push(Binding {
                    ty: self.show_scheme(&scheme),
                    resolved: self.resolve(&scheme.ty),
                    name,
                });
            }
            if let StmtKind::Declaration(Decl {
                value: DeclKind::Open(paths),
                ..
            }) = &statement.value
            {
                for path in paths {
                    if let Some(env) = self.lookup_struct(path) {
                        echoes.extend(
                            self.structure_items(env)
                                .into_iter()
                                .filter(|(_, echo)| echo.starts_with("exception "))
                                .map(|(_, echo)| (bindings.len(), echo)),
                        );
                    }
                }
            }
        }
        Ok((bindings, echoes))
    }

    fn resolve(&self, ty: &Type) -> Ty {
        match self.prune(ty) {
            Type::Var(id) => Ty::Var {
                id,
                equality: self.vars[id].equality,
            },
            Type::Con(tycon, args) => Ty::Con {
                name: tycon.name,
                stamp: tycon.stamp,
                args: args.iter().map(|arg| self.resolve(arg)).collect(),
            },
            Type::Arrow(from, to) => {
                Ty::Arrow(Box::new(self.resolve(&from)), Box::new(self.resolve(&to)))
            }
            Type::Record(fields) => Ty::Record(
                fields
                    .iter()
                    .map(|(label, field)| (label.clone(), self.resolve(field)))
                    .collect(),
            ),
        }
    }
}

fn substitute(ty: &Ty, variables: &[usize], arguments: &[Ty]) -> Ty {
    match ty {
        Ty::Var { id, .. } => variables
            .iter()
            .position(|var| var == id)
            .and_then(|index| arguments.get(index))
            .cloned()
            .unwrap_or_else(|| ty.clone()),
        Ty::Con { name, stamp, args } => Ty::Con {
            name: name.clone(),
            stamp: *stamp,
            args: args
                .iter()
                .map(|ty| substitute(ty, variables, arguments))
                .collect(),
        },
        Ty::Arrow(from, to) => Ty::Arrow(
            Box::new(substitute(from, variables, arguments)),
            Box::new(substitute(to, variables, arguments)),
        ),
        Ty::Record(fields) => Ty::Record(
            fields
                .iter()
                .map(|(name, ty)| (name.clone(), substitute(ty, variables, arguments)))
                .collect(),
        ),
    }
}
