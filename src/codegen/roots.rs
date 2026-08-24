use std::collections::BTreeSet;

use super::*;
use cranelift_codegen::ir::{StackSlotData, StackSlotKind};

type Live = BTreeSet<core::Var>;

fn atom(live: &mut Live, atom: &Atom) {
    if let Atom::Var(var) = atom {
        live.insert(*var);
    }
}

fn callee(live: &mut Live, callee: &Callee) {
    match callee {
        Callee::Closure(value) | Callee::Known(_, value) => atom(live, value),
    }
}

fn statement(live: &mut Live, stmt: &Stmt) {
    match stmt {
        Stmt::Let(var, op) => {
            live.remove(var);
            match op {
                Op::Atom(value) | Op::Select(value, _) => atom(live, value),
                Op::Prim(_, values) | Op::Record(values) => {
                    for value in values {
                        atom(live, value);
                    }
                }
                Op::Call(target, values) => {
                    callee(live, target);
                    for value in values {
                        atom(live, value);
                    }
                }
                Op::Global(_) => {}
            }
        }
        Stmt::SetGlobal(_, value) => atom(live, value),
        Stmt::Closures(closures) => {
            for closure in closures {
                for value in &closure.captured {
                    atom(live, value);
                }
            }
            for closure in closures {
                live.remove(&closure.var);
            }
        }
    }
}

fn terminator(term: &Term, entries: &[Live]) -> Live {
    let mut live = Live::new();
    match term {
        Term::Return(value) | Term::Raise(value, _) => atom(&mut live, value),
        Term::Jump(target, values) => {
            live.extend(&entries[*target]);
            for value in values {
                atom(&mut live, value);
            }
        }
        Term::If(value, yes, no) => {
            atom(&mut live, value);
            live.extend(&entries[*yes]);
            live.extend(&entries[*no]);
        }
        Term::TailCall(target, values) => {
            callee(&mut live, target);
            for value in values {
                atom(&mut live, value);
            }
        }
        Term::Fail(_, _) => {}
    }
    live
}

pub(super) fn live_roots(function: &core::Function) -> Vec<Vec<Live>> {
    let mut entries = vec![Live::new(); function.blocks.len()];
    loop {
        let mut changed = false;
        for (index, block) in function.blocks.iter().enumerate().rev() {
            let mut live = terminator(&block.term, &entries);
            if let Some(handler) = block.handler {
                live.extend(&entries[handler]);
            }
            for stmt in block.stmts.iter().rev() {
                statement(&mut live, stmt);
            }
            for param in &block.params {
                live.remove(param);
            }
            if live != entries[index] {
                entries[index] = live;
                changed = true;
            }
        }
        if !changed {
            break;
        }
    }
    function
        .blocks
        .iter()
        .map(|block| {
            let mut live = terminator(&block.term, &entries);
            if let Some(handler) = block.handler {
                live.extend(&entries[handler]);
            }
            let mut points = vec![live.clone()];
            for stmt in block.stmts.iter().rev() {
                statement(&mut live, stmt);
                points.push(live.clone());
            }
            points.reverse();
            points
        })
        .collect()
}

pub(super) struct RootFrame {
    address: Value,
    capacity: usize,
    temporary: usize,
}

impl<M: Module> Translator<'_, M> {
    pub(super) fn begin_roots(&mut self, capacity: usize) -> Result<(), CodegenError> {
        let bytes = u32::try_from((capacity + 3) * 8).map_err(backend)?;
        let slot = self.builder.create_sized_stack_slot(StackSlotData::new(
            StackSlotKind::ExplicitSlot,
            bytes,
            3,
        ));
        let address = self.builder.ins().stack_addr(types::I64, slot, 0);
        let zero = self.word(0);
        for field in 0..capacity {
            self.store(zero, address, ((field + 3) * 8) as i32);
        }
        let count = self.word(capacity as i64);
        let code = match self.root_function {
            Some(function) => self.builder.ins().func_addr(types::I64, function),
            None => self.word(0),
        };
        self.call_c(
            "nassau_roots_push",
            &[types::I64, types::I64, types::I64],
            None,
            &[address, count, code],
        )?;
        self.roots = Some(RootFrame {
            address,
            capacity,
            temporary: 0,
        });
        Ok(())
    }

    pub(super) fn publish_roots(&mut self, values: &[Value]) {
        let Some(frame) = &self.roots else {
            return;
        };
        let address = frame.address;
        let capacity = frame.capacity;
        assert!(values.len() <= capacity);
        let zero = self.word(0);
        for field in 0..capacity {
            self.store(
                values.get(field).copied().unwrap_or(zero),
                address,
                ((field + 3) * 8) as i32,
            );
        }
        self.roots.as_mut().unwrap().temporary = values.len();
    }

    pub(super) fn root_temporary(&mut self, value: Value) {
        if let Some(frame) = &mut self.roots {
            assert!(frame.temporary < frame.capacity);
            let address = frame.address;
            let offset = ((frame.temporary + 3) * 8) as i32;
            frame.temporary += 1;
            self.store(value, address, offset);
        }
    }

    pub(super) fn end_roots(&mut self) -> Result<(), CodegenError> {
        if let Some(frame) = &self.roots {
            let address = frame.address;
            self.call_c("nassau_roots_pop", &[types::I64], None, &[address])?;
        }
        Ok(())
    }
}

pub(super) fn global_dependencies(module: &core::Module, symbols: &mut Symbols) {
    let mut calls = HashMap::new();
    for function in module
        .functions
        .iter()
        .chain(std::iter::once(&module.entry))
    {
        let mut globals = BTreeSet::new();
        let mut functions = BTreeSet::new();
        for block in &function.blocks {
            for stmt in &block.stmts {
                match stmt {
                    Stmt::Let(_, Op::Global(global)) | Stmt::SetGlobal(global, _) => {
                        globals.insert(*global);
                    }
                    Stmt::Let(_, Op::Call(Callee::Known(id, _), _)) => {
                        functions.insert(*id);
                    }
                    Stmt::Closures(closures) => {
                        functions.extend(closures.iter().map(|closure| closure.code));
                    }
                    _ => {}
                }
            }
            if let Term::TailCall(Callee::Known(id, _), _) = &block.term {
                functions.insert(*id);
            }
        }
        symbols.dependencies.insert(function.id, globals);
        calls.insert(function.id, functions);
    }
    loop {
        let mut changed = false;
        for (id, targets) in &calls {
            let mut globals = symbols.dependencies[id].clone();
            for target in targets {
                globals.extend(&symbols.dependencies[target]);
            }
            if globals != symbols.dependencies[id] {
                symbols.dependencies.insert(*id, globals);
                changed = true;
            }
        }
        if !changed {
            break;
        }
    }
    symbols.pending_globals = module.globals.iter().map(|(id, _)| *id).collect();
    symbols.pending_functions = module
        .functions
        .iter()
        .chain(std::iter::once(&module.entry))
        .map(|function| function.id)
        .collect();
}

impl<M: Module> Translator<'_, M> {
    pub(super) fn register_module_roots(&mut self) -> Result<(), CodegenError> {
        for global in self.symbols.pending_globals.clone() {
            let address = self.data_address(self.symbols.globals[&global]);
            self.call_c("nassau_global_root", &[types::I64], None, &[address])?;
        }
        for function in self.symbols.pending_functions.clone() {
            let reference = self
                .target
                .declare_func_in_func(self.symbols.functions[&function], self.builder.func);
            let code = self.builder.ins().func_addr(types::I64, reference);
            for global in self.symbols.dependencies[&function].clone() {
                let address = self.data_address(self.symbols.globals[&global]);
                self.call_c(
                    "nassau_code_global",
                    &[types::I64, types::I64],
                    None,
                    &[code, address],
                )?;
            }
        }
        Ok(())
    }
}
