use console_error_panic_hook::set_once as set_panic_hook;

use js_sys::ArrayBuffer;
use wasm_bindgen::prelude::{wasm_bindgen, JsValue};

use crate::console;
use crate::simulation_program::program;
use crate::simulation_state::{AnimationData, AnimationStep, Resources};
use crate::wasm_error::WasmError;
use std::cell::RefCell;
use std::rc::Rc;

#[wasm_bindgen]
pub fn load_simulation_resources() -> ResourcesWasm {
    ResourcesWasm {
        data: Rc::new(RefCell::new(Resources::new())),
    }
}

#[wasm_bindgen]
pub fn run_program(gl: JsValue, res: &ResourcesWasm, animation: AnimationWasm) {
    set_panic_hook();
    if let Err(e) = program(gl, res.data.clone(), animation.into_animation_data()) {
        print_error(e);
    }
}

fn print_error(e: WasmError) {
    match e {
        WasmError::Js(o) => console!(error. "An unexpected error ocurred.", o),
        WasmError::Str(s) => console!(error. "An unexpected error ocurred.", s),
    };
}

#[wasm_bindgen]
pub struct ResourcesWasm {
    data: Rc<RefCell<Resources>>,
}

#[wasm_bindgen]
pub struct AnimationWasm {
    data: AnimationData,
}

#[wasm_bindgen]
impl AnimationWasm {
    #[allow(clippy::too_many_arguments)]
    #[wasm_bindgen(constructor)]
    pub fn new(image_width: u32, image_height: u32, background_width: u32, background_height: u32, canvas_width: u32, canvas_height: u32, pixel_width: f32, stretch: bool) -> AnimationWasm {
        AnimationWasm {
            data: AnimationData {
                image_width,
                image_height,
                background_width,
                background_height,
                viewport_width: canvas_width,
                viewport_height: canvas_height,
                steps: Vec::new(),
                pixel_width,
                stretch,
                current_frame: 0,
                last_frame_change: -100.0,
                needs_buffer_data_load: true,
            },
        }
    }

    pub fn add(&mut self, buffer: ArrayBuffer, delay: u32) {
        self.data.steps.push(AnimationStep { buffer, delay });
    }
}

trait IntoAnimationData {
    fn into_animation_data(self) -> AnimationData;
}

impl IntoAnimationData for AnimationWasm {
    fn into_animation_data(self) -> AnimationData {
        self.data
    }
}
