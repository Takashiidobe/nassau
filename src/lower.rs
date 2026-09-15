//! Lowers a type-checked program to the core IR.
//!
//! Each SML function becomes an IR function whose first parameter is its
//! closure. A function's free variables are found as its body is lowered:
//! the first use of a variable bound by an enclosing function adds it to the
//! closure and loads it from the environment on entry. Top-level bindings
//! live in globals instead, so they are never captured.

use miette::SourceSpan;
use std::rc::Rc;

use crate::core::{
    Atom, Block, BlockId, Callee, Closure, FnId, Function, GlobalId, Module, Op, Prim, Stmt, Term,
    Var,
};
use crate::error::ThisError;
use crate::infer::{Ty, TypeTable};
use crate::parser::{
    Decl, DeclKind, ExceptionKind, Expr, ExprKind, FunctorParameter, Pat, PatKind, Program,
    StmtKind, StrExp, StrExpKind,
};
use crate::value;

mod decision;
mod functor_roots;

use decision::Test;

#[derive(Debug, ThisError)]
pub enum LowerError {
    #[error("{0} are not supported by code generation yet")]
    Unsupported(String),
}

pub type Failure = (LowerError, SourceSpan);
type Res<T> = Result<T, Failure>;

/// A function whose code is known where it is called: calls with at least
/// `arity` arguments go straight to `worker`, which takes the closure and
/// then `arity` arguments.
#[derive(Clone, Copy, Debug)]
struct Known {
    worker: FnId,
    arity: usize,
}

/// What a name stands for.
#[derive(Clone, Debug)]
enum Binding {
    Constructor(Option<(Test, bool)>),
    Global(GlobalId, Option<Known>),
    /// A variable of the function at `depth` in the stack being lowered.
    Local {
        depth: usize,
        var: Var,
        known: Option<Known>,
    },
}

#[derive(Clone, Default)]
struct Structure {
    values: Vec<(String, Binding)>,
    structures: Vec<(String, Structure)>,
}

#[derive(Clone)]
struct Functor {
    parameter: Option<String>,
    roots: std::collections::BTreeSet<GlobalId>,
    environment: Environment,
    source: Source,
}

#[derive(Clone)]
struct Environment {
    globals: Vec<(String, Binding)>,
    locals: Vec<(String, Binding)>,
    structures: Vec<(String, Structure)>,
    functors: Vec<(String, Rc<Functor>)>,
}

/// What lowering keeps from one program to the next: the REPL lowers each
/// chunk in the environment of the earlier ones.
#[derive(Clone, Default)]
pub struct Session {
    next_function: FnId,
    printing: bool,
    next_global: GlobalId,
    /// Top-level names, latest last.
    globals: Vec<(String, Binding)>,
    structures: Vec<(String, Structure)>,
    functors: Vec<(String, Rc<Functor>)>,
}

fn visible<T>(values: &[(String, T)]) -> impl Iterator<Item = &T> {
    let mut names = std::collections::BTreeSet::new();
    values
        .iter()
        .rev()
        .filter_map(move |(name, value)| names.insert(name).then_some(value))
}

fn environment_roots(
    globals: &[(String, Binding)],
    structures: &[(String, Structure)],
    functors: &[(String, Rc<Functor>)],
    roots: &mut std::collections::BTreeSet<GlobalId>,
) {
    for binding in visible(globals) {
        if let Binding::Global(global, _) = binding {
            roots.insert(*global);
        }
    }
    for structure in visible(structures) {
        environment_roots(&structure.values, &structure.structures, &[], roots);
    }
    for functor in visible(functors) {
        roots.extend(&functor.roots);
    }
}

impl Session {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn new_repl() -> Self {
        Self {
            printing: true,
            ..Self::default()
        }
    }

    pub fn root_globals(&self) -> std::collections::BTreeSet<GlobalId> {
        let mut roots = std::collections::BTreeSet::new();
        environment_roots(&self.globals, &self.structures, &self.functors, &mut roots);
        roots
    }

    /// The global that holds the top-level binding `name`.
    pub fn global(&self, name: &str) -> Option<GlobalId> {
        match self.lookup(name)? {
            Binding::Global(global, _) => Some(global),
            Binding::Local { .. } | Binding::Constructor(_) => None,
        }
    }

    fn lookup(&self, name: &str) -> Option<Binding> {
        self.globals
            .iter()
            .rev()
            .find(|(known, _)| known == name)
            .map(|(_, binding)| binding.clone())
    }

    /// Drops the top-level names bound since `earlier`, keeping the
    /// functions and globals numbered since, which are already compiled.
    pub fn forget_bindings(&mut self, earlier: &Session) {
        self.globals.clone_from(&earlier.globals);
        self.structures.clone_from(&earlier.structures);
        self.functors.clone_from(&earlier.functors);
    }

    fn function_id(&mut self) -> FnId {
        self.next_function += 1;
        self.next_function - 1
    }

    /// Lowers `program`, read from `source`; its top-level code becomes the
    /// function `entry`.
    pub fn lower(
        &mut self,
        program: &Program,
        types: &TypeTable,
        source: &Source,
        entry: &str,
    ) -> Res<Module> {
        self.lower_parts(
            &[Part {
                program,
                types,
                source,
            }],
            entry,
        )
    }

    /// Lowers `parts` in order into one module whose entry runs them all.
    pub fn lower_parts(&mut self, parts: &[Part], entry: &str) -> Res<Module> {
        let last = parts.last().expect("at least one part");
        let mut session = self.clone();
        let id = session.function_id();
        let mut lowerer = Lowerer {
            session: &mut session,
            types: last.types,
            source: last.source.clone(),
            new_globals: Vec::new(),
            functions: Vec::new(),
            frames: vec![Builder::new(id, entry, Vec::new())],
            scope: Vec::new(),
        };
        for part in parts {
            lowerer.types = part.types;
            lowerer.source = part.source.clone();
            for statement in &part.program.statements {
                lowerer.statement(&statement.value)?;
            }
        }
        let status = value::tagged(i64::from(last.program.result));
        lowerer.terminate(Term::Return(Atom::Word(status)));
        let entry = lowerer.frames.pop().expect("the entry frame").finish();
        let module = Module {
            functions: lowerer.functions,
            globals: lowerer.new_globals,
            entry,
            file: last.source.file.clone(),
        };
        *self = session;
        Ok(module)
    }
}

/// One program lowered into a shared module, with the types and source it
/// was checked and read against.
pub struct Part<'a> {
    pub program: &'a Program,
    pub types: &'a TypeTable,
    pub source: &'a Source,
}

/// The program's source, for the positions SML/NJ reports exceptions at.
#[derive(Clone)]
pub struct Source {
    /// The file name as SML/NJ shows it: without directories.
    pub file: String,
    pub text: String,
    /// The line `text` starts on: the REPL numbers lines across its whole
    /// input.
    pub first_line: usize,
}

impl Source {
    /// SML/NJ's `line.column` of the byte at `offset`, both counted from 1.
    /// SML/NJ counts the first line's columns from 2.
    fn position(&self, offset: usize) -> String {
        let before = &self.text[..offset.min(self.text.len())];
        let line = before.matches('\n').count() + self.first_line;
        let mut column = before.len() - before.rfind('\n').map_or(0, |newline| newline + 1) + 1;
        if line == 1 {
            column += 1;
        }
        format!("{line}.{column}")
    }

    /// The bytes from `start` to `end` as SML/NJ names them: a single
    /// character by its own position.
    fn span(&self, start: usize, end: usize) -> String {
        if end <= start + 1 {
            format!("{}:{}", self.file, self.position(start))
        } else {
            format!(
                "{}:{}-{}",
                self.file,
                self.position(start),
                self.position(end)
            )
        }
    }

    /// Where SML/NJ reports an exception raised by the infix operator `name`
    /// between `lhs` and `rhs`: the operator's own span.
    fn operator(&self, name: &str, lhs: &Expr, rhs: &Expr) -> String {
        let between = &self.text[lhs.end.offset..rhs.start.offset];
        let start = lhs.end.offset + between.find(name).unwrap_or(0);
        self.span(start, start + name.len())
    }

    /// Where SML/NJ reports `raise argument`: the argument's span.
    fn raised(&self, argument: &Expr) -> String {
        self.span(argument.start.offset, argument.end.offset)
    }

    /// Where a failed match is reported: the end of `expr`.
    fn end(&self, expr: &Expr) -> String {
        format!("{}:{}", self.file, self.position(expr.end.offset))
    }
}

/// A block's parameters, statements, terminator once it has one, and
/// handler.
type PartialBlock = (Vec<Var>, Vec<Stmt>, Option<Term>, Option<BlockId>);

/// A function whose blocks are being built.
struct Builder {
    id: FnId,
    name: String,
    params: Vec<Var>,
    vars: Vec<String>,
    blocks: Vec<PartialBlock>,
    current: BlockId,
    /// The handlers of the `handle` expressions being lowered, innermost
    /// last; new blocks raise to the innermost.
    handlers: Vec<BlockId>,
    /// Loads of captured variables from the environment, run on entry.
    prologue: Vec<Stmt>,
    /// The enclosing functions' variables this one uses, by where they are
    /// bound, with the variable each is loaded into.
    captures: Vec<((usize, Var), Var)>,
}

impl Builder {
    /// A function whose parameters are named `params`; the first is the
    /// environment, except in a chunk's entry.
    fn new(id: FnId, name: &str, params: Vec<&str>) -> Self {
        let mut builder = Self {
            id,
            name: name.to_string(),
            params: Vec::new(),
            vars: Vec::new(),
            blocks: vec![(Vec::new(), Vec::new(), None, None)],
            current: 0,
            handlers: Vec::new(),
            prologue: Vec::new(),
            captures: Vec::new(),
        };
        for param in params {
            let var = builder.var(param);
            builder.params.push(var);
        }
        builder
    }

    fn var(&mut self, name: &str) -> Var {
        self.vars.push(name.to_string());
        self.vars.len() - 1
    }

    fn block(&mut self, params: Vec<Var>) -> BlockId {
        let handler = self.handlers.last().copied();
        self.blocks.push((params, Vec::new(), None, handler));
        self.blocks.len() - 1
    }

    fn finish(mut self) -> Function {
        let prologue = std::mem::take(&mut self.prologue);
        self.blocks[0].1.splice(0..0, prologue);
        Function {
            id: self.id,
            name: self.name,
            params: self.params,
            vars: self.vars,
            blocks: self
                .blocks
                .into_iter()
                .map(|(params, stmts, term, handler)| Block {
                    params,
                    stmts,
                    term: term.expect("every block is terminated"),
                    handler,
                })
                .collect(),
        }
    }
}

/// Where the value of an expression goes.
#[derive(Clone, Copy)]
enum Dest {
    /// Returned from the current function.
    Return,
    /// Passed to a join block.
    Jump(BlockId),
}

/// A function about to be compiled: `fun` clauses, or `fn` rules with one
/// parameter each.
struct Definition<'a> {
    name: &'a str,
    clauses: Vec<(Vec<&'a Pat>, &'a Expr)>,
    /// Where a match failure in it is reported.
    location: String,
}

/// An application lowered up to its last call.
enum Applied {
    Value(Atom),
    Call(Callee, Vec<Atom>),
}

/// Built-in functions that are compiled inline when applied.
#[derive(Clone, Copy)]
enum Builtin {
    Print,
    IntToString,
    Size,
    Not,
    Negate,
    NegateReal,
    Ref,
    Deref,
    Equal,
    Unequal,
    Assign,
}

fn builtin(name: &str) -> Option<Builtin> {
    Some(match name {
        "print" => Builtin::Print,
        "Int.toString" => Builtin::IntToString,
        "size" => Builtin::Size,
        "not" => Builtin::Not,
        "~" => Builtin::Negate,
        "ref" => Builtin::Ref,
        "!" => Builtin::Deref,
        "=" => Builtin::Equal,
        "<>" => Builtin::Unequal,
        ":=" => Builtin::Assign,
        _ => return None,
    })
}

struct Lowerer<'a> {
    session: &'a mut Session,
    types: &'a TypeTable,
    source: Source,
    new_globals: Vec<(GlobalId, String)>,
    /// Finished functions.
    functions: Vec<Function>,
    /// The functions being lowered, innermost last; the chunk's entry first.
    frames: Vec<Builder>,
    /// Names bound inside the functions being lowered, latest last.
    scope: Vec<(String, Binding)>,
}

impl Lowerer<'_> {
    fn frame(&mut self) -> &mut Builder {
        self.frames.last_mut().expect("a function is being lowered")
    }

    fn depth(&self) -> usize {
        self.frames.len() - 1
    }

    fn emit(&mut self, stmt: Stmt) {
        let frame = self.frame();
        let current = frame.current;
        frame.blocks[current].1.push(stmt);
    }

    /// Names the result of `op`.
    fn bind(&mut self, name: &str, op: Op) -> Atom {
        let var = self.frame().var(name);
        self.emit(Stmt::Let(var, op));
        Atom::Var(var)
    }

    /// A variable holding `atom`, for binding a name to it.
    fn var_for(&mut self, name: &str, atom: Atom) -> Var {
        match atom {
            Atom::Var(var) => var,
            atom => match self.bind(name, Op::Atom(atom)) {
                Atom::Var(var) => var,
                _ => unreachable!(),
            },
        }
    }

    fn terminate(&mut self, term: Term) {
        let frame = self.frame();
        let current = frame.current;
        debug_assert!(frame.blocks[current].2.is_none(), "block terminated twice");
        frame.blocks[current].2 = Some(term);
    }

    fn switch(&mut self, block: BlockId) {
        self.frame().current = block;
    }

    fn block(&mut self, params: Vec<Var>) -> BlockId {
        self.frame().block(params)
    }

    fn lookup(&self, name: &str) -> Option<Binding> {
        self.lookup_raw(name)
            .filter(|binding| !matches!(binding, Binding::Constructor(_)))
    }

    fn lookup_raw(&self, name: &str) -> Option<Binding> {
        if let Some((path, base)) = name.rsplit_once('.') {
            let (path, key) = match path.strip_prefix("exn ") {
                Some(path) => (path, format!("exn {base}")),
                None => (path, base.to_string()),
            };
            return self
                .structure_named(path)?
                .values
                .iter()
                .rev()
                .find(|(name, _)| name == &key)
                .map(|(_, binding)| binding.clone());
        }
        self.scope
            .iter()
            .rev()
            .find(|(known, _)| known == name)
            .map(|(_, binding)| binding.clone())
            .or_else(|| self.session.lookup(name))
    }

    fn structure_named(&self, path: &str) -> Option<&Structure> {
        let mut parts = path.split('.');
        let name = parts.next()?;
        let mut structure = &self
            .session
            .structures
            .iter()
            .rev()
            .find(|(known, _)| known == name)?
            .1;
        for name in parts {
            structure = &structure
                .structures
                .iter()
                .rev()
                .find(|(known, _)| known == name)?
                .1;
        }
        Some(structure)
    }

    fn environment(&self) -> Environment {
        Environment {
            globals: self.session.globals.clone(),
            locals: self.scope.clone(),
            structures: self.session.structures.clone(),
            functors: self.session.functors.clone(),
        }
    }

    fn restore(&mut self, environment: Environment) {
        self.session.globals = environment.globals;
        self.scope = environment.locals;
        self.session.structures = environment.structures;
        self.session.functors = environment.functors;
    }

    fn open(&mut self, structure: &Structure, top: bool) {
        if top {
            self.session.globals.extend(structure.values.clone());
        } else {
            self.scope.extend(structure.values.clone());
        }
        self.session.structures.extend(structure.structures.clone());
    }

    fn structure(&mut self, exp: &StrExp, top: bool) -> Res<Structure> {
        let saved = self.environment();
        let result = match &exp.value {
            StrExpKind::Name(name) => self
                .structure_named(name)
                .cloned()
                .expect("checked structure"),
            StrExpKind::Struct(declarations) => {
                self.declarations(declarations, top)?;
                let values = if top {
                    self.session.globals[saved.globals.len()..].to_vec()
                } else {
                    self.scope[saved.locals.len()..].to_vec()
                };
                Structure {
                    values,
                    structures: self.session.structures[saved.structures.len()..].to_vec(),
                }
            }
            StrExpKind::Ascribed { body, .. } => self.structure(body, top)?,
            StrExpKind::Let(declarations, body) => {
                self.declarations(declarations, top)?;
                self.structure(body, top)?
            }
            StrExpKind::Apply(name, argument) => {
                let functor = self
                    .session
                    .functors
                    .iter()
                    .rev()
                    .find(|(known, _)| known == name)
                    .map(|(_, functor)| functor.clone())
                    .expect("checked functor");
                let argument = self.structure(argument, top)?;
                let parameter = self
                    .types
                    .structure(exp)
                    .parameter
                    .as_ref()
                    .expect("elaborated parameter");
                let argument = self.restrict(argument, parameter, top)?;
                self.restore(functor.environment.clone());
                match &functor.parameter {
                    Some(name) => self.session.structures.push((name.clone(), argument)),
                    None => self.open(&argument, top),
                }
                let body = self
                    .types
                    .structure(exp)
                    .application
                    .clone()
                    .expect("elaborated functor body");
                let source = std::mem::replace(&mut self.source, functor.source.clone());
                let result = self.structure(&body, top);
                self.source = source;
                result?
            }
        };
        self.restore(saved);
        self.restrict(result, self.types.structure(exp), top)
    }

    fn restrict(
        &mut self,
        structure: Structure,
        info: &crate::infer::StructureInfo,
        top: bool,
    ) -> Res<Structure> {
        let mut values = Vec::new();
        for export in &info.values {
            let name = &export.name;
            if let Some((_, binding)) = structure
                .values
                .iter()
                .rev()
                .find(|(known, _)| known == name)
            {
                let binding = if !export.constructor
                    && let Binding::Constructor(test) = binding
                {
                    let (test, carries) = match test {
                        Some(test) => test.clone(),
                        None => {
                            let key = format!("exn {name}");
                            let identity = &structure
                                .values
                                .iter()
                                .rev()
                                .find(|(known, _)| known == &key)
                                .expect("exported exception identity")
                                .1;
                            let identity = self.load(&key, identity);
                            (
                                Test::Exception {
                                    identity,
                                    carries: export.carries,
                                },
                                export.carries,
                            )
                        }
                    };
                    let value = if carries {
                        self.constructor_closure(name, &test)
                    } else {
                        self.construct(&test, Atom::Word(value::tagged(0)))
                    };
                    if top {
                        let global = self.new_global(name, None);
                        self.emit(Stmt::SetGlobal(global, value));
                        self.session.globals.pop();
                        Binding::Global(global, None)
                    } else {
                        let var = self.var_for(name, value);
                        Binding::Local {
                            depth: self.depth(),
                            var,
                            known: None,
                        }
                    }
                } else {
                    binding.clone()
                };
                values.push((name.clone(), binding));
            } else if export.constructor {
                values.push((name.clone(), Binding::Constructor(None)));
            } else {
                return Err((
                    LowerError::Unsupported(format!("exported Basis value {name}")),
                    (0, 0).into(),
                ));
            }
            let key = format!("exn {name}");
            if let Some((_, binding)) = structure
                .values
                .iter()
                .rev()
                .find(|(known, _)| known == &key)
            {
                values.push((key, binding.clone()));
            }
        }
        let structures = info
            .structures
            .iter()
            .map(|(name, info)| {
                let inner = structure
                    .structures
                    .iter()
                    .rev()
                    .find(|(known, _)| known == name)
                    .expect("checked exported structure")
                    .1
                    .clone();
                Ok((name.clone(), self.restrict(inner, info, top)?))
            })
            .collect::<Res<_>>()?;
        Ok(Structure { values, structures })
    }

    /// The variable of the current function that holds variable `var` of the
    /// function at `depth`, capturing it into every function in between.
    fn access(&mut self, depth: usize, var: Var) -> Var {
        self.capture(self.depth(), depth, var)
    }

    fn capture(&mut self, at: usize, depth: usize, var: Var) -> Var {
        if at == depth {
            return var;
        }
        let frame = &mut self.frames[at];
        if let Some((_, found)) = frame.captures.iter().find(|(key, _)| *key == (depth, var)) {
            return *found;
        }
        // Field 0 of a closure is its code.
        let index = frame.captures.len() + 1;
        let name = self.frames[depth].vars[var].clone();
        let frame = &mut self.frames[at];
        let loaded = frame.var(&name);
        let env = frame.params[0];
        frame
            .prologue
            .push(Stmt::Let(loaded, Op::Select(Atom::Var(env), index)));
        frame.captures.push(((depth, var), loaded));
        loaded
    }

    /// The value of a bound name.
    fn load(&mut self, name: &str, binding: &Binding) -> Atom {
        match binding {
            Binding::Constructor(_) => unreachable!("constructors have no variable storage"),
            Binding::Global(global, _) => self.bind(name, Op::Global(*global)),
            Binding::Local { depth, var, .. } => Atom::Var(self.access(*depth, *var)),
        }
    }

    fn ty(&self, expr: &Expr) -> Option<&Ty> {
        self.types.expr(expr)
    }

    /// Whether `=` on values of type `ty` can compare their words: every
    /// value is an immediate, or `ty` is a reference, which is equal only to
    /// itself.
    fn compared_by_word(&self, ty: &Ty) -> bool {
        match ty {
            Ty::Con { name, stamp: 0, .. } => {
                matches!(
                    name.as_str(),
                    "int" | "word" | "char" | "bool" | "unit" | "order" | "ref"
                )
            }
            // An enumeration: every constructor is nullary.
            Ty::Con { stamp, .. } => self
                .types
                .constructors(*stamp)
                .is_some_and(|constructors| constructors.iter().all(|(_, carries)| !carries)),
            Ty::Record(fields) => fields.is_empty(),
            _ => false,
        }
    }

    fn is_real(&self, expr: &Expr) -> bool {
        self.ty(expr).is_some_and(|ty| ty.is("real"))
    }

    // --- declarations ----------------------------------------------------

    fn statement(&mut self, statement: &StmtKind) -> Res<()> {
        match statement {
            StmtKind::Val(name, expr) => {
                let value = self.value(expr)?;
                self.bind_top(name, value, None);
            }
            StmtKind::Print(expr) => {
                let text = self.value(expr)?;
                self.bind("", Op::Prim(Prim::Print, vec![text]));
            }
            StmtKind::Exit(expr) => {
                self.value(expr)?;
            }
            StmtKind::Declaration(declaration) => self.declaration(declaration, true)?,
        }
        Ok(())
    }

    /// Binds the top-level `name` to `value` in a new global.
    fn bind_top(&mut self, name: &str, value: Atom, known: Option<Known>) {
        let global = self.new_global(name, known);
        self.emit(Stmt::SetGlobal(global, value));
    }

    fn new_global(&mut self, name: &str, known: Option<Known>) -> GlobalId {
        let global = self.session.next_global;
        self.session.next_global += 1;
        self.new_globals.push((global, name.to_string()));
        self.session
            .globals
            .push((name.to_string(), Binding::Global(global, known)));
        global
    }

    /// Lowers a declaration. At the top level, the names it binds become
    /// globals; elsewhere they are local variables.
    fn declaration(&mut self, declaration: &Decl, top: bool) -> Res<()> {
        match &declaration.value {
            DeclKind::Val {
                recursive: false,
                bindings,
            } => {
                // Every right-hand side is evaluated before any name is bound.
                let mut values = Vec::new();
                for (_, expr) in bindings {
                    values.push(self.value(expr)?);
                }
                let mut bound = Vec::new();
                for ((pattern, expr), value) in bindings.iter().zip(values) {
                    let location = self.source.span(pattern.start.offset, expr.end.offset);
                    let decided = self.decide(
                        &[value],
                        &[vec![pattern]],
                        Term::Fail(crate::core::Failure::Bind, location),
                    )?;
                    let (Some(next), names) = decided.into_iter().next().expect("one rule") else {
                        // No value matches: the declaration always raises.
                        let next = self.block(Vec::new());
                        self.switch(next);
                        continue;
                    };
                    // The code after the binding continues where the pattern
                    // matched, with its variables as the block's parameters.
                    self.switch(next);
                    let params = self.frame().blocks[next].0.clone();
                    let mark = self.scope.len();
                    for (name, var) in names.iter().zip(params) {
                        self.bind_local(name, Atom::Var(var));
                    }
                    bound.extend(self.scope.drain(mark..));
                }
                self.bind_names(bound, top);
            }
            DeclKind::Val {
                recursive: true,
                bindings,
            } => {
                let mut definitions = Vec::new();
                for (pattern, expr) in bindings {
                    let (name, rules) = match (
                        &strip_typed_pattern(pattern).value,
                        &strip_typed(expr).value,
                    ) {
                        (PatKind::Variable(name), ExprKind::Fn(rules)) => (name, rules),
                        _ => return Err(unsupported("val rec bindings that are not fn", expr)),
                    };
                    definitions.push(Definition {
                        name,
                        clauses: rules.iter().map(|(p, body)| (vec![p], body)).collect(),
                        location: self.source.end(expr),
                    });
                }
                self.functions_group(definitions, top)?;
            }
            DeclKind::Fun(bindings) => {
                let definitions = bindings
                    .iter()
                    .map(|binding| Definition {
                        name: &binding.name,
                        clauses: binding
                            .clauses
                            .iter()
                            .map(|clause| (clause.parameters.iter().collect(), &clause.body))
                            .collect(),
                        location: binding
                            .clauses
                            .last()
                            .map_or_else(String::new, |clause| self.source.end(&clause.body)),
                    })
                    .collect();
                self.functions_group(definitions, top)?;
            }
            DeclKind::Local(private, public) => {
                let structures_before = self.session.structures.len();
                let functors_before = self.session.functors.len();
                // The private declarations' names are dropped once the
                // public ones are lowered; their values live on.
                let (outer, inner) = if top {
                    let outer = self.session.globals.len();
                    self.declarations(private, top)?;
                    (outer, self.session.globals.len())
                } else {
                    let outer = self.scope.len();
                    self.declarations(private, top)?;
                    (outer, self.scope.len())
                };
                let structures_private = self.session.structures.len();
                let functors_private = self.session.functors.len();
                self.declarations(public, top)?;
                self.session
                    .structures
                    .drain(structures_before..structures_private);
                self.session
                    .functors
                    .drain(functors_before..functors_private);
                if top {
                    self.session.globals.drain(outer..inner);
                } else {
                    self.scope.drain(outer..inner);
                }
            }
            // Types have no run-time presence, and constructors are compiled
            // where they are used; fixity only affects parsing.
            DeclKind::Type(_) | DeclKind::Fixity { .. } | DeclKind::Signature(_) => {}
            DeclKind::Datatype { .. } | DeclKind::DatatypeCopy { .. } => {
                self.datatype_constructors(declaration, top);
            }
            DeclKind::Abstype { body, .. } => {
                let before = if top {
                    self.session.globals.len()
                } else {
                    self.scope.len()
                };
                self.datatype_constructors(declaration, top);
                let after = if top {
                    self.session.globals.len()
                } else {
                    self.scope.len()
                };
                self.declarations(body, top)?;
                if top {
                    self.session.globals.drain(before..after);
                } else {
                    self.scope.drain(before..after);
                }
            }
            DeclKind::Exception(bindings) => {
                // Each evaluation of the declaration makes new exceptions:
                // an exception's identity is a fresh cell holding its name.
                // A replication shares the identity it names.
                for (index, binding) in bindings.iter().enumerate() {
                    let identity = match &binding.kind {
                        ExceptionKind::Fresh(_) if !self.session.printing => self.bind(
                            &binding.name,
                            Op::Prim(Prim::Ref, vec![Atom::String(binding.name.clone())]),
                        ),
                        ExceptionKind::Fresh(_) => {
                            let descriptor = self
                                .types
                                .exception_argument(declaration, index)
                                .cloned()
                                .map(|ty| self.type_descriptor(&ty))
                                .unwrap_or(Atom::Word(value::NIL));
                            self.bind(
                                &binding.name,
                                Op::Record(vec![Atom::String(binding.name.clone()), descriptor]),
                            )
                        }
                        ExceptionKind::Copy(original) => {
                            let exn = Ty::Con {
                                name: "exn".to_string(),
                                stamp: 0,
                                args: Vec::new(),
                            };
                            match self.exception(original, Some(&exn)) {
                                Some((identity, _)) => identity,
                                None => {
                                    return Err((
                                        LowerError::Unsupported(
                                            "this exception replication".into(),
                                        ),
                                        declaration.source_span(),
                                    ));
                                }
                            }
                        }
                    };
                    let key = format!("exn {}", binding.name);
                    self.bind_constructor(&binding.name, None, top);
                    if top {
                        self.bind_top(&key, identity, None);
                    } else {
                        self.bind_local(&key, identity);
                    }
                }
            }
            DeclKind::Structure(bindings) => {
                let mut built = Vec::new();
                for binding in bindings {
                    built.push((binding.name.clone(), self.structure(&binding.body, top)?));
                }
                self.session.structures.extend(built);
            }
            DeclKind::Open(paths) => {
                for path in paths {
                    let structure = self.structure_named(path).cloned().expect("checked open");
                    self.open(&structure, top);
                }
            }
            DeclKind::Functor(bindings) => {
                let environment = self.environment();
                for binding in bindings {
                    let parameter = match &binding.parameter {
                        FunctorParameter::Named(name, _) => Some(name.clone()),
                        FunctorParameter::Specs(_) => None,
                    };
                    self.session.functors.push((
                        binding.name.clone(),
                        Rc::new(Functor {
                            parameter,
                            roots: functor_roots::roots(
                                &crate::span::Span::new(
                                    declaration.start.clone(),
                                    declaration.end.clone(),
                                    DeclKind::Functor(vec![binding.clone()]),
                                ),
                                &environment,
                            ),
                            environment: environment.clone(),
                            source: self.source.clone(),
                        }),
                    ));
                }
            }
        }
        Ok(())
    }

    fn type_descriptor(&mut self, ty: &Ty) -> Atom {
        let fields = match ty {
            Ty::Con { name, stamp, args } => {
                let args: Vec<_> = args.iter().map(|ty| self.type_descriptor(ty)).collect();
                let args = self.bind("", Op::Record(args));
                vec![
                    Atom::Word(value::tagged(0)),
                    Atom::String(name.clone()),
                    Atom::Word(value::tagged(*stamp as i64)),
                    args,
                ]
            }
            Ty::Record(fields) => {
                let fields: Vec<_> = fields
                    .iter()
                    .map(|(name, ty)| {
                        let ty = self.type_descriptor(ty);
                        self.bind("", Op::Record(vec![Atom::String(name.clone()), ty]))
                    })
                    .collect();
                let fields = self.bind("", Op::Record(fields));
                vec![Atom::Word(value::tagged(1)), fields]
            }
            Ty::Arrow(..) => vec![Atom::Word(value::tagged(2))],
            Ty::Var { .. } => vec![Atom::Word(value::tagged(3))],
        };
        self.bind("", Op::Record(fields))
    }

    fn declarations(&mut self, declarations: &[Decl], top: bool) -> Res<()> {
        for declaration in declarations {
            self.declaration(declaration, top)?;
        }
        Ok(())
    }

    fn bind_constructor(&mut self, name: &str, test: Option<(Test, bool)>, top: bool) {
        let binding = (name.to_string(), Binding::Constructor(test));
        if top {
            self.session.globals.push(binding);
        } else {
            self.scope.push(binding);
        }
    }

    fn datatype_constructors(&mut self, declaration: &Decl, top: bool) {
        for (name, stamp) in self.types.declaration_constructors(declaration) {
            let ty = Ty::Con {
                name: String::new(),
                stamp: *stamp,
                args: Vec::new(),
            };
            let test = self
                .constructor_type(name, Some(&ty))
                .expect("declared datatype constructor");
            self.bind_constructor(name, Some(test), top);
        }
    }

    /// Makes names bound by patterns visible: as globals at the top level.
    fn bind_names(&mut self, bound: Vec<(String, Binding)>, top: bool) {
        for (name, binding) in bound {
            if top {
                let value = self.load(&name, &binding);
                self.bind_top(&name, value, None);
            } else {
                self.scope.push((name, binding));
            }
        }
    }

    /// Compiles an `and` group of mutually recursive functions.
    fn functions_group(&mut self, definitions: Vec<Definition>, top: bool) -> Res<()> {
        let depth = self.depth();
        let mut group = Vec::new();
        for definition in &definitions {
            let arity = definition.clauses[0].0.len();
            let known = Known {
                worker: self.session.function_id(),
                arity,
            };
            // The closure is a variable of the enclosing function; at the top
            // level it is also stored in a global.
            let closure = self.frame().var(definition.name);
            let global = if top {
                Some(self.new_global(definition.name, Some(known)))
            } else {
                self.scope.push((
                    definition.name.to_string(),
                    Binding::Local {
                        depth,
                        var: closure,
                        known: Some(known),
                    },
                ));
                None
            };
            group.push((closure, known, global));
        }
        let mut closures = Vec::new();
        for (definition, (closure, known, _)) in definitions.iter().zip(&group) {
            let captures = self.function(known.worker, definition)?;
            let captured = captures
                .into_iter()
                .map(|(depth, var)| Atom::Var(self.access(depth, var)))
                .collect();
            let code = if known.arity == 1 {
                known.worker
            } else {
                self.curried_entry(definition.name, *known)
            };
            closures.push(Closure {
                var: *closure,
                code,
                captured,
            });
        }
        self.emit(Stmt::Closures(closures));
        for (closure, _, global) in group {
            if let Some(global) = global {
                self.emit(Stmt::SetGlobal(global, Atom::Var(closure)));
            }
        }
        Ok(())
    }

    /// The code of a curried function's closure: stage `k` takes the `k`th
    /// argument and returns a closure holding the arguments so far, and the
    /// last stage calls the worker with all of them. Stage 1's environment
    /// is the function's own closure, which is also the worker's.
    fn curried_entry(&mut self, name: &str, known: Known) -> FnId {
        let stages: Vec<FnId> = (0..known.arity)
            .map(|_| self.session.function_id())
            .collect();
        for (index, id) in stages.iter().enumerate() {
            self.frames.push(Builder::new(*id, name, vec!["env", ""]));
            let env = Atom::Var(self.frame().params[0]);
            let argument = Atom::Var(self.frame().params[1]);
            // The worker's environment and the earlier arguments.
            let mut held = if index == 0 {
                vec![env]
            } else {
                (1..=index + 1)
                    .map(|field| self.bind("", Op::Select(env.clone(), field)))
                    .collect()
            };
            held.push(argument);
            if index + 1 == known.arity {
                let env = held.remove(0);
                self.terminate(Term::TailCall(Callee::Known(known.worker, env), held));
            } else {
                let result = self.closure("", stages[index + 1], held);
                self.terminate(Term::Return(result));
            }
            let frame = self.frames.pop().expect("the stage's frame");
            self.functions.push(frame.finish());
        }
        stages[0]
    }

    /// Compiles `definition` as the function `id`, taking its closure and one
    /// argument per curried parameter; returns the variables it captures.
    fn function(&mut self, id: FnId, definition: &Definition) -> Res<Vec<(usize, Var)>> {
        let arity = definition.clauses[0].0.len();
        let mut params = vec!["env"];
        params.extend(std::iter::repeat_n("", arity));
        self.frames.push(Builder::new(id, definition.name, params));
        let mark = self.scope.len();
        let args: Vec<Atom> = self.frame().params[1..]
            .iter()
            .map(|var| Atom::Var(*var))
            .collect();
        let result = self.rules(
            &args,
            &definition.clauses,
            Dest::Return,
            Term::Fail(crate::core::Failure::Match, definition.location.clone()),
        );
        self.scope.truncate(mark);
        let frame = self.frames.pop().expect("the function's frame");
        result?;
        let captures = frame.captures.iter().map(|(key, _)| *key).collect();
        self.functions.push(frame.finish());
        Ok(captures)
    }

    // --- matching --------------------------------------------------------

    /// Binds `name` to `value` in the scope.
    fn bind_local(&mut self, name: &str, value: Atom) -> Var {
        let var = self.var_for(name, value);
        // Name an anonymous variable, such as a parameter, for dumps.
        let frame = self.frame();
        if frame.vars[var].is_empty() {
            frame.vars[var] = name.to_string();
        }
        let depth = self.depth();
        self.scope.push((
            name.to_string(),
            Binding::Local {
                depth,
                var,
                known: None,
            },
        ));
        var
    }

    // --- expressions -----------------------------------------------------

    /// Lowers `expr` and sends its value to `dest`, ending the current block.
    fn into(&mut self, expr: &Expr, dest: Dest) -> Res<()> {
        match &expr.value {
            ExprKind::If(condition, consequent, alternative) => {
                let condition = self.value(condition)?;
                let then = self.block(Vec::new());
                let otherwise = self.block(Vec::new());
                self.terminate(Term::If(condition, then, otherwise));
                self.switch(then);
                self.into(consequent, dest)?;
                self.switch(otherwise);
                self.into(alternative, dest)
            }
            ExprKind::Case(scrutinee, rules) => {
                let scrutinee = self.value(scrutinee)?;
                let rules: Vec<(Vec<&Pat>, &Expr)> = rules
                    .iter()
                    .map(|(pattern, body)| (vec![pattern], body))
                    .collect();
                let miss = Term::Fail(crate::core::Failure::Match, self.source.end(expr));
                self.rules(&[scrutinee], &rules, dest, miss)
            }
            ExprKind::Typed(inner, _) => self.into(inner, dest),
            ExprKind::Handle(body, rules) => {
                // The handler starts with the exception as its parameter. The
                // body runs in blocks that raise to it, so none of its calls
                // is a tail call: the handler must outlive them.
                let exception = self.frame().var("");
                let handler = self.block(vec![exception]);
                self.frame().handlers.push(handler);
                let start = self.block(Vec::new());
                self.terminate(Term::Jump(start, Vec::new()));
                self.switch(start);
                let lowered = match dest {
                    Dest::Jump(_) => self.into(body, dest),
                    Dest::Return => self.value(body).map(|value| self.send(value, dest)),
                };
                self.frame().handlers.pop();
                lowered?;
                self.switch(handler);
                let rules: Vec<(Vec<&Pat>, &Expr)> = rules
                    .iter()
                    .map(|(pattern, body)| (vec![pattern], body))
                    .collect();
                // An exception no rule matches goes on to the enclosing
                // handler.
                let miss = Term::Raise(Atom::Var(exception), None);
                self.rules(&[Atom::Var(exception)], &rules, dest, miss)
            }
            ExprKind::Let(declarations, body) => {
                let mark = self.scope.len();
                let structures = self.session.structures.len();
                let functors = self.session.functors.len();
                self.declarations(declarations, false)?;
                self.into(body, dest)?;
                self.scope.truncate(mark);
                self.session.structures.truncate(structures);
                self.session.functors.truncate(functors);
                Ok(())
            }
            // A call whose value is returned is a tail call: it reuses the
            // caller's frame, so loops written as recursion run in constant
            // stack.
            ExprKind::Apply(..) if matches!(dest, Dest::Return) => {
                match self.application(expr)? {
                    Applied::Value(value) => self.send(value, dest),
                    Applied::Call(callee, args) => self.terminate(Term::TailCall(callee, args)),
                }
                Ok(())
            }
            ExprKind::Sequence(items) => {
                let (last, rest) = items.split_last().expect("a sequence has an expression");
                for item in rest {
                    self.value(item)?;
                }
                self.into(last, dest)
            }
            _ => {
                let value = self.value(expr)?;
                self.send(value, dest);
                Ok(())
            }
        }
    }

    fn send(&mut self, value: Atom, dest: Dest) {
        match dest {
            Dest::Return => self.terminate(Term::Return(value)),
            Dest::Jump(join) => self.terminate(Term::Jump(join, vec![value])),
        }
    }

    /// Lowers a branching expression through a join block.
    fn joined(&mut self, expr: &Expr) -> Res<Atom> {
        let result = self.frame().var("");
        let join = self.block(vec![result]);
        self.into(expr, Dest::Jump(join))?;
        self.switch(join);
        Ok(Atom::Var(result))
    }

    /// Lowers `expr` in the current block and returns its value.
    fn value(&mut self, expr: &Expr) -> Res<Atom> {
        Ok(match &expr.value {
            ExprKind::Integer(value) => Atom::Word(value::tagged(*value)),
            ExprKind::Real(value) => Atom::Real(*value),
            ExprKind::Boolean(value) => Atom::Word(value::tagged(i64::from(*value))),
            ExprKind::Character(value) => Atom::Word(value::tagged(i64::from(u32::from(*value)))),
            ExprKind::String(value) => Atom::String(value.clone()),
            ExprKind::Unit => Atom::Word(value::tagged(0)),
            ExprKind::Word(text) => match decision::parse_word(text) {
                Some(word) => Atom::Word(value::tagged(word)),
                None => return Err(unsupported("this word constant", expr)),
            },
            ExprKind::Variable(name)
                if let Some((identity, carries)) =
                    self.exception(name, self.ty(expr).cloned().as_ref()) =>
            {
                let test = Test::Exception { identity, carries };
                if carries {
                    self.constructor_closure(name, &test)
                } else {
                    self.construct(&test, Atom::Word(value::tagged(0)))
                }
            }
            ExprKind::Variable(name) => match self.lookup(name) {
                Some(binding) => self.load(name, &binding),
                None => match (self.constructor(name, self.ty(expr)), builtin(name)) {
                    (Some((Test::Word { word, .. }, _)), _) => Atom::Word(word),
                    (Some((test, _)), _) => self.constructor_closure(name, &test),
                    (None, Some(builtin)) => {
                        let operand = match self.ty(expr) {
                            Some(Ty::Arrow(from, _)) => Some(from.as_ref()),
                            _ => None,
                        };
                        let builtin = self.typed_builtin(builtin, operand, expr)?;
                        self.builtin_closure(name, builtin)
                    }
                    (None, None) => {
                        return Err(unsupported(
                            &format!("built-in functions used as values, such as {name},"),
                            expr,
                        ));
                    }
                },
            },
            ExprKind::Tuple(items) => {
                let items = items
                    .iter()
                    .map(|item| self.value(item))
                    .collect::<Res<Vec<_>>>()?;
                self.bind("", Op::Record(items))
            }
            ExprKind::Record(fields) => {
                // Fields are evaluated in source order and stored in label
                // order.
                let Some(Ty::Record(labels)) = self.ty(expr).cloned() else {
                    return Err(unsupported("this record", expr));
                };
                let mut values = Vec::new();
                for (label, field) in fields {
                    values.push((label.clone(), self.value(field)?));
                }
                let ordered = labels
                    .iter()
                    .map(|(label, _)| {
                        values
                            .iter()
                            .find(|(known, _)| known == label)
                            .map(|(_, value)| value.clone())
                            .expect("the record has every label of its type")
                    })
                    .collect();
                self.bind("", Op::Record(ordered))
            }
            ExprKind::List(items) => {
                let items = items
                    .iter()
                    .map(|item| self.value(item))
                    .collect::<Res<Vec<_>>>()?;
                let mut list = Atom::Word(value::NIL);
                for item in items.into_iter().rev() {
                    list = self.bind("", Op::Record(vec![item, list]));
                }
                list
            }
            ExprKind::If(..) | ExprKind::Case(..) | ExprKind::Handle(..) => self.joined(expr)?,
            ExprKind::Raise(argument) => {
                let exception = self.value(argument)?;
                let location = self.source.raised(argument);
                self.terminate(Term::Raise(exception, Some(location)));
                // Code after the raise is unreachable, but still lowered.
                let next = self.block(Vec::new());
                self.switch(next);
                Atom::Word(value::tagged(0))
            }
            ExprKind::AndAlso(lhs, rhs) | ExprKind::OrElse(lhs, rhs) => {
                let result = self.frame().var("");
                let join = self.block(vec![result]);
                let condition = self.value(lhs)?;
                let evaluate = self.block(Vec::new());
                let skip = self.block(Vec::new());
                let (then, otherwise, short) = match &expr.value {
                    ExprKind::AndAlso(..) => (evaluate, skip, value::FALSE),
                    _ => (skip, evaluate, value::TRUE),
                };
                self.terminate(Term::If(condition, then, otherwise));
                self.switch(skip);
                self.terminate(Term::Jump(join, vec![Atom::Word(short)]));
                self.switch(evaluate);
                self.into(rhs, Dest::Jump(join))?;
                self.switch(join);
                Atom::Var(result)
            }
            ExprKind::Sequence(items) => {
                let mut last = Atom::Word(value::tagged(0));
                for item in items {
                    last = self.value(item)?;
                }
                last
            }
            ExprKind::Typed(inner, _) => self.value(inner)?,
            ExprKind::Let(declarations, body) => {
                let mark = self.scope.len();
                let structures = self.session.structures.len();
                let functors = self.session.functors.len();
                self.declarations(declarations, false)?;
                let value = self.value(body)?;
                self.scope.truncate(mark);
                self.session.structures.truncate(structures);
                self.session.functors.truncate(functors);
                value
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
                if self.ty(lhs).is_some_and(|ty| ty.is("word")) {
                    return Err(unsupported("word arithmetic and comparisons", expr));
                }
                let real = self.is_real(lhs);
                let prim = match (&expr.value, real) {
                    (ExprKind::Less(..), _) if self.ty(lhs).is_some_and(|ty| ty.is("string")) => {
                        Prim::StringLt
                    }
                    (ExprKind::LessEqual(..), _)
                        if self.ty(lhs).is_some_and(|ty| ty.is("string")) =>
                    {
                        Prim::StringLe
                    }
                    (ExprKind::Greater(..), _)
                        if self.ty(lhs).is_some_and(|ty| ty.is("string")) =>
                    {
                        Prim::StringGt
                    }
                    (ExprKind::GreaterEqual(..), _)
                        if self.ty(lhs).is_some_and(|ty| ty.is("string")) =>
                    {
                        Prim::StringGe
                    }
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
                let mut args = vec![self.value(lhs)?, self.value(rhs)?];
                if prim == Prim::IntDiv {
                    args.push(Atom::String(self.source.operator("div", lhs, rhs)));
                }
                self.bind("", Op::Prim(prim, args))
            }
            ExprKind::Equal(lhs, rhs) | ExprKind::NotEqual(lhs, rhs) => {
                // Values whose type makes them immediates are equal when
                // their words are, and references when they are the same
                // cell; anything else is compared by the runtime.
                let immediate = self.ty(lhs).is_some_and(|ty| self.compared_by_word(ty));
                let prim = match (immediate, matches!(expr.value, ExprKind::Equal(..))) {
                    (true, true) => Prim::WordEq,
                    (true, false) => Prim::WordNe,
                    (false, true) => Prim::Equal,
                    (false, false) => Prim::Unequal,
                };
                let lhs = self.value(lhs)?;
                let rhs = self.value(rhs)?;
                self.bind("", Op::Prim(prim, vec![lhs, rhs]))
            }
            ExprKind::Infix(name, lhs, rhs) => self.infix(name, lhs, rhs, expr)?,
            ExprKind::Fn(rules) => {
                let definition = Definition {
                    name: "fn",
                    clauses: rules.iter().map(|(p, body)| (vec![p], body)).collect(),
                    location: self.source.end(expr),
                };
                self.lambda("", &definition)?
            }
            ExprKind::While(condition, body) => {
                let header = self.block(Vec::new());
                let looped = self.block(Vec::new());
                let done = self.block(Vec::new());
                self.terminate(Term::Jump(header, Vec::new()));
                self.switch(header);
                let condition = self.value(condition)?;
                self.terminate(Term::If(condition, looped, done));
                self.switch(looped);
                self.value(body)?;
                self.terminate(Term::Jump(header, Vec::new()));
                self.switch(done);
                Atom::Word(value::tagged(0))
            }
            ExprKind::Apply(..) => self.apply(expr)?,
            ExprKind::Selector(label) => {
                let Some(Ty::Arrow(record, _)) = self.ty(expr) else {
                    return Err(unsupported("selectors on records of unknown shape", expr));
                };
                let Some(index) = field_index(record, label) else {
                    return Err(unsupported("selectors on records of unknown shape", expr));
                };
                self.selector_closure(label, index)
            }
            ExprKind::PosixExit(word8) => {
                let ExprKind::Word8FromInt(status) = &word8.value else {
                    return Err(unsupported(
                        "Posix.Process.exit without Word8.fromInt",
                        expr,
                    ));
                };
                let status = self.value(status)?;
                self.bind("", Op::Prim(Prim::Exit, vec![status]))
            }
            other => return Err(unsupported(unsupported_name(other), expr)),
        })
    }

    fn infix(&mut self, name: &str, lhs: &Expr, rhs: &Expr, expr: &Expr) -> Res<Atom> {
        let prim = match name {
            "::" => None,
            "^" => Some(Prim::Concat),
            "mod" => Some(Prim::IntMod),
            ":=" => Some(Prim::Assign),
            _ => {
                // A function declared infix is called with the pair.
                let Some(binding) = self.lookup(name) else {
                    return Err(unsupported(&format!("the infix operator {name}"), expr));
                };
                let pair = vec![self.value(lhs)?, self.value(rhs)?];
                let pair = self.bind("", Op::Record(pair));
                let known = match &binding {
                    Binding::Constructor(_) => unreachable!("constructor calls are inlined"),
                    Binding::Global(_, known) | Binding::Local { known, .. } => *known,
                };
                let function = self.load(name, &binding);
                let callee = match known {
                    Some(Known { worker, arity: 1 }) => Callee::Known(worker, function),
                    _ => Callee::Closure(function),
                };
                return Ok(self.bind("", Op::Call(callee, vec![pair])));
            }
        };
        let mut args = vec![self.value(lhs)?, self.value(rhs)?];
        Ok(match prim {
            // A cons cell is a record of the head and the tail.
            None => self.bind("", Op::Record(args)),
            Some(prim) => {
                if prim == Prim::IntMod {
                    args.push(Atom::String(self.source.operator("mod", lhs, rhs)));
                }
                self.bind("", Op::Prim(prim, args))
            }
        })
    }

    /// Lowers an application.
    fn apply(&mut self, expr: &Expr) -> Res<Atom> {
        Ok(match self.application(expr)? {
            Applied::Value(value) => value,
            Applied::Call(callee, args) => self.bind("", Op::Call(callee, args)),
        })
    }

    /// Lowers an application up to its last call, which is left to the
    /// caller so that a call in tail position can become a tail call.
    fn application(&mut self, expr: &Expr) -> Res<Applied> {
        let mut args = Vec::new();
        let mut head = expr;
        while let ExprKind::Apply(function, argument) = &head.value {
            args.push(argument.as_ref());
            head = function;
        }
        args.reverse();
        let head = strip_typed(head);
        let mut rest = args.as_slice();
        let mut function = None;
        if let ExprKind::Variable(name) = &head.value {
            let binding = self.lookup(name);
            let exception = self.exception(name, self.ty(head).cloned().as_ref());
            if let Some((identity, true)) = exception {
                let argument = self.value(args[0])?;
                let test = Test::Exception {
                    identity,
                    carries: true,
                };
                function = Some(self.construct(&test, argument));
                rest = &args[1..];
            } else if binding.is_none()
                && let Some((test, true)) = self.constructor(name, self.ty(head))
            {
                let argument = self.value(args[0])?;
                function = Some(self.construct(&test, argument));
                rest = &args[1..];
            } else if binding.is_none()
                && let Some(builtin) = builtin(name)
            {
                let builtin = self.typed_builtin(builtin, self.ty(args[0]), head)?;
                let argument = self.value(args[0])?;
                function = Some(self.builtin(builtin, argument));
                rest = &args[1..];
            } else if let Some(binding) = binding {
                let known = match &binding {
                    Binding::Constructor(_) => unreachable!("constructor calls are inlined"),
                    Binding::Global(_, known) | Binding::Local { known, .. } => *known,
                };
                if let Some(known) = known
                    && args.len() >= known.arity
                {
                    let closure = self.load(name, &binding);
                    let values = args[..known.arity]
                        .iter()
                        .map(|arg| self.value(arg))
                        .collect::<Res<Vec<_>>>()?;
                    let mut call = Applied::Call(Callee::Known(known.worker, closure), values);
                    for argument in &args[known.arity..] {
                        let function = self.finish_call(call);
                        let argument = self.value(argument)?;
                        call = Applied::Call(Callee::Closure(function), vec![argument]);
                    }
                    return Ok(call);
                }
            }
        }
        if let ExprKind::Selector(label) = &head.value {
            let record = self.value(args[0])?;
            let Some(index) = self.ty(args[0]).and_then(|ty| field_index(ty, label)) else {
                return Err(unsupported("selectors on records of unknown shape", expr));
            };
            function = Some(self.bind("", Op::Select(record, index)));
            rest = &args[1..];
        }
        // Anything else is a closure called one argument at a time.
        let mut call = Applied::Value(match function {
            Some(function) => function,
            None => self.value(head)?,
        });
        for argument in rest {
            let function = self.finish_call(call);
            let argument = self.value(argument)?;
            call = Applied::Call(Callee::Closure(function), vec![argument]);
        }
        Ok(call)
    }

    /// The value of an application, making its pending call.
    fn finish_call(&mut self, applied: Applied) -> Atom {
        match applied {
            Applied::Value(value) => value,
            Applied::Call(callee, args) => self.bind("", Op::Call(callee, args)),
        }
    }

    /// The built-in to use on an operand of type `operand`, for those that
    /// are overloaded.
    fn typed_builtin(&self, builtin: Builtin, operand: Option<&Ty>, expr: &Expr) -> Res<Builtin> {
        if !matches!(builtin, Builtin::Negate) {
            return Ok(builtin);
        }
        match operand {
            Some(ty) if ty.is("real") => Ok(Builtin::NegateReal),
            Some(ty) if ty.is("word") => Err(unsupported("word negation", expr)),
            _ => Ok(builtin),
        }
    }

    fn builtin(&mut self, builtin: Builtin, argument: Atom) -> Atom {
        match builtin {
            Builtin::Print => self.bind("", Op::Prim(Prim::Print, vec![argument])),
            Builtin::IntToString => self.bind("", Op::Prim(Prim::IntToString, vec![argument])),
            Builtin::Size => self.bind("", Op::Prim(Prim::Size, vec![argument])),
            Builtin::Not => self.bind(
                "",
                Op::Prim(Prim::WordEq, vec![argument, Atom::Word(value::FALSE)]),
            ),
            Builtin::Negate => self.bind("", Op::Prim(Prim::IntNeg, vec![argument])),
            Builtin::NegateReal => self.bind("", Op::Prim(Prim::RealNeg, vec![argument])),
            Builtin::Ref => self.bind("", Op::Prim(Prim::Ref, vec![argument])),
            Builtin::Deref => self.bind("", Op::Select(argument, 0)),
            Builtin::Equal | Builtin::Unequal => {
                let lhs = self.bind("", Op::Select(argument.clone(), 0));
                let rhs = self.bind("", Op::Select(argument, 1));
                let prim = match builtin {
                    Builtin::Equal => Prim::Equal,
                    _ => Prim::Unequal,
                };
                self.bind("", Op::Prim(prim, vec![lhs, rhs]))
            }
            Builtin::Assign => {
                let cell = self.bind("", Op::Select(argument.clone(), 0));
                let value = self.bind("", Op::Select(argument, 1));
                self.bind("", Op::Prim(Prim::Assign, vec![cell, value]))
            }
        }
    }

    fn construct(&mut self, test: &Test, argument: Atom) -> Atom {
        match test {
            // A cons cell is laid out as the pair it is built from.
            Test::Cons => argument,
            Test::Ref => self.bind("", Op::Prim(Prim::Ref, vec![argument])),
            Test::Boxed { tag: None, .. } => self.bind("", Op::Record(vec![argument])),
            Test::Boxed { tag: Some(tag), .. } => {
                self.bind("", Op::Record(vec![Atom::Word(*tag), argument]))
            }
            Test::Word { word, .. } => Atom::Word(*word),
            Test::String(_) => unreachable!("strings are not constructors"),
            Test::Exception { identity, .. } => {
                // The last field is where the value is first raised.
                let unraised = Atom::Word(value::tagged(0));
                self.bind("", Op::Record(vec![identity.clone(), argument, unraised]))
            }
        }
    }

    /// A closure for a constructor used as a function, as in `map SOME xs`.
    fn constructor_closure(&mut self, name: &str, test: &Test) -> Atom {
        let id = self.session.function_id();
        self.frames.push(Builder::new(id, name, vec!["env", ""]));
        let argument = Atom::Var(self.frame().params[1]);
        // An exception's identity is only known at run time, so the closure
        // holds it.
        let (test, captured) = match test {
            Test::Exception { identity, carries } => {
                let env = Atom::Var(self.frame().params[0]);
                let inner = self.bind("", Op::Select(env, 1));
                let test = Test::Exception {
                    identity: inner,
                    carries: *carries,
                };
                (test, vec![identity.clone()])
            }
            test => (test.clone(), Vec::new()),
        };
        let result = self.construct(&test, argument);
        self.terminate(Term::Return(result));
        let frame = self.frames.pop().expect("the constructor's frame");
        self.functions.push(frame.finish());
        self.closure(name, id, captured)
    }

    /// A closure for a built-in function used as a value.
    fn builtin_closure(&mut self, name: &str, builtin: Builtin) -> Atom {
        let id = self.session.function_id();
        self.frames.push(Builder::new(id, name, vec!["env", ""]));
        let argument = Atom::Var(self.frame().params[1]);
        let result = self.builtin(builtin, argument);
        self.terminate(Term::Return(result));
        let frame = self.frames.pop().expect("the builtin's frame");
        self.functions.push(frame.finish());
        self.closure(name, id, Vec::new())
    }

    /// A closure for `#label` used as a value: it selects field `index`.
    fn selector_closure(&mut self, label: &str, index: usize) -> Atom {
        let id = self.session.function_id();
        let name = format!("#{label}");
        self.frames.push(Builder::new(id, &name, vec!["env", ""]));
        let argument = Atom::Var(self.frame().params[1]);
        let result = self.bind("", Op::Select(argument, index));
        self.terminate(Term::Return(result));
        let frame = self.frames.pop().expect("the selector's frame");
        self.functions.push(frame.finish());
        self.closure(&name, id, Vec::new())
    }

    /// Allocates a closure of the function `code`.
    fn closure(&mut self, name: &str, code: FnId, captured: Vec<Atom>) -> Atom {
        let var = self.frame().var(name);
        self.emit(Stmt::Closures(vec![Closure {
            var,
            code,
            captured,
        }]));
        Atom::Var(var)
    }

    /// Compiles an anonymous function and allocates its closure.
    fn lambda(&mut self, name: &str, definition: &Definition) -> Res<Atom> {
        let id = self.session.function_id();
        let captures = self.function(id, definition)?;
        let captured = captures
            .into_iter()
            .map(|(depth, var)| Atom::Var(self.access(depth, var)))
            .collect();
        Ok(self.closure(name, id, captured))
    }
}

/// Where field `label` of a record of type `ty` is stored: fields are laid out
/// in label order, so a tuple and the record `{1 = _, 2 = _}` agree.
fn field_index(ty: &Ty, label: &str) -> Option<usize> {
    match ty {
        Ty::Record(labels) => labels.iter().position(|(known, _)| known == label),
        _ => None,
    }
}

fn strip_typed(expr: &Expr) -> &Expr {
    match &expr.value {
        ExprKind::Typed(inner, _) => strip_typed(inner),
        _ => expr,
    }
}

fn strip_typed_pattern(pattern: &Pat) -> &Pat {
    match &pattern.value {
        PatKind::Typed(inner, _) => strip_typed_pattern(inner),
        _ => pattern,
    }
}

fn unsupported(what: &str, expr: &Expr) -> Failure {
    (
        LowerError::Unsupported(what.to_string()),
        expr.source_span(),
    )
}

fn unsupported_pattern(what: &str, pattern: &Pat) -> Failure {
    (
        LowerError::Unsupported(what.to_string()),
        pattern.source_span(),
    )
}

fn unsupported_name(kind: &ExprKind) -> &'static str {
    match kind {
        ExprKind::Word(_) => "word literals",
        _ => "this expression",
    }
}
