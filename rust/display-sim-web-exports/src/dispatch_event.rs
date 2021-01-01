/* Copyright (c) 2019-2021 José manuel Barroso Galindo <theypsilon@gmail.com>
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

use app_error::AppResult;
use wasm_bindgen::{JsCast, JsValue};

pub fn dispatch_event(observer: &JsValue, kind: &str) -> AppResult<()> {
    dispatch_event_with(observer, kind, &"".into())
}

pub fn dispatch_event_with(observer: &JsValue, kind: &str, value: &JsValue) -> AppResult<()> {
    let event = js_sys::Object::new();
    js_sys::Reflect::set(&event, &"type".into(), &kind.into())?;
    let has_content = if let Some(string) = value.as_string() { &string != "" } else { true };
    if has_content {
        js_sys::Reflect::set(&event, &"message".into(), value)?;
    }
    dispatch_event_internal(observer, &event)
}

fn dispatch_event_internal(observer: &JsValue, event: &js_sys::Object) -> AppResult<()> {
    let fire = js_sys::Reflect::get(&observer, &"fire".into())?.dyn_into::<js_sys::Function>()?;
    let args = js_sys::Array::new();
    args.push(event);
    fire.apply(&observer, &args)?;
    Ok(())
}
