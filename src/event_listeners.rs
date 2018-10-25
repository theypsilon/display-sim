use wasm_bindgen::prelude::{JsValue, Closure};
use wasm_bindgen::JsCast;
use web_sys::{
    console,
    KeyboardEvent, MouseEvent, WheelEvent, EventTarget, CustomEvent
};
use std::rc::Rc;
use std::cell::RefCell;

use wasm_error::{WasmResult};
use web_utils::{window};
use simulation_state::{Input, OwnedClosure};

pub fn set_event_listeners(input: &Rc<RefCell<Input>>) -> WasmResult<Vec<OwnedClosure>> {

    let onkeydown: Closure<FnMut(JsValue)> = {
        let mut input = Rc::clone(&input);
        Closure::wrap(Box::new(move |event: JsValue| {
            if let Ok(e) = event.dyn_into::<KeyboardEvent>() {
                let mut input = input.borrow_mut();
                match e.key().to_lowercase().as_ref() {
                    "a" => input.walk_left = true,
                    "d" => input.walk_right = true,
                    "w" => input.walk_forward = true,
                    "s" => input.walk_backward = true,
                    "q" => input.walk_up = true,
                    "e" => input.walk_down = true,
                    "arrowleft" => input.turn_left = true,
                    "arrowright" => input.turn_right = true,
                    "arrowup" => input.turn_up = true,
                    "arrowdown" => input.turn_down = true,
                    "+" => input.rotate_left = true,
                    "-" => input.rotate_right = true,
                    "f" => input.speed_up = true,
                    "r" => input.speed_down = true,
                    "t" => input.reset_speeds = true,
                    "u" => input.increase_pixel_scale_x = true,
                    "i" => input.decrease_pixel_scale_x = true,
                    "j" => input.increase_pixel_scale_y = true,
                    "k" => input.decrease_pixel_scale_y = true,
                    "n" => input.increase_pixel_gap = true,
                    "m" => input.decrease_pixel_gap = true,
                    "b" => input.increase_blur = true,
                    "v" => input.decrease_blur = true,
                    "c" => input.increase_bright = true,
                    "x" => input.decrease_bright = true,
                    "z" => input.reset_brightness = true,
                    "o" => input.toggle_pixels_render_kind = true,
                    "p" => input.showing_pixels_pulse = true,
                    "shift" => input.shift = true,
                    "alt" => input.alt = true,
                    " " => input.space = true,
                    "escape" => input.esc = true,
                    _ => console::log_2(&"down".into(), &e.key().into())
                }
            }
        }))
    };

    let onkeyup: Closure<FnMut(JsValue)> = {
        let mut input = Rc::clone(&input);
        Closure::wrap(Box::new(move |event: JsValue| {
            if let Ok(e) = event.dyn_into::<KeyboardEvent>() {
                let mut input = input.borrow_mut();
                match e.key().to_lowercase().as_ref() {
                    "a" => input.walk_left = false,
                    "d" => input.walk_right = false,
                    "w" => input.walk_forward = false,
                    "s" => input.walk_backward = false,
                    "q" => input.walk_up = false,
                    "e" => input.walk_down = false,
                    "arrowleft" => input.turn_left = false,
                    "arrowright" => input.turn_right = false,
                    "arrowup" => input.turn_up = false,
                    "arrowdown" => input.turn_down = false,
                    "+" => input.rotate_left = false,
                    "-" => input.rotate_right = false,
                    "f" => input.speed_up = false,
                    "r" => input.speed_down = false,
                    "t" => input.reset_speeds = false,
                    "u" => input.increase_pixel_scale_x = false,
                    "i" => input.decrease_pixel_scale_x = false,
                    "j" => input.increase_pixel_scale_y = false,
                    "k" => input.decrease_pixel_scale_y = false,
                    "n" => input.increase_pixel_gap = false,
                    "m" => input.decrease_pixel_gap = false,
                    "b" => input.increase_blur = false,
                    "v" => input.decrease_blur = false,
                    "c" => input.increase_bright = false,
                    "x" => input.decrease_bright = false,
                    "z" => input.reset_brightness = false,
                    "o" => input.toggle_pixels_render_kind = false,
                    "p" => input.showing_pixels_pulse = false,
                    "shift" => input.shift = false,
                    "alt" => input.alt = false,
                    " " => input.space = false,
                    "escape" => input.esc = false,
                    _ => console::log_2(&"up".into(), &e.key().into())
                }
            }
        }))
    };

    let onmousedown: Closure<FnMut(JsValue)> = {
        let mut input = Rc::clone(&input);
        Closure::wrap(Box::new(move |event: JsValue| {
            if let Ok(e) = event.dyn_into::<MouseEvent>() {
                let mut input = input.borrow_mut();
                input.mouse_left_click = e.buttons() == 1;
            }
        }))
    };

    let onmouseup: Closure<FnMut(JsValue)> = {
        let mut input = Rc::clone(&input);
        Closure::wrap(Box::new(move |event: JsValue| {
            if event.dyn_into::<MouseEvent>().is_ok() {
                let mut input = input.borrow_mut();
                input.mouse_left_click = false;
            }
        }))
    };

    let onmousemove: Closure<FnMut(JsValue)> = {
        let mut input = Rc::clone(&input);
        Closure::wrap(Box::new(move |event: JsValue| {
            if let Ok(e) = event.dyn_into::<MouseEvent>() {
                let mut input = input.borrow_mut();
                input.mouse_position_x = e.movement_x();
                input.mouse_position_y = e.movement_y();
            }
        }))
    };

    let onmousewheel: Closure<FnMut(JsValue)> = {
        let mut input = Rc::clone(&input);
        Closure::wrap(Box::new(move |event: JsValue| {
            if let Ok(e) = event.dyn_into::<WheelEvent>() {
                let mut input = input.borrow_mut();
                input.mouse_scroll_y = e.delta_y() as f32;
            }
        }))
    };

    let onpickcolor: Closure<FnMut(JsValue)> = {
        let mut input = Rc::clone(&input);
        Closure::wrap(Box::new(move |event: JsValue| {
            if let Ok(e) = event.dyn_into::<CustomEvent>() {
                let mut input = input.borrow_mut();
                let object = e.detail();
                if let Ok(value) = js_sys::Reflect::get(&object, &"color".into()) {
                    if let Some(js_color) = value.as_f64() {
                        input.color_value = js_color as i32;
                    }
                }
                if let Ok(value) = js_sys::Reflect::get(&object, &"kind".into()) {
                    if let Some(js_kind) = value.as_f64() {
                        input.color_kind = js_kind as i32;
                    }
                }
            }
        }))
    };

    let document = window()?.document().ok_or("cannot access document")?;
    document.set_onkeydown(Some(onkeydown.as_ref().unchecked_ref()));
    document.set_onkeyup(Some(onkeyup.as_ref().unchecked_ref()));
    document.set_onmousedown(Some(onmousedown.as_ref().unchecked_ref()));
    document.set_onmouseup(Some(onmouseup.as_ref().unchecked_ref()));
    document.set_onmousemove(Some(onmousemove.as_ref().unchecked_ref()));
    document.set_onwheel(Some(onmousewheel.as_ref().unchecked_ref()));
    EventTarget::from(window()?).add_event_listener_with_callback("app-event.pick_color", onpickcolor.as_ref().unchecked_ref())?;

    let mut closures: Vec<OwnedClosure> = vec!();
    closures.push(Some(onkeydown));
    closures.push(Some(onkeyup));
    closures.push(Some(onmousedown));
    closures.push(Some(onmouseup));
    closures.push(Some(onmousemove));
    closures.push(Some(onmousewheel));
    closures.push(Some(onpickcolor));

    Ok(closures)
}