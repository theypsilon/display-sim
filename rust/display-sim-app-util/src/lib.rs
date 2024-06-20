/* Copyright (c) 2019-2024 José manuel Barroso Galindo <theypsilon@gmail.com>
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

use std::{
    convert::Infallible,
    error::Error,
    fmt::{Display, Formatter, Result},
};

pub type AppResult<T> = std::result::Result<T, AppError>;

#[derive(Clone, Debug)]
pub struct AppError {
    err: String,
}

impl AppError {
    pub fn new(err: String) -> Self {
        AppError { err }
    }
}

impl Error for AppError {}

impl Display for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.err)
    }
}

#[cfg(target_arch = "wasm32")]
impl From<AppError> for wasm_bindgen::JsValue {
    fn from(e: AppError) -> Self {
        e.err.into()
    }
}

#[cfg(target_arch = "wasm32")]
impl From<wasm_bindgen::JsValue> for AppError {
    fn from(o: wasm_bindgen::JsValue) -> Self {
        AppError { err: format!("{:#?}", o) }
    }
}

impl From<std::string::String> for AppError {
    fn from(string: std::string::String) -> Self {
        AppError { err: string }
    }
}

impl From<Infallible> for AppError {
    fn from(_: Infallible) -> Self {
        AppError {
            err: "Infallible error?".into(),
        }
    }
}

impl<'a> From<&'a str> for AppError {
    fn from(string: &'a str) -> Self {
        AppError { err: string.into() }
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn log(string: &str) {
    println!("{}", string);
}

#[cfg(target_arch = "wasm32")]
pub fn log(string: &str) {
    web_sys::console::log_1(&(string.to_string()).into())
}

#[cfg(not(target_arch = "wasm32"))]
pub fn log_error(string: &dyn Display) {
    println!("ERROR: {}", string);
}

#[cfg(target_arch = "wasm32")]
pub fn log_error(string: &dyn Display) {
    web_sys::console::error_1(&(string.to_string()).into())
}
