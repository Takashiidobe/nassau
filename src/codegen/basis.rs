use super::*;

#[cfg(feature = "native")]
mod compiled {
    include!(concat!(env!("OUT_DIR"), "/basis_symbols.rs"));
}

#[cfg(feature = "native")]
pub(super) fn symbols() -> Vec<(&'static str, *const u8)> {
    compiled::symbols()
}

impl Codegen {
    #[cfg(feature = "basis-bootstrap")]
    pub fn compile_basis(
        &self,
        module: &core::Module,
        triple: &str,
    ) -> Result<(Vec<u8>, String), CodegenError> {
        let mut flags = settings::builder();
        flags.set("opt_level", "speed").map_err(backend)?;
        flags
            .set("preserve_frame_pointers", "true")
            .map_err(backend)?;
        flags.set("is_pic", "true").map_err(backend)?;
        let isa = cranelift_codegen::isa::lookup(triple.parse().map_err(backend)?)
            .map_err(backend)?
            .finish(settings::Flags::new(flags))
            .map_err(backend)?;
        let builder =
            ObjectBuilder::new(isa, "nassau_basis", default_libcall_names()).map_err(backend)?;
        let mut target = ObjectModule::new(builder);
        let mut symbols = Symbols::default();
        self.define_module_with_linkage(&mut target, &mut symbols, module, Linkage::Export)?;
        let object = target.finish().emit().map_err(backend)?;
        let mut bindings = String::from("unsafe extern \"C\" {\n");
        let mut addresses = Vec::new();
        for function in &module.functions {
            let name = format!("nassau_f{}_{}", function.id, symbol_name(&function.name));
            bindings.push_str(&format!("#[link_name = {name:?}] fn f{}();\n", function.id));
            addresses.push(format!("({name:?}, f{} as *const u8)", function.id));
        }
        for (id, name) in &module.globals {
            let name = format!("nassau_global_{id}_{}", symbol_name(name));
            bindings.push_str(&format!("#[link_name = {name:?}] static mut g{id}: u64;\n"));
            addresses.push(format!("({name:?}, &raw const g{id} as *const u8)"));
        }
        bindings.push_str(&format!(
            "#[link_name = {:?}] fn entry() -> i32;\n}}\n",
            module.entry.name
        ));
        bindings.push_str(&format!(
            "pub fn symbols() -> Vec<(&'static str, *const u8)> {{ vec![{}] }}\n",
            addresses.join(",")
        ));
        let globals = module
            .globals
            .iter()
            .map(|(id, _)| format!("({id}, &raw const g{id})"))
            .collect::<Vec<_>>();
        bindings.push_str(&format!(
            "pub fn globals() -> [(usize, *const u64); {}] {{ [{}] }}\n",
            globals.len(),
            globals.join(",")
        ));
        let dependencies = module
            .functions
            .iter()
            .chain(std::iter::once(&module.entry))
            .map(|function| {
                let globals = symbols.dependencies[&function.id]
                    .iter()
                    .map(usize::to_string)
                    .collect::<Vec<_>>()
                    .join(",");
                format!("({}, &[{globals}])", function.id)
            })
            .collect::<Vec<_>>();
        bindings.push_str(&format!(
            "pub fn dependencies() -> Vec<(usize, &'static [usize])> {{ vec![{}] }}\n",
            dependencies.join(",")
        ));
        bindings.push_str("pub fn initialize() -> i32 { unsafe { for (_, address) in globals() { address.cast_mut().write(crate::value::NIL as u64); } entry() } }\n");
        Ok((object, bindings))
    }

    #[cfg(feature = "native")]
    pub fn run_basis(
        target: &mut JITModule,
        symbols: &mut Symbols,
        module: &core::Module,
    ) -> Result<i32, CodegenError> {
        for (global, name) in &module.globals {
            let id = target
                .declare_data(
                    &format!("nassau_global_{global}_{}", symbol_name(name)),
                    Linkage::Import,
                    true,
                    false,
                )
                .map_err(backend)?;
            symbols.globals.insert(*global, id);
        }
        symbols.linked_globals.extend(compiled::globals());
        for function in &module.functions {
            let id = target
                .declare_function(
                    &format!("nassau_f{}_{}", function.id, symbol_name(&function.name)),
                    Linkage::Import,
                    &sml_signature(function.params.len()),
                )
                .map_err(backend)?;
            symbols.functions.insert(function.id, id);
        }
        symbols.dependencies.extend(
            compiled::dependencies()
                .into_iter()
                .map(|(id, globals)| (id, globals.iter().copied().collect())),
        );
        Ok(compiled::initialize())
    }
}
