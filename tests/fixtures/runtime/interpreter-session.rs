use boa_gc::{WeakGc, force_collect};
use nassau::core::{Atom, Block, Callee, Closure, Function, Module, Op, Prim, Stmt, Term};
use nassau::interpreter::{Interpreter, Object, Signal, Value};
use nassau::value::NIL;

fn function(id: usize, params: Vec<usize>, stmts: Vec<Stmt>, term: Term) -> Function {
    Function {
        id,
        name: format!("fixture_{id}"),
        params,
        vars: vec![String::new(); 8],
        blocks: vec![Block {
            params: vec![],
            stmts,
            term,
            handler: None,
        }],
    }
}

fn run(interpreter: &mut Interpreter, module: Module) {
    match interpreter.run(module) {
        Ok(result) => assert_eq!(result.word(), NIL),
        Err(Signal::Raised(_)) => panic!("unexpected exception"),
        Err(Signal::Exit(status)) => panic!("unexpected exit {status}"),
    }
}

fn weak(value: Value) -> WeakGc<Object> {
    match &value {
        Value::Block(object) => WeakGc::new(object),
        Value::Word(_) => panic!("expected a heap object"),
    }
}

fn main() {
    let mut interpreter = Interpreter::default();
    let text = "saved global".repeat(1000);
    run(
        &mut interpreter,
        Module {
            file: "session.sml".into(),
            globals: vec![
                (0, "original".into()),
                (1, "saved".into()),
                (2, "cycle".into()),
            ],
            functions: vec![
                function(
                    1,
                    vec![0],
                    vec![Stmt::Let(
                        1,
                        Op::Call(Callee::Known(2, Atom::Var(0)), vec![]),
                    )],
                    Term::Return(Atom::Var(1)),
                ),
                function(
                    2,
                    vec![0],
                    vec![Stmt::Let(1, Op::Global(0))],
                    Term::Return(Atom::Var(1)),
                ),
            ],
            entry: function(
                0,
                vec![],
                vec![
                    Stmt::SetGlobal(0, Atom::String(text.clone())),
                    Stmt::Closures(vec![Closure {
                        var: 0,
                        code: 1,
                        captured: vec![],
                    }]),
                    Stmt::SetGlobal(1, Atom::Var(0)),
                    Stmt::Let(1, Op::Prim(Prim::Ref, vec![Atom::Word(NIL)])),
                    Stmt::Let(2, Op::Record(vec![Atom::Var(1)])),
                    Stmt::Let(3, Op::Prim(Prim::Assign, vec![Atom::Var(1), Atom::Var(2)])),
                    Stmt::SetGlobal(2, Atom::Var(1)),
                ],
                Term::Return(Atom::Word(NIL)),
            ),
        },
    );
    force_collect();
    let cycle = weak(interpreter.global(2).unwrap());
    let original = weak(interpreter.global(0).unwrap());
    let saved = weak(interpreter.global(1).unwrap());
    interpreter.retain_globals(&[1]);
    force_collect();
    assert!(
        cycle.upgrade().is_none(),
        "unreachable cycle was not reclaimed"
    );
    assert!(interpreter.global(0).is_none());
    assert!(
        original.upgrade().is_some(),
        "closure lost its historical global"
    );
    run(
        &mut interpreter,
        Module {
            file: "next.sml".into(),
            globals: vec![(3, "shadow".into()), (4, "result".into())],
            functions: vec![],
            entry: function(
                3,
                vec![],
                vec![
                    Stmt::SetGlobal(3, Atom::String("shadowed".into())),
                    Stmt::Let(0, Op::Global(1)),
                    Stmt::Let(1, Op::Call(Callee::Closure(Atom::Var(0)), vec![])),
                    Stmt::SetGlobal(4, Atom::Var(1)),
                ],
                Term::Return(Atom::Word(NIL)),
            ),
        },
    );
    assert_eq!(
        interpreter.global(4).unwrap().object().bytes,
        text.as_bytes()
    );
    interpreter.retain_globals(&[]);
    force_collect();
    assert!(
        original.upgrade().is_none(),
        "historical global was not reclaimed"
    );
    assert!(saved.upgrade().is_none(), "closure was not reclaimed");
    drop(interpreter);
    force_collect();
    assert!(cycle.upgrade().is_none());

    let mut session = nassau::session::Session::default();
    let response = session.submit("val kept = 41;\n");
    assert_eq!(response.output, b"val kept = 41 : int\n");
    let response = session.submit("val bad = true + 1; val after = kept + 1;\n");
    assert!(!response.diagnostics.is_empty());
    assert_eq!(response.output, b"val after = 42 : int\n");
    assert!(
        !session
            .submit("val bad = \"unfinished")
            .diagnostics
            .is_empty()
    );
    assert!(!session.submit("bad;\n").diagnostics.is_empty());
    assert!(session.submit("val cell = ref 0;\n").diagnostics.is_empty());
    let response = session.submit("val broken = (cell := 7; raise Fail \"stop\");\n");
    assert!(String::from_utf8_lossy(&response.output).contains("uncaught exception Fail"));
    assert!(!session.submit("broken;\n").diagnostics.is_empty());
    assert_eq!(
        session.submit("!cell + kept;\n").output,
        b"val it = 48 : int\n"
    );
    let response = session.submit(" clear ;; ");
    assert!(response.clear);
    assert_eq!(response.output, b"Cleared\n");
    assert!(response.diagnostics.is_empty());
    assert_eq!(session.submit("kept;;").output, b"val it = 41 : int\n");
    let response = session.submit("reset;;");
    assert!(response.clear);
    assert_eq!(response.output, b"Reset\n");
    assert!(!session.submit("kept;;").diagnostics.is_empty());
    assert!(!session.submit("cell;;").diagnostics.is_empty());
    assert_eq!(
        session.submit("val kept = 7;;").output,
        b"val kept = 7 : int\n"
    );
    assert_eq!(session.submit("kept;;").output, b"val it = 7 : int\n");
    let response =
        session.submit("val _ = Posix.Process.exit (Word8.fromInt 7); val unreachable = 1;\n");
    assert_eq!(response.exit, Some(7));
    assert!(response.output.is_empty());
    let response = session.submit("val ignored = 99;\n");
    assert_eq!(response.exit, Some(7));
    assert!(response.output.is_empty());
    assert_eq!(session.submit("reset;;").exit, None);
    assert_eq!(session.submit("42;;").output, b"val it = 42 : int\n");
    assert!(
        !nassau::session::Session::default()
            .submit("kept;\n")
            .diagnostics
            .is_empty()
    );
}
