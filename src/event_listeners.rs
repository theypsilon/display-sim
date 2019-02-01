use wasm_bindgen::prelude::{JsValue, Closure};
use wasm_bindgen::JsCast;
use web_sys::{
    console,
    KeyboardEvent, MouseEvent, WheelEvent, EventTarget, CustomEvent
};
use std::rc::Rc;

use wasm_error::{WasmResult};
use web_utils::{window};
use simulation_state::{OwnedClosure, StateOwner, Input};

pub fn set_event_listeners(state_owner: &Rc<StateOwner>) -> WasmResult<Vec<OwnedClosure>> {

    let onblur: Closure<FnMut(JsValue)> = {
        let mut state_owner = Rc::clone(&state_owner);
        Closure::wrap(Box::new(move |_: JsValue| {
            let mut input = state_owner.input.borrow_mut();
            *input = Input::new().ok().unwrap();
        }))
    };

    let onkeydown: Closure<FnMut(JsValue)> = {
        let mut state_owner = Rc::clone(&state_owner);
        Closure::wrap(Box::new(move |event: JsValue| {
            if let Ok(e) = event.dyn_into::<KeyboardEvent>() {
                let mut input = state_owner.input.borrow_mut();
                on_button_action(&mut input, e.key().to_lowercase().as_ref(), true);
            }
        }))
    };

    let onkeyup: Closure<FnMut(JsValue)> = {
        let mut state_owner = Rc::clone(&state_owner);
        Closure::wrap(Box::new(move |event: JsValue| {
            if let Ok(e) = event.dyn_into::<KeyboardEvent>() {
                let mut input = state_owner.input.borrow_mut();
                on_button_action(&mut input, e.key().to_lowercase().as_ref(), false);
            }
        }))
    };

    let onmousedown: Closure<FnMut(JsValue)> = {
        let mut state_owner = Rc::clone(&state_owner);
        Closure::wrap(Box::new(move |event: JsValue| {
            if let Ok(e) = event.dyn_into::<MouseEvent>() {
                let mut input = state_owner.input.borrow_mut();
                input.mouse_left_click = e.buttons() == 1;
            }
        }))
    };

    let onmouseup: Closure<FnMut(JsValue)> = {
        let mut state_owner = Rc::clone(&state_owner);
        Closure::wrap(Box::new(move |event: JsValue| {
            if event.dyn_into::<MouseEvent>().is_ok() {
                let mut input = state_owner.input.borrow_mut();
                input.mouse_left_click = false;
            }
        }))
    };

    let onmousemove: Closure<FnMut(JsValue)> = {
        let mut state_owner = Rc::clone(&state_owner);
        Closure::wrap(Box::new(move |event: JsValue| {
            if let Ok(e) = event.dyn_into::<MouseEvent>() {
                let mut input = state_owner.input.borrow_mut();
                input.mouse_position_x = e.movement_x();
                input.mouse_position_y = e.movement_y();
            }
        }))
    };

    let onmousewheel: Closure<FnMut(JsValue)> = {
        let mut state_owner = Rc::clone(&state_owner);
        Closure::wrap(Box::new(move |event: JsValue| {
            if let Ok(e) = event.dyn_into::<WheelEvent>() {
                let mut input = state_owner.input.borrow_mut();
                input.mouse_scroll_y = e.delta_y() as f32;
            }
        }))
    };

    let oncustominputevent: Closure<FnMut(JsValue)> = {
        let mut state_owner = Rc::clone(&state_owner);
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

pub fn on_button_action(input: &mut Input, button_action: &str, pressed: bool) {
    match button_action {
        "a" => input.walk_left = pressed,
        "d" => input.walk_right = pressed,
        "w" => input.walk_forward = pressed,
        "s" => input.walk_backward = pressed,
        "q" => input.walk_up = pressed,
        "e" => input.walk_down = pressed,
        "arrowleft" | "←" | "◀" => input.turn_left = pressed,
        "arrowright" | "→" | "▶" => input.turn_right = pressed,
        "arrowup" | "↑" | "▲" => input.turn_up = pressed,
        "arrowdown" | "↓" | "▼" => input.turn_down = pressed,
        "+" => input.rotate_left = pressed,
        "-" => input.rotate_right = pressed,
        "f" => input.speed_up = pressed,
        "r" => input.speed_down = pressed,
        "t" => input.reset_speeds = pressed,
        "u" => input.increase_pixel_scale_x = pressed,
        "i" => input.decrease_pixel_scale_x = pressed,
        "j" => input.increase_pixel_scale_y = pressed,
        "k" => input.decrease_pixel_scale_y = pressed,
        "n" => input.increase_pixel_gap = pressed,
        "m" => input.decrease_pixel_gap = pressed,
        "b" => input.increase_blur = pressed,
        "v" => input.decrease_blur = pressed,
        "c" => input.increase_bright = pressed,
        "x" => input.decrease_bright = pressed,
        "y" => input.toggle_split_colors = pressed,
        "o" => input.toggle_pixels_render_kind = pressed,
        "p" => input.showing_pixels_pulse = pressed,
        "shift" => input.shift = pressed,
        "alt" => input.alt = pressed,
        " " | "space" => input.space = pressed,
        "escape" | "esc" => input.esc = pressed,
        "reset position" => input.reset_position = pressed,
        "reset filters" => input.reset_filters = pressed,
        _ => {
            if button_action.contains("+") {
                for button_fraction in button_action.split("+") {
                    on_button_action(input, button_fraction, pressed);
                }
            } else if pressed {
                console::log_2(&"Ignored key: ".into(), &button_action.into());
            }
        }
    }
}