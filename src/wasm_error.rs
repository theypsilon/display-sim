pub enum WasmError {
    Js(wasm_bindgen::JsValue),
    Str(String)
}

impl WasmError {
    pub fn to_js(&self) -> wasm_bindgen::JsValue {
        match self { WasmError::Js(o) => o.clone(), WasmError::Str(s) => s.into()}
    }
}

impl From<std::string::String> for WasmError {
    fn from(string: std::string::String) -> Self {
        WasmError::Str(string)
    }
}

impl<'a> From<&'a str> for WasmError {
    fn from(string: &'a str) -> Self {
        WasmError::Str(string.into())
    }
}

impl From<wasm_bindgen::JsValue> for WasmError {
    fn from(o: wasm_bindgen::JsValue) -> Self {
        WasmError::Js(o)
    }
}

pub type Result<T> = std::result::Result<T, WasmError>;
