use wasm_bindgen::prelude::{JsValue, Closure};
use wasm_bindgen::JsCast;
use web_sys::{
    KeyboardEvent, MouseEvent, WheelEvent, EventTarget, CustomEvent
};
use std::rc::Rc;

use crate::wasm_error::{WasmResult};
use crate::web_utils::{window};
use crate::simulation_state::{OwnedClosure, StateOwner, Input};
use crate::action_bindings::on_button_action;

pub fn set_event_listeners(state_owner: &Rc<StateOwner>) -> WasmResult<Vec<OwnedClosure>> {

    let onblur: Closure<FnMut(JsValue)> = {
        let state_owner = Rc::clone(&state_owner);
        Closure::wrap(Box::new(move |_: JsValue| {
            let mut input = state_owner.input.borrow_mut();
            *input = Input::new().ok().unwrap();
        }))
    };

    let onkeydown: Closure<FnMut(JsValue)> = {
        let state_owner = Rc::clone(&state_owner);
        Closure::wrap(Box::new(move |event: JsValue| {
            if let Ok(e) = event.dyn_into::<KeyboardEvent>() {
                let mut input = state_owner.input.borrow_mut();
                on_button_action(&mut input, e.key().to_lowercase().as_ref(), true);
            }
        }))
    };

    let onkeyup: Closure<FnMut(JsValue)> = {
        let state_owner = Rc::clone(&state_owner);
        Closure::wrap(Box::new(move |event: JsValue| {
            if let Ok(e) = event.dyn_into::<KeyboardEvent>() {
                let mut input = state_owner.input.borrow_mut();
                on_button_action(&mut input, e.key().to_lowercase().as_ref(), false);
            }
        }))
    };

    let onmousedown: Closure<FnMut(JsValue)> = {
        let state_owner = Rc::clone(&state_owner);
        Closure::wrap(Box::new(move |event: JsValue| {
            if let Ok(e) = event.dyn_into::<MouseEvent>() {
                let mut input = state_owner.input.borrow_mut();
                input.mouse_left_click = e.buttons() == 1;
            }
        }))
    };

    let onmouseup: Closure<FnMut(JsValue)> = {
        let state_owner = Rc::clone(&state_owner);
        Closure::wrap(Box::new(move |event: JsValue| {
            if event.dyn_into::<MouseEvent>().is_ok() {
                let mut input = state_owner.input.borrow_mut();
                input.mouse_left_click = false;
            }
        }))
    };

    let onmousemove: Closure<FnMut(JsValue)> = {
        let state_owner = Rc::clone(&state_owner);
        Closure::wrap(Box::new(move |event: JsValue| {
            if let Ok(e) = event.dyn_into::<MouseEvent>() {
                let mut input = state_owner.input.borrow_mut();
                input.mouse_position_x = e.movement_x();
                input.mouse_position_y = e.movement_y();
            }
        }))
    };

    let onmousewheel: Closure<FnMut(JsValue)> = {
        let state_owner = Rc::clone(&state_owner);
        Closure::wrap(Box::new(move |event: JsValue| {
            if let Ok(e) = event.dyn_into::<WheelEvent>() {
                let mut input = state_owner.input.borrow_mut();
                input.mouse_scroll_y = e.delta_y() as f32;
            }
        }))
    };

    let oncustominputevent: Closure<FnMut(JsValue)> = {
        let state_owner = Rc::clone(&state_owner);
        Closure::wrap(Box::new(move |event: JsValue| {
            if let Ok(e) = event.dyn_into::<CustomEvent>() {
                let mut input = state_owner.input.borrow_mut();
                let object = e.detail();
                if let Ok(value) = js_sys::Reflect::get(&object, &"value".into()) {
                    input.custom_event.value = value;
                }
                if let Ok(value) = js_sys::Reflect::get(&object, &"kind".into()) {
                    if let Some(js_kind) = value.as_string() {
                        input.custom_event.kind = js_kind;
                    }
                }
            }
        }))
    };

    let window = window()?;
    window.set_onblur(Some(onblur.as_ref().unchecked_ref()));

    let document = window.document().ok_or("cannot access document")?;
    document.set_onkeydown(Some(onkeydown.as_ref().unchecked_ref()));
    document.set_onkeyup(Some(onkeyup.as_ref().unchecked_ref()));
    document.set_onmousedown(Some(onmousedown.as_ref().unchecked_ref()));
    document.set_onmouseup(Some(onmouseup.as_ref().unchecked_ref()));
    document.set_onmousemove(Some(onmousemove.as_ref().unchecked_ref()));
    document.set_onwheel(Some(onmousewheel.as_ref().unchecked_ref()));
    EventTarget::from(window).add_event_listener_with_callback("app-event.custom_input_event", oncustominputevent.as_ref().unchecked_ref())?;

    let mut closures: Vec<OwnedClosure> = vec!();
    closures.push(Some(onblur));
    closures.push(Some(onkeydown));
    closures.push(Some(onkeyup));
    closures.push(Some(onmousedown));
    closures.push(Some(onmouseup));
    closures.push(Some(onmousemove));
    closures.push(Some(onmousewheel));
    closures.push(Some(oncustominputevent));

    Ok(closures)
}