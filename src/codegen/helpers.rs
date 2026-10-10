use std::collections::HashSet;

use super::*;
use cranelift_codegen::ir::{StackSlotData, StackSlotKind};

const HELPERS: &[(&str, usize)] = &[
    ("nassau_concat", 2),
    ("nassau_int_to_string", 1),
    ("nassau_equal", 2),
];

impl Codegen {
    pub(super) fn define_helpers<M: Module>(
        &self,
        target: &mut M,
        symbols: &mut Symbols,
        module: &core::Module,
    ) -> Result<Vec<FunctionBuild>, CodegenError> {
        let needed: HashSet<_> = module
            .functions
            .iter()
            .chain(std::iter::once(&module.entry))
            .flat_map(|function| &function.blocks)
            .flat_map(|block| &block.stmts)
            .filter_map(|stmt| match stmt {
                Stmt::Let(_, Op::Prim(Prim::Concat, _)) => Some("nassau_concat"),
                Stmt::Let(_, Op::Prim(Prim::IntToString, _)) => Some("nassau_int_to_string"),
                Stmt::Let(_, Op::Prim(Prim::Equal | Prim::Unequal, _)) => Some("nassau_equal"),
                _ => None,
            })
            .collect();
        let helpers: Vec<_> = HELPERS
            .iter()
            .copied()
            .filter(|(name, _)| needed.contains(name) && !symbols.helpers.contains_key(name))
            .collect();
        for &(name, params) in &helpers {
            let mut signature = target.make_signature();
            signature
                .params
                .extend((0..params).map(|_| AbiParam::new(types::I64)));
            signature.returns.push(AbiParam::new(types::I64));
            let id = target
                .declare_function(name, Linkage::Local, &signature)
                .map_err(backend)?;
            symbols.helpers.insert(name, id);
        }
        let mut builds = Vec::new();
        for &(name, _) in &helpers {
            let start = Instant::now();
            let id = symbols.helpers[name];
            let frontend_config = target.target_config();
            let mut context = target.make_context();
            context.func.signature = target
                .declarations()
                .get_function_decl(id)
                .signature
                .clone();
            let mut builder_context = FunctionBuilderContext::new();
            let builder = FunctionBuilder::new(&mut context.func, &mut builder_context);
            let mut translator = Translator {
                target,
                symbols,
                builder,
                vars: HashMap::new(),
                blocks: Vec::new(),
                imports: HashMap::new(),
                entry: false,
                file: "",
                handler: None,
                landings: Vec::new(),
                roots: None,
                root_function: None,
            };
            let entry = translator.builder.create_block();
            translator
                .builder
                .append_block_params_for_function_params(entry);
            translator.builder.switch_to_block(entry);
            let args = translator.builder.block_params(entry).to_vec();
            translator.begin_roots(args.len() + 16)?;
            translator.publish_roots(&args);
            match name {
                "nassau_concat" => translator.concat(&args)?,
                "nassau_int_to_string" => translator.int_to_string(args[0])?,
                "nassau_equal" => translator.equal(&args)?,
                _ => unreachable!(),
            }
            translator.builder.seal_all_blocks();
            translator.builder.finalize(frontend_config);
            builds.push(self.finish_function(target, id, context, start.elapsed())?);
        }
        Ok(builds)
    }
}

impl<M: Module> Translator<'_, M> {
    pub(super) fn raised_address(&mut self) -> Result<Value, CodegenError> {
        Ok(self
            .call_c("nassau_raised", &[], Some(types::I64), &[])?
            .expect("nassau_raised returns the exception cell"))
    }

    pub(super) fn raise_exception(
        &mut self,
        exception: Value,
        location: Value,
    ) -> Result<(), CodegenError> {
        let position = self.load(exception, 2);
        let bit = self.builder.ins().band_imm_u(position, 1);
        let unset = self.builder.ins().icmp_imm_s(IntCC::NotEqual, bit, 0);
        let supplied = self.builder.ins().icmp_imm_s(IntCC::NotEqual, location, 0);
        let update = self.builder.ins().band(unset, supplied);
        let set = self.builder.create_block();
        let next = self.builder.create_block();
        self.builder.ins().brif(update, set, &[], next, &[]);
        self.builder.switch_to_block(set);
        self.store(location, exception, 24);
        self.builder.ins().jump(next, &[]);
        self.builder.switch_to_block(next);
        let state = self.raised_address()?;
        self.store(exception, state, 0);
        Ok(())
    }

    fn length(&mut self, string: Value) -> Value {
        let header = self.load_memory(types::I64, MemFlagsData::trusted(), string, 0);
        self.builder.ins().ushr_imm_u(header, 8)
    }

    fn allocate_string(&mut self, length: Value) -> Result<Value, CodegenError> {
        let kind = self.word(value::KIND_STRING);
        let block = self
            .call_c(
                "nassau_alloc",
                &[types::I64, types::I64],
                Some(types::I64),
                &[length, kind],
            )?
            .expect("nassau_alloc returns a pointer");
        Ok(block)
    }

    fn copy_bytes(&mut self, source: Value, destination: Value, length: Value) {
        let source = self.pointer(source);
        let destination = self.pointer(destination);
        let length = self.pointer(length);
        self.builder
            .call_memcpy(self.target.target_config(), destination, source, length);
    }

    fn concat(&mut self, args: &[Value]) -> Result<(), CodegenError> {
        let left = self.length(args[0]);
        let right = self.length(args[1]);
        let length = self.builder.ins().iadd(left, right);
        let string = self.allocate_string(length)?;
        let destination = self.builder.ins().iadd_imm_s(string, 8);
        let source = self.builder.ins().iadd_imm_s(args[0], 8);
        self.copy_bytes(source, destination, left);
        let destination = self.builder.ins().iadd(destination, left);
        let source = self.builder.ins().iadd_imm_s(args[1], 8);
        self.copy_bytes(source, destination, right);
        self.end_roots()?;
        self.builder.ins().return_(&[string]);
        Ok(())
    }

    fn int_to_string(&mut self, integer: Value) -> Result<(), CodegenError> {
        let slot = self.builder.create_sized_stack_slot(StackSlotData::new(
            StackSlotKind::ExplicitSlot,
            32,
            0,
        ));
        let buffer = self.stack_address(types::I64, slot, 0);
        let end = self.builder.ins().iadd_imm_s(buffer, 32);
        let integer = self.untag(integer);
        let negative = self
            .builder
            .ins()
            .icmp_imm_s(IntCC::SignedLessThan, integer, 0);
        let negated = self.builder.ins().ineg(integer);
        let magnitude = self.builder.ins().select(negative, negated, integer);
        let digits = self.builder.create_block();
        self.builder.append_block_param(digits, types::I64);
        self.builder.append_block_param(digits, types::I64);
        let sign = self.builder.create_block();
        let prefix = self.builder.create_block();
        let finish = self.builder.create_block();
        self.builder.append_block_param(finish, types::I64);
        self.builder
            .ins()
            .jump(digits, &[magnitude.into(), end.into()]);
        self.builder.switch_to_block(digits);
        let number = self.builder.block_params(digits)[0];
        let cursor = self.builder.block_params(digits)[1];
        let ten = self.word(10);
        let digit = self.builder.ins().urem(number, ten);
        let digit = self.builder.ins().iadd_imm_s(digit, 48);
        let digit = self.builder.ins().ireduce(types::I8, digit);
        let cursor = self.builder.ins().iadd_imm_s(cursor, -1);
        self.store_memory(MemFlagsData::trusted(), digit, cursor, 0);
        let rest = self.builder.ins().udiv(number, ten);
        let more = self.builder.ins().icmp_imm_s(IntCC::NotEqual, rest, 0);
        self.builder
            .ins()
            .brif(more, digits, &[rest.into(), cursor.into()], sign, &[]);
        self.builder.switch_to_block(sign);
        self.builder
            .ins()
            .brif(negative, prefix, &[], finish, &[cursor.into()]);
        self.builder.switch_to_block(prefix);
        let cursor = self.builder.ins().iadd_imm_s(cursor, -1);
        let tilde = self.builder.ins().iconst(types::I8, 126);
        self.store_memory(MemFlagsData::trusted(), tilde, cursor, 0);
        self.builder.ins().jump(finish, &[cursor.into()]);
        self.builder.switch_to_block(finish);
        let cursor = self.builder.block_params(finish)[0];
        let length = self.builder.ins().isub(end, cursor);
        let string = self.allocate_string(length)?;
        let destination = self.builder.ins().iadd_imm_s(string, 8);
        self.copy_bytes(cursor, destination, length);
        self.end_roots()?;
        self.builder.ins().return_(&[string]);
        Ok(())
    }

    fn equal(&mut self, args: &[Value]) -> Result<(), CodegenError> {
        let compare = self.builder.create_block();
        self.builder.append_block_param(compare, types::I64);
        self.builder.append_block_param(compare, types::I64);
        let boxed = self.builder.create_block();
        let headers = self.builder.create_block();
        let kinds = self.builder.create_block();
        let string = self.builder.create_block();
        let real = self.builder.create_block();
        let record = self.builder.create_block();
        let yes = self.builder.create_block();
        let no = self.builder.create_block();
        self.builder
            .ins()
            .jump(compare, &[args[0].into(), args[1].into()]);
        self.builder.switch_to_block(compare);
        let lhs = self.builder.block_params(compare)[0];
        let rhs = self.builder.block_params(compare)[1];
        let identical = self.builder.ins().icmp(IntCC::Equal, lhs, rhs);
        self.builder.ins().brif(identical, yes, &[], boxed, &[]);
        self.builder.switch_to_block(boxed);
        let bits = self.builder.ins().bor(lhs, rhs);
        let immediate = self.builder.ins().band_imm_u(bits, 1);
        self.builder.ins().brif(immediate, no, &[], headers, &[]);
        self.builder.switch_to_block(headers);
        let header = self.load_memory(types::I64, MemFlagsData::trusted(), lhs, 0);
        let other = self.load_memory(types::I64, MemFlagsData::trusted(), rhs, 0);
        let same = self.builder.ins().icmp(IntCC::Equal, header, other);
        self.builder.ins().brif(same, kinds, &[], no, &[]);
        self.builder.switch_to_block(kinds);
        let kind = self.builder.ins().band_imm_u(header, 0xff);
        let length = self.builder.ins().ushr_imm_u(header, 8);
        let mut switch = cranelift_frontend::Switch::new();
        switch.set_entry(value::KIND_STRING as u128, string);
        switch.set_entry(value::KIND_REAL as u128, real);
        switch.set_entry(value::KIND_RECORD as u128, record);
        switch.set_entry(value::KIND_VECTOR as u128, record);
        switch.emit(&mut self.builder, kind, no);
        self.builder.switch_to_block(string);
        let left = self.builder.ins().iadd_imm_s(lhs, 8);
        let right = self.builder.ins().iadd_imm_s(rhs, 8);
        let compared = self.compare_bytes(self.target.target_config(), left, right, length);
        let same = self.builder.ins().icmp_imm_s(IntCC::Equal, compared, 0);
        self.builder.ins().brif(same, yes, &[], no, &[]);
        self.builder.switch_to_block(real);
        let left = self.load_memory(types::F64, MemFlagsData::trusted(), lhs, 8);
        let right = self.load_memory(types::F64, MemFlagsData::trusted(), rhs, 8);
        let same = self.builder.ins().fcmp(FloatCC::Equal, left, right);
        self.builder.ins().brif(same, yes, &[], no, &[]);
        self.builder.switch_to_block(record);
        let empty = self.builder.ins().icmp_imm_s(IntCC::Equal, length, 0);
        let fields = self.builder.create_block();
        self.builder.append_block_param(fields, types::I64);
        let recurse = self.builder.create_block();
        let next_field = self.builder.create_block();
        let last_field = self.builder.create_block();
        let zero = self.word(0);
        self.builder
            .ins()
            .brif(empty, yes, &[], fields, &[zero.into()]);
        self.builder.switch_to_block(fields);
        let index = self.builder.block_params(fields)[0];
        let last = self.builder.ins().iadd_imm_s(length, -1);
        let is_last = self.builder.ins().icmp(IntCC::Equal, index, last);
        let offset = self.builder.ins().ishl_imm_u(index, 3);
        let left = self.builder.ins().iadd(lhs, offset);
        let right = self.builder.ins().iadd(rhs, offset);
        let left = self.load_memory(types::I64, MemFlagsData::trusted(), left, 8);
        let right = self.load_memory(types::I64, MemFlagsData::trusted(), right, 8);
        self.builder
            .ins()
            .brif(is_last, last_field, &[], recurse, &[]);
        self.builder.switch_to_block(last_field);
        self.builder
            .ins()
            .jump(compare, &[left.into(), right.into()]);
        self.builder.switch_to_block(recurse);
        let equal = self
            .call_c(
                "nassau_equal",
                &[types::I64, types::I64],
                Some(types::I64),
                &[left, right],
            )?
            .expect("nassau_equal returns a boolean");
        let same = self
            .builder
            .ins()
            .icmp_imm_s(IntCC::Equal, equal, value::TRUE);
        self.builder.ins().brif(same, next_field, &[], no, &[]);
        self.builder.switch_to_block(next_field);
        let next = self.builder.ins().iadd_imm_s(index, 1);
        self.builder.ins().jump(fields, &[next.into()]);
        self.builder.switch_to_block(yes);
        let yes = self.word(value::TRUE);
        self.end_roots()?;
        self.builder.ins().return_(&[yes]);
        self.builder.switch_to_block(no);
        let no = self.word(value::FALSE);
        self.end_roots()?;
        self.builder.ins().return_(&[no]);
        Ok(())
    }
}
