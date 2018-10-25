use web_sys::{Event, EventTarget, CustomEvent, CustomEventInit};

use wasm_bindgen::*;
use wasm_error::{WasmError, Result};
use web_utils::{window};

pub fn dispatch_event(kind: &str) -> Result<()> {
    dispatch_event_internal(&Event::new(kind)?)
}

pub fn dispatch_event_with(kind: &str, value: &JsValue) -> Result<()> {
    let mut parameters = CustomEventInit::new();
    parameters.detail(&value);
    let event = CustomEvent::new_with_event_init_dict(kind, &parameters)?
    .dyn_into::<Event>()
    .ok()
    .ok_or("cannot make a custom event")?;
    dispatch_event_internal(&event)
}

fn dispatch_event_internal(event: &Event) -> Result<()> {
    window()?
    .dyn_into::<EventTarget>()
    .ok()
    .ok_or("cannot have even target")?
    .dispatch_event(&event)
    .map_err(WasmError::Js)
    .and_then(|result| 
        if result {
            Ok(())
        } else {
            Err(WasmError::Str("could not dispatch event".to_string()))
        }
    )
}