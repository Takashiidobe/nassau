use std::collections::{BTreeMap, BTreeSet};
use std::rc::Rc;

use boa_gc::{Finalize, Gc, GcRefCell, Trace};

use crate::core::{self, Atom, Callee, Failure, FnId, GlobalId, Op, Prim, Stmt, Term, Var};
use crate::value::{self, KIND_CLOSURE, KIND_REAL, KIND_RECORD, KIND_REF, KIND_STRING, NIL};

type Globals = BTreeMap<GlobalId, Gc<GcRefCell<Value>>>;

#[derive(Clone, Trace, Finalize)]
pub enum Value {
    Word(i64),
    Block(Gc<Object>),
}

#[derive(Trace, Finalize)]
pub struct Object {
    pub kind: i64,
    pub fields: GcRefCell<Vec<Value>>,
    pub bytes: Vec<u8>,
    pub real: f64,
    globals: Option<Gc<Globals>>,
}

impl Value {
    pub fn word(&self) -> i64 {
        match self {
            Self::Word(word) => *word,
            Self::Block(_) => panic!("expected an immediate"),
        }
    }

    pub fn object(&self) -> &Object {
        match self {
            Self::Block(object) => object,
            Self::Word(_) => panic!("expected a heap object"),
        }
    }

    pub fn field(&self, index: usize) -> Value {
        self.object().fields.borrow()[index].clone()
    }

    fn identical(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Word(a), Self::Word(b)) => a == b,
            (Self::Block(a), Self::Block(b)) => Gc::ptr_eq(a, b),
            _ => false,
        }
    }
}

fn object(kind: i64, fields: Vec<Value>, bytes: Vec<u8>, real: f64) -> Object {
    Object {
        kind,
        fields: GcRefCell::new(fields),
        bytes,
        real,
        globals: None,
    }
}

fn record(kind: i64, fields: Vec<Value>) -> Value {
    Value::Block(Gc::new(object(kind, fields, vec![], 0.0)))
}

fn string(bytes: Vec<u8>) -> Value {
    Value::Block(Gc::new(object(KIND_STRING, vec![], bytes, 0.0)))
}

fn literal(text: &str) -> Value {
    string(text.chars().map(|ch| ch as u8).collect())
}

fn real(number: f64) -> Value {
    Value::Block(Gc::new(object(KIND_REAL, vec![], vec![], number)))
}

pub enum Signal {
    Raised(Value),
    Exit(u8),
}

struct Code {
    function: core::Function,
    file: String,
}

struct Frame {
    code: Rc<Code>,
    globals: Gc<Globals>,
    vars: Vec<Value>,
    block: usize,
    statement: usize,
    waiting: Option<Var>,
}

impl Frame {
    fn new(code: Rc<Code>, globals: Gc<Globals>, args: Vec<Value>) -> Self {
        let mut vars = vec![Value::Word(NIL); code.function.vars.len()];
        for (param, arg) in code.function.params.iter().zip(args) {
            vars[*param] = arg;
        }
        Self {
            code,
            globals,
            vars,
            block: 0,
            statement: 0,
            waiting: None,
        }
    }

    fn jump(&mut self, block: usize, args: Vec<Value>) {
        for (param, arg) in self.code.function.blocks[block].params.iter().zip(args) {
            self.vars[*param] = arg;
        }
        self.block = block;
        self.statement = 0;
        self.waiting = None;
    }
}

enum Step {
    Next,
    Call(Frame, Var),
    Tail(Frame),
    Return(Value),
}

#[derive(Default)]
pub struct Interpreter {
    functions: BTreeMap<FnId, Rc<Code>>,
    dependencies: BTreeMap<FnId, BTreeSet<GlobalId>>,
    globals: Globals,
    exceptions: BTreeMap<i64, Value>,
    pub output: Vec<u8>,
}

impl Interpreter {
    pub fn global(&self, id: GlobalId) -> Option<Value> {
        self.globals.get(&id).map(|cell| cell.borrow().clone())
    }

    pub fn retain_globals(&mut self, roots: &[GlobalId]) {
        self.globals.retain(|id, _| roots.contains(id));
    }

    pub fn run(&mut self, module: core::Module) -> Result<Value, Signal> {
        let new_functions: Vec<_> = module.functions.iter().map(|f| f.id).collect();
        for function in module.functions {
            self.functions.insert(
                function.id,
                Rc::new(Code {
                    function,
                    file: module.file.clone(),
                }),
            );
        }
        for id in new_functions {
            self.dependencies.insert(id, self.global_dependencies(id));
        }
        for (id, _) in module.globals {
            self.globals
                .insert(id, Gc::new(GcRefCell::new(Value::Word(NIL))));
        }
        let entry = Rc::new(Code {
            function: module.entry,
            file: module.file,
        });
        let mut frames = vec![Frame::new(entry, Gc::new(self.globals.clone()), vec![])];
        loop {
            match self.step(frames.last_mut().expect("an active frame")) {
                Ok(Step::Next) => {}
                Ok(Step::Call(frame, var)) => {
                    frames.last_mut().unwrap().waiting = Some(var);
                    frames.push(frame);
                }
                Ok(Step::Tail(frame)) => *frames.last_mut().unwrap() = frame,
                Ok(Step::Return(value)) => {
                    frames.pop();
                    let Some(caller) = frames.last_mut() else {
                        return Ok(value);
                    };
                    caller.vars[caller.waiting.take().expect("a call result")] = value;
                }
                Err(Signal::Exit(status)) => return Err(Signal::Exit(status)),
                Err(Signal::Raised(exception)) => loop {
                    let Some(frame) = frames.last_mut() else {
                        return Err(Signal::Raised(exception));
                    };
                    if let Some(handler) = frame.code.function.blocks[frame.block].handler {
                        frame.jump(handler, vec![exception]);
                        break;
                    }
                    frames.pop();
                },
            }
        }
    }

    fn global_dependencies(&self, id: FnId) -> BTreeSet<GlobalId> {
        let mut globals = BTreeSet::new();
        let mut visited = BTreeSet::new();
        let mut pending = vec![id];
        while let Some(id) = pending.pop() {
            if !visited.insert(id) {
                continue;
            }
            for block in &self.functions[&id].function.blocks {
                for stmt in &block.stmts {
                    match stmt {
                        Stmt::SetGlobal(id, _) | Stmt::Let(_, Op::Global(id)) => {
                            globals.insert(*id);
                        }
                        Stmt::Let(_, Op::Call(Callee::Known(id, _), _)) => pending.push(*id),
                        Stmt::Closures(closures) => pending.extend(closures.iter().map(|c| c.code)),
                        _ => {}
                    }
                }
                if let Term::TailCall(Callee::Known(id, _), _) = &block.term {
                    pending.push(*id);
                }
            }
        }
        globals
    }

    fn atom(atom: &Atom, frame: &Frame) -> Value {
        match atom {
            Atom::Var(var) => frame.vars[*var].clone(),
            Atom::Word(word) => Value::Word(*word),
            Atom::String(text) => literal(text),
            Atom::Real(number) => real(*number),
        }
    }

    fn atoms(atoms: &[Atom], frame: &Frame) -> Vec<Value> {
        atoms.iter().map(|atom| Self::atom(atom, frame)).collect()
    }

    fn call(&self, callee: &Callee, args: &[Atom], frame: &Frame) -> Frame {
        let (id, env) = match callee {
            Callee::Known(id, env) => (*id, Self::atom(env, frame)),
            Callee::Closure(env) => {
                let env = Self::atom(env, frame);
                ((env.field(0).word() >> 1) as usize, env)
            }
        };
        let globals = match &env {
            Value::Block(object) if object.kind == KIND_CLOSURE => object.globals.clone().unwrap(),
            _ => frame.globals.clone(),
        };
        let mut values = vec![env];
        values.extend(Self::atoms(args, frame));
        Frame::new(self.functions[&id].clone(), globals, values)
    }

    fn step(&mut self, frame: &mut Frame) -> Result<Step, Signal> {
        let code = frame.code.clone();
        let block = &code.function.blocks[frame.block];
        if let Some(stmt) = block.stmts.get(frame.statement) {
            frame.statement += 1;
            match stmt {
                Stmt::Let(var, Op::Call(callee, args)) => {
                    return Ok(Step::Call(self.call(callee, args, frame), *var));
                }
                Stmt::Let(var, op) => {
                    frame.vars[*var] = match op {
                        Op::Atom(atom) => Self::atom(atom, frame),
                        Op::Prim(prim, args) => {
                            self.prim(*prim, &Self::atoms(args, frame), &code.file)?
                        }
                        Op::Record(fields) => record(KIND_RECORD, Self::atoms(fields, frame)),
                        Op::Select(atom, index) => Self::atom(atom, frame).field(*index),
                        Op::Global(id) => frame.globals[id].borrow().clone(),
                        Op::Call(..) => unreachable!(),
                    };
                }
                Stmt::SetGlobal(id, atom) => {
                    *frame.globals[id].borrow_mut() = Self::atom(atom, frame)
                }
                Stmt::Closures(closures) => {
                    for closure in closures {
                        let mut object = object(
                            KIND_CLOSURE,
                            vec![Value::Word(value::tagged(closure.code as i64))],
                            vec![],
                            0.0,
                        );
                        object.globals = Some(Gc::new(
                            self.dependencies[&closure.code]
                                .iter()
                                .filter_map(|id| {
                                    frame.globals.get(id).map(|cell| (*id, cell.clone()))
                                })
                                .collect(),
                        ));
                        frame.vars[closure.var] = Value::Block(Gc::new(object));
                    }
                    for closure in closures {
                        let captured = Self::atoms(&closure.captured, frame);
                        frame.vars[closure.var]
                            .object()
                            .fields
                            .borrow_mut()
                            .extend(captured);
                    }
                }
            }
            return Ok(Step::Next);
        }
        match &block.term {
            Term::Return(atom) => return Ok(Step::Return(Self::atom(atom, frame))),
            Term::Jump(target, args) => frame.jump(*target, Self::atoms(args, frame)),
            Term::If(condition, yes, no) => frame.jump(
                if Self::atom(condition, frame).word() != value::FALSE {
                    *yes
                } else {
                    *no
                },
                vec![],
            ),
            Term::TailCall(callee, args) => return Ok(Step::Tail(self.call(callee, args, frame))),
            Term::Fail(failure, location) => {
                return Err(self.builtin_raise(
                    match failure {
                        Failure::Match => "Match",
                        Failure::Bind => "Bind",
                    },
                    literal(location),
                ));
            }
            Term::Raise(atom, location) => {
                let exception = Self::atom(atom, frame);
                if exception.field(2).identical(&Value::Word(NIL))
                    && let Some(location) = location
                {
                    exception.object().fields.borrow_mut()[2] = literal(location);
                }
                return Err(Signal::Raised(exception));
            }
        }
        Ok(Step::Next)
    }

    fn exception(&mut self, index: i64) -> Value {
        self.exceptions
            .entry(index)
            .or_insert_with(|| {
                record(
                    KIND_REF,
                    vec![literal(value::BUILTIN_EXCEPTIONS[index as usize])],
                )
            })
            .clone()
    }

    fn builtin_raise(&mut self, name: &str, location: Value) -> Signal {
        let identity = self.exception(value::builtin_exception(name).unwrap());
        Signal::Raised(record(
            KIND_RECORD,
            vec![identity, Value::Word(NIL), location],
        ))
    }

    fn integer(&mut self, integer: i64, file: &str) -> Result<Value, Signal> {
        if value::int_fits(integer) {
            Ok(Value::Word(value::tagged(integer)))
        } else {
            Err(self.builtin_raise("Overflow", literal(&format!("<file {file}>"))))
        }
    }

    fn equal(left: &Value, right: &Value) -> bool {
        let mut pending = vec![(left.clone(), right.clone())];
        while let Some((left, right)) = pending.pop() {
            if left.identical(&right) {
                continue;
            }
            let (Value::Block(left), Value::Block(right)) = (&left, &right) else {
                return false;
            };
            if left.kind != right.kind {
                return false;
            }
            match left.kind {
                KIND_STRING if left.bytes == right.bytes => {}
                KIND_REAL if left.real == right.real => {}
                KIND_RECORD => {
                    let left = left.fields.borrow();
                    let right = right.fields.borrow();
                    if left.len() != right.len() {
                        return false;
                    }
                    pending.extend(left.iter().cloned().zip(right.iter().cloned()));
                }
                _ => return false,
            }
        }
        true
    }

    fn prim(&mut self, prim: Prim, args: &[Value], file: &str) -> Result<Value, Signal> {
        use Prim::*;
        let boolean = |yes| Value::Word(value::tagged(i64::from(yes)));
        let int = |i: usize| args[i].word() >> 1;
        let number = |i: usize| args[i].object().real;
        Ok(match prim {
            IntAdd => self.integer(int(0) + int(1), file)?,
            IntSub => self.integer(int(0) - int(1), file)?,
            IntMul => self.integer(int(0) * int(1), file)?,
            IntNeg => self.integer(-int(0), file)?,
            IntDiv | IntMod => {
                let (left, right) = (int(0), int(1));
                if right == 0 {
                    return Err(self.builtin_raise("Div", args[2].clone()));
                }
                let (quotient, remainder) = (left / right, left % right);
                let adjust = i64::from(remainder != 0 && (remainder < 0) != (right < 0));
                self.integer(
                    if prim == IntDiv {
                        quotient - adjust
                    } else {
                        remainder + adjust * right
                    },
                    file,
                )?
            }
            IntLt => boolean(int(0) < int(1)),
            IntLe => boolean(int(0) <= int(1)),
            IntGt => boolean(int(0) > int(1)),
            IntGe => boolean(int(0) >= int(1)),
            StringLt => boolean(args[0].object().bytes < args[1].object().bytes),
            StringLe => boolean(args[0].object().bytes <= args[1].object().bytes),
            StringGt => boolean(args[0].object().bytes > args[1].object().bytes),
            StringGe => boolean(args[0].object().bytes >= args[1].object().bytes),
            RealAdd => real(number(0) + number(1)),
            RealSub => real(number(0) - number(1)),
            RealMul => real(number(0) * number(1)),
            RealDiv => real(number(0) / number(1)),
            RealNeg => real(-number(0)),
            RealLt => boolean(number(0) < number(1)),
            RealLe => boolean(number(0) <= number(1)),
            RealGt => boolean(number(0) > number(1)),
            RealGe => boolean(number(0) >= number(1)),
            WordEq => boolean(args[0].identical(&args[1])),
            WordNe => boolean(!args[0].identical(&args[1])),
            Equal => boolean(Self::equal(&args[0], &args[1])),
            Unequal => boolean(!Self::equal(&args[0], &args[1])),
            IsBoxed => boolean(matches!(args[0], Value::Block(_))),
            Ref => record(KIND_REF, vec![args[0].clone()]),
            Assign => {
                args[0].object().fields.borrow_mut()[0] = args[1].clone();
                Value::Word(NIL)
            }
            Print => {
                self.output.extend(&args[0].object().bytes);
                Value::Word(NIL)
            }
            Concat => string(
                [
                    args[0].object().bytes.as_slice(),
                    args[1].object().bytes.as_slice(),
                ]
                .concat(),
            ),
            IntToString => string(int(0).to_string().replace('-', "~").into_bytes()),
            Size => Value::Word(value::tagged(args[0].object().bytes.len() as i64)),
            Exit => return Err(Signal::Exit((int(0) & 0xff) as u8)),
            BuiltinException => self.exception(int(0)),
        })
    }
}
