use clif2wasm::WasmModule;
use wasm_bindgen::prelude::*;

use crate::codegen::{Codegen, CodegenOptions, Symbols};
use crate::core;
use crate::printing::ReplValue;
use crate::session::{Backend, Execution, Session};
use crate::value;

#[wasm_bindgen(module = "/www/frontend/runtime.js")]
extern "C" {
    #[wasm_bindgen(js_name = NassauRuntime)]
    #[derive(Clone)]
    type Runtime;
    #[wasm_bindgen(constructor, js_class = NassauRuntime)]
    fn new() -> Runtime;
    #[wasm_bindgen(method, catch)]
    fn allocate_static(this: &Runtime, size: u32, align: u32) -> Result<u32, JsValue>;
    #[wasm_bindgen(method, catch)]
    fn instantiate(this: &Runtime, bytes: &[u8], names: JsValue) -> Result<(), JsValue>;
    #[wasm_bindgen(method, catch)]
    fn execute(this: &Runtime, entry: &str) -> Result<i32, JsValue>;
    #[wasm_bindgen(method)]
    fn read_word(this: &Runtime, address: u32) -> i64;
    #[wasm_bindgen(method)]
    fn read_real(this: &Runtime, address: u32) -> f64;
    #[wasm_bindgen(method)]
    fn read_bytes(this: &Runtime, address: u32, length: u32) -> Vec<u8>;
    #[wasm_bindgen(method)]
    fn builtin(this: &Runtime, index: u32) -> i64;
    #[wasm_bindgen(method)]
    fn take_uncaught(this: &Runtime) -> i64;
    #[wasm_bindgen(method)]
    fn exit_status(this: &Runtime) -> i32;
    #[wasm_bindgen(method)]
    fn retain_globals(this: &Runtime, addresses: JsValue);
    #[wasm_bindgen(method)]
    fn take_output(this: &Runtime) -> Vec<u8>;
    #[wasm_bindgen(method)]
    fn push_host(this: &Runtime, value: i64);
    #[wasm_bindgen(method)]
    fn pop_host(this: &Runtime);
}

fn js_error(error: JsValue) -> miette::Report {
    miette::Report::msg(
        js_sys::Error::from(error)
            .message()
            .as_string()
            .unwrap_or_else(|| "browser execution failed".into()),
    )
}

#[derive(Clone)]
struct WasmValue {
    word: i64,
    runtime: Runtime,
}

impl ReplValue for WasmValue {
    fn is_boxed(&self) -> bool {
        self.word != value::RAISED && self.word & 1 == 0
    }
    fn immediate(&self) -> i64 {
        self.word
    }
    fn field(&self, index: usize) -> Self {
        Self {
            word: self
                .runtime
                .read_word(self.word as u32 + (index as u32 + 1) * 8),
            runtime: self.runtime.clone(),
        }
    }
    fn length(&self) -> usize {
        (self.runtime.read_word(self.word as u32) >> 8) as usize
    }
    fn bytes(&self) -> Vec<u8> {
        self.runtime
            .read_bytes(self.word as u32 + 8, self.length() as u32)
    }
    fn real(&self) -> f64 {
        self.runtime.read_real(self.word as u32 + 8)
    }
    fn builtin(&self, name: &str) -> bool {
        value::builtin_exception(name)
            .is_some_and(|index| self.word == self.runtime.builtin(index as u32))
    }
    fn with_root<T>(&self, action: impl FnOnce() -> T) -> T {
        struct Root<'a>(&'a Runtime);
        impl Drop for Root<'_> {
            fn drop(&mut self) {
                self.0.pop_host();
            }
        }
        self.runtime.push_host(self.word);
        let _root = Root(&self.runtime);
        action()
    }
}

struct WasmBackend {
    codegen: Codegen,
    module: WasmModule,
    symbols: Symbols,
    runtime: Runtime,
}

impl WasmBackend {
    fn new() -> miette::Result<Self> {
        let mut module =
            WasmModule::new().map_err(|error| miette::Report::msg(error.to_string()))?;
        for name in [
            "nassau_alloc",
            "nassau_print",
            "nassau_exception",
            "nassau_raised",
            "nassau_uncaught",
            "nassau_exit",
            "nassau_roots_push",
            "nassau_roots_pop",
            "nassau_global_root",
            "nassau_code_global",
        ] {
            module.allow_import(name);
        }
        Ok(Self {
            codegen: Codegen::new(CodegenOptions::default()),
            module,
            symbols: Symbols::default(),
            runtime: Runtime::new(),
        })
    }

    fn value(&self, word: i64) -> WasmValue {
        WasmValue {
            word,
            runtime: self.runtime.clone(),
        }
    }
}

impl Backend for WasmBackend {
    type Value = WasmValue;

    fn execute(&mut self, module: core::Module) -> miette::Result<Execution<WasmValue>> {
        let mut target = self.module.clone();
        let mut symbols = self.symbols.clone();
        self.codegen
            .define_chunk(&mut target, &mut symbols, &module)
            .map_err(|error| miette::Report::msg(error.to_string()))?;
        let bytes = target
            .emit_incremental(|size, align| {
                self.runtime
                    .allocate_static(
                        u32::try_from(size)
                            .map_err(|error| clif2wasm::error::Error::general(error.to_string()))?,
                        align,
                    )
                    .map_err(|error| clif2wasm::error::Error::general(js_error(error).to_string()))
            })
            .map_err(|error| miette::Report::msg(format!("{error:#}")))?;
        let names = serde_wasm_bindgen::to_value(&target.function_names())
            .map_err(|error| miette::Report::msg(error.to_string()))?;
        self.runtime.instantiate(&bytes, names).map_err(js_error)?;
        self.module = target;
        self.symbols = symbols;
        let status = self
            .runtime
            .execute(&module.entry.name)
            .unwrap_or_else(|error| wasm_bindgen::throw_val(error));
        let exit = self.runtime.exit_status();
        let exception = self.runtime.take_uncaught();
        Ok(if exit >= 0 {
            Execution::Exit(exit as u8)
        } else if exception != value::RAISED {
            Execution::Raised(self.value(exception))
        } else {
            Execution::Returned(self.value(value::tagged(status as i64)))
        })
    }

    fn global(&self, id: core::GlobalId) -> Option<WasmValue> {
        let address = self.module.data_address(self.symbols.global_data(id)?)?;
        Some(self.value(self.runtime.read_word(address)))
    }

    fn retain_globals(&mut self, roots: &[core::GlobalId]) {
        let addresses: Vec<_> = roots
            .iter()
            .filter_map(|id| {
                self.symbols
                    .global_data(*id)
                    .and_then(|data| self.module.data_address(data))
            })
            .collect();
        self.runtime.retain_globals(
            serde_wasm_bindgen::to_value(&addresses).expect("global addresses serialize"),
        );
    }

    fn reset(&mut self) -> miette::Result<()> {
        *self = Self::new()?;
        Ok(())
    }

    fn take_output(&mut self) -> Vec<u8> {
        self.runtime.take_output()
    }
}

#[wasm_bindgen]
pub struct BrowserRepl {
    session: Session<WasmBackend>,
}

#[wasm_bindgen]
impl BrowserRepl {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<BrowserRepl, JsValue> {
        let backend = WasmBackend::new().map_err(|error| JsValue::from_str(&error.to_string()))?;
        Ok(Self {
            session: Session::new(backend),
        })
    }

    pub fn submit(&mut self, source: &str) -> Result<JsValue, JsValue> {
        serde_wasm_bindgen::to_value(&self.session.submit(source))
            .map_err(|error| JsValue::from_str(&error.to_string()))
    }

    pub fn completions(&self, prefix: &str) -> Result<JsValue, JsValue> {
        serde_wasm_bindgen::to_value(&self.session.completions(prefix))
            .map_err(|error| JsValue::from_str(&error.to_string()))
    }
}
