use wasm_bindgen::prelude::*;

use crate::session::Session;

#[wasm_bindgen]
#[derive(Default)]
pub struct BrowserRepl {
    session: Session,
}

#[wasm_bindgen]
impl BrowserRepl {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self::default()
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
