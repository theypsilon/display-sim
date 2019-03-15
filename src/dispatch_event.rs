use web_sys::{CustomEvent, CustomEventInit, Event, EventTarget};

use crate::wasm_error::{WasmError, WasmResult};
use crate::web_utils::window;
use wasm_bindgen::{JsCast, JsValue};

pub fn dispatch_event(kind: &str) -> WasmResult<()> {
    dispatch_event_internal(&Event::new(kind)?)
}

pub fn dispatch_event_with(kind: &str, value: &JsValue) -> WasmResult<()> {
    let mut parameters = CustomEventInit::new();
    parameters.detail(&value);
    let event = CustomEvent::new_with_event_init_dict(kind, &parameters)?.dyn_into::<Event>().map_err(|_| "cannot make a custom event")?;
    dispatch_event_internal(&event)
}

fn dispatch_event_internal(event: &Event) -> WasmResult<()> {
    window()?
        .dyn_into::<EventTarget>()
        .map_err(|_| "cannot have event target")?
        .dispatch_event(&event)
        .map_err(WasmError::Js)
        .and_then(|success| if success { Ok(()) } else { Err(WasmError::Str("could not dispatch event".to_string())) })
}
