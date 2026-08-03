//! Nassau's core intermediate language, between the typed syntax tree and
//! Cranelift.
//!
//! Every value is one word (docs/value-representation.md). Functions are
//! closed: closure conversion has made each captured variable a field of the
//! function's environment, which is its first parameter. A function body is a
//! list of blocks in A-normal form: each statement names the result of one
//! operation on atoms, and each block ends in a terminator. Blocks take
//! parameters, so a join point after a conditional is a block whose parameter
//! is the conditional's value.

use std::fmt::{self, Write};

pub type Var = usize;
pub type BlockId = usize;
/// Functions and globals are numbered across a whole REPL session.
pub type FnId = usize;
pub type GlobalId = usize;

#[derive(Clone, Debug, PartialEq)]
pub enum Atom {
    Var(Var),
    /// An immediate value, already tagged.
    Word(i64),
    /// A boxed real in static data.
    Real(f64),
    /// A string block in static data.
    String(String),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Prim {
    IntAdd,
    IntSub,
    IntMul,
    /// `div` and `mod` take a third argument: the position to report `Div`
    /// at, as a string.
    IntDiv,
    IntMod,
    IntNeg,
    IntLt,
    IntLe,
    IntGt,
    IntGe,
    RealAdd,
    RealSub,
    RealMul,
    RealDiv,
    RealNeg,
    RealLt,
    RealLe,
    RealGt,
    RealGe,
    /// Whether two words are identical: equality on immediates.
    WordEq,
    WordNe,
    /// Structural equality on any equality type.
    Equal,
    Unequal,
    /// Whether a value is a pointer to a heap block rather than an immediate.
    IsBoxed,
    /// Allocates a reference cell holding its argument.
    Ref,
    /// `r := v`: stores into a reference cell.
    Assign,
    /// Prints a string.
    Print,
    /// `^`.
    Concat,
    IntToString,
    /// A string's length.
    Size,
    /// `Posix.Process.exit (Word8.fromInt n)`.
    Exit,
    /// The identity of a built-in exception, by its index in
    /// `value::BUILTIN_EXCEPTIONS`.
    BuiltinException,
}

impl Prim {
    pub fn name(self) -> &'static str {
        match self {
            Prim::IntAdd => "int.add",
            Prim::IntSub => "int.sub",
            Prim::IntMul => "int.mul",
            Prim::IntDiv => "int.div",
            Prim::IntMod => "int.mod",
            Prim::IntNeg => "int.neg",
            Prim::IntLt => "int.lt",
            Prim::IntLe => "int.le",
            Prim::IntGt => "int.gt",
            Prim::IntGe => "int.ge",
            Prim::RealAdd => "real.add",
            Prim::RealSub => "real.sub",
            Prim::RealMul => "real.mul",
            Prim::RealDiv => "real.div",
            Prim::RealNeg => "real.neg",
            Prim::RealLt => "real.lt",
            Prim::RealLe => "real.le",
            Prim::RealGt => "real.gt",
            Prim::RealGe => "real.ge",
            Prim::WordEq => "word.eq",
            Prim::WordNe => "word.ne",
            Prim::Equal => "equal",
            Prim::Unequal => "unequal",
            Prim::IsBoxed => "is_boxed",
            Prim::Ref => "ref",
            Prim::Assign => "assign",
            Prim::Print => "print",
            Prim::Concat => "concat",
            Prim::IntToString => "int.to_string",
            Prim::Size => "size",
            Prim::Exit => "exit",
            Prim::BuiltinException => "exception.builtin",
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Callee {
    /// A closure whose code is unknown: field 0 holds it.
    Closure(Atom),
    /// A known function, given the environment it expects.
    Known(FnId, Atom),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Op {
    Atom(Atom),
    Prim(Prim, Vec<Atom>),
    /// Allocates a record block.
    Record(Vec<Atom>),
    /// Field `index` of a heap block.
    Select(Atom, usize),
    Global(GlobalId),
    Call(Callee, Vec<Atom>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Stmt {
    Let(Var, Op),
    SetGlobal(GlobalId, Atom),
    /// Allocates closures that may refer to each other: every closure is
    /// allocated before any field is filled in.
    Closures(Vec<Closure>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Closure {
    pub var: Var,
    pub code: FnId,
    pub captured: Vec<Atom>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Failure {
    Match,
    Bind,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Term {
    Return(Atom),
    Jump(BlockId, Vec<Atom>),
    /// Branches on a `bool`.
    If(Atom, BlockId, BlockId),
    TailCall(Callee, Vec<Atom>),
    /// Raises `Match` or `Bind`, reported at the given position.
    Fail(Failure, String),
    /// Raises an exception value. A raise names the position SML/NJ reports
    /// it at; a handler passing on an exception it does not match keeps the
    /// position it was first raised at.
    Raise(Atom, Option<String>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Block {
    pub params: Vec<Var>,
    pub stmts: Vec<Stmt>,
    pub term: Term,
    /// The block an exception raised here goes to, taking the exception as
    /// its parameter; with none, it leaves the function.
    pub handler: Option<BlockId>,
}

#[derive(Clone, Debug)]
pub struct Function {
    pub id: FnId,
    pub name: String,
    /// The environment comes first, except in a chunk's entry function.
    pub params: Vec<Var>,
    /// The source name of each variable, for dumps.
    pub vars: Vec<String>,
    /// Block 0 is the entry.
    pub blocks: Vec<Block>,
}

/// The code of one program or REPL chunk.
#[derive(Clone, Debug)]
pub struct Module {
    pub functions: Vec<Function>,
    /// Globals first defined by this module.
    pub globals: Vec<(GlobalId, String)>,
    /// The top-level code, run once; it returns the exit status.
    pub entry: Function,
    /// The source file's name, for reporting uncaught exceptions.
    pub file: String,
}

impl fmt::Display for Atom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Atom::Var(var) => write!(f, "v{var}"),
            // An immediate shows the integer it tags.
            Atom::Word(word) if *word >> 1 < 0 => write!(f, "#~{}", -(word >> 1)),
            Atom::Word(word) => write!(f, "#{}", word >> 1),
            Atom::Real(value) => write!(f, "{value:?}"),
            Atom::String(value) => write!(f, "{value:?}"),
        }
    }
}

fn atoms(atoms: &[Atom]) -> String {
    atoms
        .iter()
        .map(|atom| atom.to_string())
        .collect::<Vec<_>>()
        .join(", ")
}

impl fmt::Display for Callee {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Callee::Closure(closure) => write!(f, "{closure}"),
            Callee::Known(function, env) => write!(f, "f{function}[{env}]"),
        }
    }
}

impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Op::Atom(atom) => write!(f, "{atom}"),
            Op::Prim(prim, args) => write!(f, "{}({})", prim.name(), atoms(args)),
            Op::Record(fields) => write!(f, "record({})", atoms(fields)),
            Op::Select(block, index) => write!(f, "{block}.{index}"),
            Op::Global(global) => write!(f, "g{global}"),
            Op::Call(callee, args) => write!(f, "call {callee}({})", atoms(args)),
        }
    }
}

impl Function {
    fn var(&self, var: Var) -> String {
        match self.vars.get(var).map(String::as_str) {
            Some("") | None => format!("v{var}"),
            Some(name) => format!("v{var}:{name}"),
        }
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let params: Vec<String> = self.params.iter().map(|var| self.var(*var)).collect();
        writeln!(f, "fn f{} {}({}) {{", self.id, self.name, params.join(", "))?;
        for (index, block) in self.blocks.iter().enumerate() {
            let params: Vec<String> = block.params.iter().map(|var| self.var(*var)).collect();
            match block.handler {
                Some(handler) => {
                    writeln!(f, "  b{index}({}) handle b{handler}:", params.join(", "))?
                }
                None => writeln!(f, "  b{index}({}):", params.join(", "))?,
            }
            for stmt in &block.stmts {
                let mut line = String::new();
                match stmt {
                    Stmt::Let(var, op) => write!(line, "{} = {op}", self.var(*var))?,
                    Stmt::SetGlobal(global, atom) => write!(line, "g{global} := {atom}")?,
                    Stmt::Closures(closures) => {
                        let items: Vec<String> = closures
                            .iter()
                            .map(|closure| {
                                format!(
                                    "{} = closure f{}({})",
                                    self.var(closure.var),
                                    closure.code,
                                    atoms(&closure.captured)
                                )
                            })
                            .collect();
                        write!(line, "{}", items.join("; "))?;
                    }
                }
                writeln!(f, "    {line}")?;
            }
            let term = match &block.term {
                Term::Return(atom) => format!("return {atom}"),
                Term::Jump(target, args) => format!("jump b{target}({})", atoms(args)),
                Term::If(condition, then, otherwise) => {
                    format!("if {condition} then b{then} else b{otherwise}")
                }
                Term::TailCall(callee, args) => format!("tailcall {callee}({})", atoms(args)),
                Term::Fail(Failure::Match, location) => format!("fail Match at {location}"),
                Term::Fail(Failure::Bind, location) => format!("fail Bind at {location}"),
                Term::Raise(exception, Some(location)) => {
                    format!("raise {exception} at {location}")
                }
                Term::Raise(exception, None) => format!("reraise {exception}"),
            };
            writeln!(f, "    {term}")?;
        }
        writeln!(f, "}}")
    }
}

impl fmt::Display for Module {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (global, name) in &self.globals {
            writeln!(f, "global g{global} {name}")?;
        }
        for function in &self.functions {
            write!(f, "{function}")?;
        }
        write!(f, "{}", self.entry)
    }
}
