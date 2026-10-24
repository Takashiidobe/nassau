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

/// The SML type of an external function's parameter or result. `Real` is
/// passed unboxed as an `f64`; everything else is a 64-bit word.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExternTy {
    Int,
    String,
    Real,
    Unit,
}

/// A C-ABI function exposed to the Basis as `Prim.<sml>`: libm, or an entry
/// point of the runtime. `sqrt` is compiled inline rather than called.
pub struct ExternFn {
    pub sml: &'static str,
    pub symbol: &'static str,
    pub params: &'static [ExternTy],
    pub result: ExternTy,
}

const fn external(
    sml: &'static str,
    symbol: &'static str,
    params: &'static [ExternTy],
    result: ExternTy,
) -> ExternFn {
    ExternFn {
        sml,
        symbol,
        params,
        result,
    }
}

const REAL: ExternTy = ExternTy::Real;
const INT: ExternTy = ExternTy::Int;
const STRING: ExternTy = ExternTy::String;

pub const EXTERN_FNS: &[ExternFn] = &[
    external("Prim.mathSqrt", "sqrt", &[REAL], REAL),
    external("Prim.mathSin", "nassau_math_sin", &[REAL], REAL),
    external("Prim.mathCos", "nassau_math_cos", &[REAL], REAL),
    external("Prim.mathTan", "nassau_math_tan", &[REAL], REAL),
    external("Prim.mathAsin", "nassau_math_asin", &[REAL], REAL),
    external("Prim.mathAcos", "nassau_math_acos", &[REAL], REAL),
    external("Prim.mathAtan", "nassau_math_atan", &[REAL], REAL),
    external("Prim.mathAtan2", "nassau_math_atan2", &[REAL, REAL], REAL),
    external("Prim.mathExp", "nassau_math_exp", &[REAL], REAL),
    external("Prim.mathPow", "nassau_math_pow", &[REAL, REAL], REAL),
    external("Prim.mathLn", "nassau_math_log", &[REAL], REAL),
    external("Prim.mathLog10", "nassau_math_log10", &[REAL], REAL),
    external("Prim.mathSinh", "nassau_math_sinh", &[REAL], REAL),
    external("Prim.mathCosh", "nassau_math_cosh", &[REAL], REAL),
    external("Prim.mathTanh", "nassau_math_tanh", &[REAL], REAL),
    external("Prim.osOpen", "nassau_os_open", &[STRING, INT], INT),
    external("Prim.osRead", "nassau_os_read", &[INT, INT], STRING),
    external("Prim.osWrite", "nassau_os_write", &[INT, STRING], INT),
    external("Prim.osClose", "nassau_os_close", &[INT], INT),
    external("Prim.osRemove", "nassau_os_remove", &[STRING], INT),
    external(
        "Prim.osErrorMessage",
        "nassau_os_error_message",
        &[INT],
        STRING,
    ),
    external("Prim.osOpenDir", "nassau_os_open_dir", &[STRING], INT),
    external("Prim.osReadDir", "nassau_os_read_dir", &[INT], STRING),
    external("Prim.osRewindDir", "nassau_os_rewind_dir", &[INT], INT),
    external("Prim.osIsDir", "nassau_os_is_dir", &[STRING], INT),
    external("Prim.osFileSize", "nassau_os_file_size", &[STRING], INT),
    external("Prim.osMkDir", "nassau_os_mkdir", &[STRING], INT),
    external("Prim.osRmDir", "nassau_os_rmdir", &[STRING], INT),
    external("Prim.osGetDir", "nassau_os_get_dir", &[], STRING),
    external("Prim.osChDir", "nassau_os_chdir", &[STRING], INT),
    external("Prim.osAccess", "nassau_os_access", &[STRING, INT], INT),
    external("Prim.osRename", "nassau_os_rename", &[STRING, STRING], INT),
    external("Prim.osModTime", "nassau_os_mod_time", &[STRING], INT),
    external(
        "Prim.osSetTime",
        "nassau_os_set_time",
        &[STRING, INT, INT],
        INT,
    ),
    external("Prim.osIsLink", "nassau_os_is_link", &[STRING], INT),
    external("Prim.osReadLink", "nassau_os_read_link", &[STRING], STRING),
    external("Prim.osFullPath", "nassau_os_full_path", &[STRING], STRING),
    external("Prim.osFileId", "nassau_os_file_id", &[STRING], STRING),
    external("Prim.osTmpName", "nassau_os_tmp_name", &[], STRING),
    external("Prim.osSystem", "nassau_os_system", &[STRING], INT),
    external("Prim.osSleep", "nassau_os_sleep", &[REAL], ExternTy::Unit),
    external("Prim.osNow", "nassau_os_now", &[], REAL),
    external("Prim.osGetenv", "nassau_os_getenv", &[STRING], STRING),
    external("Prim.timeZone", "nassau_time_zone", &[REAL], INT),
    external("Prim.fpGetRound", "nassau_fp_get_round", &[], INT),
    external("Prim.fpSetRound", "nassau_fp_set_round", &[INT], INT),
    external("Prim.sockCreate", "nassau_sock_create", &[], INT),
    external("Prim.sockPair", "nassau_sock_pair", &[], INT),
    external("Prim.sockBind", "nassau_sock_bind", &[INT, STRING], INT),
    external("Prim.sockListen", "nassau_sock_listen", &[INT], INT),
    external("Prim.sockAccept", "nassau_sock_accept", &[INT], INT),
    external(
        "Prim.sockConnect",
        "nassau_sock_connect",
        &[INT, STRING],
        INT,
    ),
    external("Prim.sockName", "nassau_sock_name", &[INT, INT], STRING),
    external(
        "Prim.sockShutdown",
        "nassau_sock_shutdown",
        &[INT, INT],
        INT,
    ),
    external("Prim.osArgs", "nassau_os_args", &[], STRING),
];

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
    /// Unsigned and wrapping; `WordDiv` and `WordMod` take the same position
    /// argument as `IntDiv`.
    WordAdd,
    WordSub,
    WordMul,
    WordAnd,
    WordOr,
    WordXor,
    /// Shifts take a word count; counts past the width give zero (or the
    /// sign, for the arithmetic shift).
    WordShl,
    WordLShr,
    WordAShr,
    WordDiv,
    WordMod,
    WordLt,
    WordLe,
    WordGt,
    WordGe,
    IntLt,
    IntLe,
    IntGt,
    IntGe,
    StringLt,
    StringLe,
    StringGt,
    StringGe,
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
    ArrayNew,
    ArrayLength,
    ArraySub,
    ArrayUpdate,
    ArrayVector,
    VectorLength,
    VectorSub,
    /// Prints a string.
    Print,
    /// `^`.
    Concat,
    IntToString,
    /// An int as a real.
    IntToReal,
    /// The floor of a real known to fit an int.
    RealFloor,
    /// A call to a function from `EXTERN_FNS`, by index.
    Extern(usize),
    /// The one-character string of a char.
    CharToString,
    /// The char at an index known to be in range of a string.
    StringSub,
    /// A string's length.
    Size,
    /// Ends the program with the status given.
    Exit,
    Terminate,
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
            Prim::WordAdd => "word.add",
            Prim::WordSub => "word.sub",
            Prim::WordMul => "word.mul",
            Prim::WordAnd => "word.and",
            Prim::WordOr => "word.or",
            Prim::WordXor => "word.xor",
            Prim::WordShl => "word.shl",
            Prim::WordLShr => "word.lshr",
            Prim::WordAShr => "word.ashr",
            Prim::WordDiv => "word.div",
            Prim::WordMod => "word.mod",
            Prim::WordLt => "word.lt",
            Prim::WordLe => "word.le",
            Prim::WordGt => "word.gt",
            Prim::WordGe => "word.ge",
            Prim::IntLt => "int.lt",
            Prim::IntLe => "int.le",
            Prim::IntGt => "int.gt",
            Prim::IntGe => "int.ge",
            Prim::StringLt => "string.lt",
            Prim::StringLe => "string.le",
            Prim::StringGt => "string.gt",
            Prim::StringGe => "string.ge",
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
            Prim::ArrayNew => "array.new",
            Prim::ArrayLength => "array.length",
            Prim::ArraySub => "array.sub",
            Prim::ArrayUpdate => "array.update",
            Prim::ArrayVector => "array.vector",
            Prim::VectorLength => "vector.length",
            Prim::VectorSub => "vector.sub",
            Prim::Print => "print",
            Prim::Concat => "concat",
            Prim::IntToString => "int.to_string",
            Prim::IntToReal => "int.to_real",
            Prim::RealFloor => "real.floor",
            Prim::Extern(index) => EXTERN_FNS[index].symbol,
            Prim::CharToString => "char.to_string",
            Prim::StringSub => "string.sub",
            Prim::Size => "size",
            Prim::Exit => "exit",
            Prim::Terminate => "terminate",
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
