use web_sys::{CustomEvent, CustomEventInit, Event, EventTarget};

use wasm_bindgen::{JsCast, JsValue};
use web_error::{WebError, WebResult};
use crate::web_utils::window;

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