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

use crate::web_utils::window;
use wasm_bindgen::{JsCast, JsValue};
use web_error::{WebError, WebResult};

pub fn dispatch_event(kind: &str) -> WebResult<()> {
    dispatch_event_internal(&Event::new(kind)?)
}

pub fn dispatch_event_with(kind: &str, value: &JsValue) -> WebResult<()> {
    let mut parameters = CustomEventInit::new();
    parameters.detail(&value);
    let event = CustomEvent::new_with_event_init_dict(kind, &parameters)?
        .dyn_into::<Event>()
        .map_err(|_| "cannot make a custom event")?;
    dispatch_event_internal(&event)
}

fn dispatch_event_internal(event: &Event) -> WebResult<()> {
    window()?
        .dyn_into::<EventTarget>()
        .map_err(|_| "cannot have event target")?
        .dispatch_event(&event)
        .map_err(WebError::Js)
        .and_then(|success| {
            if success {
                Ok(())
            } else {
                Err(WebError::Str("could not dispatch event".to_string()))
            }
        })
}
