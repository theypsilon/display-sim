/* Copyright (c) 2019 José manuel Barroso Galindo <theypsilon@gmail.com>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>. */

#[derive(Clone, Debug)]
pub enum WebError {
    Js(wasm_bindgen::JsValue),
    Str(String),
}

impl WebError {
    pub fn into_js(self) -> wasm_bindgen::JsValue {
        match self {
            WebError::Js(o) => o,
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
