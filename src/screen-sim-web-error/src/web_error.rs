pub enum WebError {
    Js(wasm_bindgen::JsValue),
    Str(String),
}

impl WebError {
    pub fn to_js(&self) -> wasm_bindgen::JsValue {
        match self {
            WebError::Js(o) => o.clone(),
            WebError::Str(s) => s.into(),
        }
    }
}

impl From<std::string::String> for WebError {
    fn from(string: std::string::String) -> Self {
        WebError::Str(string)
    }
}

impl<'a> From<&'a str> for WebError {
    fn from(string: &'a str) -> Self {
        WebError::Str(string.into())
    }
}

impl From<wasm_bindgen::JsValue> for WebError {
    fn from(o: wasm_bindgen::JsValue) -> Self {
        WebError::Js(o)
    }
}

pub type WebResult<T> = std::result::Result<T, WebError>;
