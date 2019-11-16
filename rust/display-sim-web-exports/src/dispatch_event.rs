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

use web_sys::{CustomEvent, CustomEventInit, Event, EventTarget};

use app_error::AppResult;
use wasm_bindgen::{JsCast, JsValue};

pub fn dispatch_event(event_bus: &EventTarget, kind: &str) -> AppResult<()> {
    dispatch_event_with(event_bus, kind, &"".into())
}

pub fn dispatch_event_with(event_bus: &EventTarget, kind: &str, value: &JsValue) -> AppResult<()> {
    let mut parameters = CustomEventInit::new();
    parameters.detail(
        &{
            let obj = js_sys::Object::new();
            js_sys::Reflect::set(&obj, &"type".into(), &kind.into())?;
            let has_content = if let Some(string) = value.as_string() { &string != "" } else { true };
            if has_content {
                js_sys::Reflect::set(&obj, &"message".into(), value)?;
            }
            obj
        }
        .into(),
    );

    let event = CustomEvent::new_with_event_init_dict("display-sim-event:backend-channel", &parameters)?
        .dyn_into::<Event>()
        .map_err(|_| "cannot make a custom event")?;
    dispatch_event_internal(event_bus, &event)
}

fn dispatch_event_internal(event_bus: &EventTarget, event: &Event) -> AppResult<()> {
    event_bus
        .dispatch_event(&event)
        .and_then(|success| if success { Ok(()) } else { Err("could not dispatch event".into()) })
        .map_err(Into::into)
}
