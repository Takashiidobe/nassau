//! Lowers a type-checked program to the core IR.

use miette::SourceSpan;

use crate::core::{
    Atom, Block, BlockId, FnId, Function, GlobalId, Module, Op, Prim, Stmt, Term, Var,
};
use crate::error::ThisError;
use crate::infer::{Ty, TypeTable};
use crate::parser::{Expr, ExprKind, Program, StmtKind};
use crate::value;

#[derive(Debug, ThisError)]
pub enum LowerError {
    #[error("{0} are not supported by code generation yet")]
    Unsupported(String),
}

pub type Failure = (LowerError, SourceSpan);
type Res<T> = Result<T, Failure>;

/// What lowering keeps from one program to the next: the REPL lowers each
/// chunk in the environment of the earlier ones.
#[derive(Clone, Default)]
pub struct Session {
    next_function: FnId,
    next_global: GlobalId,
    /// Top-level names, latest last.
    globals: Vec<(String, GlobalId)>,
}

impl Session {
    pub fn new() -> Self {
        Self::default()
    }

    /// The global that holds the top-level binding `name`.
    pub fn global(&self, name: &str) -> Option<GlobalId> {
        self.globals
            .iter()
            .rev()
            .find(|(known, _)| known == name)
            .map(|(_, global)| *global)
    }

    /// Lowers `program`, whose top-level code becomes the function `entry`.
    pub fn lower(&mut self, program: &Program, types: &TypeTable, entry: &str) -> Res<Module> {
        let mut session = self.clone();
        let id = session.next_function;
        session.next_function += 1;
        let mut lowerer = Lowerer {
            session: &mut session,
            types,
            new_globals: Vec::new(),
            function: Builder::new(id, entry),
        };
        for statement in &program.statements {
            lowerer.statement(&statement.value)?;
        }
        let status = value::tagged(i64::from(program.result));
        lowerer.function.terminate(Term::Return(Atom::Word(status)));
        let module = Module {
            functions: Vec::new(),
            globals: lowerer.new_globals,
            entry: lowerer.function.finish(),
        };
        *self = session;
        Ok(module)
    }
}

/// A function whose blocks are being built.
struct Builder {
    id: FnId,
    name: String,
    params: Vec<Var>,
    vars: Vec<String>,
    blocks: Vec<(Vec<Var>, Vec<Stmt>, Option<Term>)>,
    current: BlockId,
}

impl Builder {
    fn new(id: FnId, name: &str) -> Self {
        Self {
            id,
            name: name.to_string(),
            params: Vec::new(),
            vars: Vec::new(),
            blocks: vec![(Vec::new(), Vec::new(), None)],
            current: 0,
        }
    }

    fn var(&mut self, name: &str) -> Var {
        self.vars.push(name.to_string());
        self.vars.len() - 1
    }

    fn block(&mut self, params: Vec<Var>) -> BlockId {
        self.blocks.push((params, Vec::new(), None));
        self.blocks.len() - 1
    }

    fn emit(&mut self, stmt: Stmt) {
        self.blocks[self.current].1.push(stmt);
    }

    /// Names the result of `op`.
    fn bind(&mut self, name: &str, op: Op) -> Atom {
        let var = self.var(name);
        self.emit(Stmt::Let(var, op));
        Atom::Var(var)
    }

    fn terminate(&mut self, term: Term) {
        let slot = &mut self.blocks[self.current].2;
        debug_assert!(slot.is_none(), "block terminated twice");
        *slot = Some(term);
    }

    fn finish(self) -> Function {
        Function {
            id: self.id,
            name: self.name,
            params: self.params,
            vars: self.vars,
            blocks: self
                .blocks
                .into_iter()
                .map(|(params, stmts, term)| Block {
                    params,
                    stmts,
                    term: term.expect("every block is terminated"),
                })
                .collect(),
        }
    }
}

struct Lowerer<'a> {
    session: &'a mut Session,
    types: &'a TypeTable,
    new_globals: Vec<(GlobalId, String)>,
    function: Builder,
}

impl Lowerer<'_> {
    fn statement(&mut self, statement: &StmtKind) -> Res<()> {
        match statement {
            StmtKind::Val(name, expr) => {
                let value = self.value(expr)?;
                let global = self.session.next_global;
                self.session.next_global += 1;
                self.new_globals.push((global, name.clone()));
                self.function.emit(Stmt::SetGlobal(global, value));
                self.session.globals.push((name.clone(), global));
            }
            StmtKind::Print(expr) => {
                let text = self.value(expr)?;
                self.function.bind("", Op::Prim(Prim::Print, vec![text]));
            }
            StmtKind::Exit(expr) => {
                // The parser only makes exit statements of this shape.
                let ExprKind::PosixExit(word8) = &expr.value else {
                    unreachable!("exit statement without Posix.Process.exit")
                };
                let ExprKind::Word8FromInt(status) = &word8.value else {
                    unreachable!("exit statement without Word8.fromInt")
                };
                let status = self.value(status)?;
                self.function.bind("", Op::Prim(Prim::Exit, vec![status]));
            }
            StmtKind::Declaration(declaration) => {
                return Err((
                    LowerError::Unsupported("declarations".into()),
                    declaration.source_span(),
                ));
            }
        }
        Ok(())
    }

    fn ty(&self, expr: &Expr) -> Option<&Ty> {
        self.types.expr(expr)
    }

    fn is_real(&self, expr: &Expr) -> bool {
        self.ty(expr).is_some_and(|ty| ty.is("real"))
    }

    /// Lowers `expr` in the current block and returns its value.
    fn value(&mut self, expr: &Expr) -> Res<Atom> {
        Ok(match &expr.value {
            ExprKind::Integer(value) => Atom::Word(value::tagged(*value)),
            ExprKind::Real(value) => Atom::Real(*value),
            ExprKind::Boolean(value) => Atom::Word(value::tagged(i64::from(*value))),
            ExprKind::String(value) => Atom::String(value.clone()),
            ExprKind::Variable(name) => match self.session.global(name) {
                Some(global) => self.function.bind(name, Op::Global(global)),
                None => return Err(unsupported(&format!("the name {name}"), expr)),
            },
            ExprKind::List(items) => {
                let items = items
                    .iter()
                    .map(|item| self.value(item))
                    .collect::<Res<Vec<_>>>()?;
                let mut list = Atom::Word(value::NIL);
                for item in items.into_iter().rev() {
                    list = self.function.bind("", Op::Record(vec![item, list]));
                }
                list
            }
            ExprKind::If(condition, consequent, alternative) => {
                let condition = self.value(condition)?;
                let result = self.function.var("");
                let then = self.function.block(Vec::new());
                let otherwise = self.function.block(Vec::new());
                let join = self.function.block(vec![result]);
                self.function
                    .terminate(Term::If(condition, then, otherwise));
                for (block, branch) in [(then, consequent), (otherwise, alternative)] {
                    self.function.current = block;
                    let value = self.value(branch)?;
                    self.function.terminate(Term::Jump(join, vec![value]));
                }
                self.function.current = join;
                Atom::Var(result)
            }
            ExprKind::Add(lhs, rhs)
            | ExprKind::Subtract(lhs, rhs)
            | ExprKind::Multiply(lhs, rhs)
            | ExprKind::Divide(lhs, rhs)
            | ExprKind::IntDivide(lhs, rhs)
            | ExprKind::Greater(lhs, rhs)
            | ExprKind::GreaterEqual(lhs, rhs)
            | ExprKind::Less(lhs, rhs)
            | ExprKind::LessEqual(lhs, rhs) => {
                let real = self.is_real(lhs);
                let prim = match (&expr.value, real) {
                    (ExprKind::Add(..), false) => Prim::IntAdd,
                    (ExprKind::Subtract(..), false) => Prim::IntSub,
                    (ExprKind::Multiply(..), false) => Prim::IntMul,
                    (ExprKind::IntDivide(..), _) => Prim::IntDiv,
                    (ExprKind::Greater(..), false) => Prim::IntGt,
                    (ExprKind::GreaterEqual(..), false) => Prim::IntGe,
                    (ExprKind::Less(..), false) => Prim::IntLt,
                    (ExprKind::LessEqual(..), false) => Prim::IntLe,
                    (ExprKind::Add(..), true) => Prim::RealAdd,
                    (ExprKind::Subtract(..), true) => Prim::RealSub,
                    (ExprKind::Multiply(..), true) => Prim::RealMul,
                    (ExprKind::Divide(..), _) => Prim::RealDiv,
                    (ExprKind::Greater(..), true) => Prim::RealGt,
                    (ExprKind::GreaterEqual(..), true) => Prim::RealGe,
                    (ExprKind::Less(..), true) => Prim::RealLt,
                    (ExprKind::LessEqual(..), true) => Prim::RealLe,
                    _ => unreachable!(),
                };
                let lhs = self.value(lhs)?;
                let rhs = self.value(rhs)?;
                self.function.bind("", Op::Prim(prim, vec![lhs, rhs]))
            }
            ExprKind::Equal(lhs, rhs) | ExprKind::NotEqual(lhs, rhs) => {
                let immediate = self
                    .ty(lhs)
                    .is_some_and(|ty| ty.is("int") || ty.is("bool") || ty.is("char"));
                if !immediate {
                    return Err(unsupported("equality on this type", expr));
                }
                let prim = if matches!(expr.value, ExprKind::Equal(..)) {
                    Prim::WordEq
                } else {
                    Prim::WordNe
                };
                let lhs = self.value(lhs)?;
                let rhs = self.value(rhs)?;
                self.function.bind("", Op::Prim(prim, vec![lhs, rhs]))
            }
            _ => return Err(unsupported("these expressions", expr)),
        })
    }
}

fn unsupported(what: &str, expr: &Expr) -> Failure {
    (
        LowerError::Unsupported(what.to_string()),
        expr.source_span(),
    )
}
