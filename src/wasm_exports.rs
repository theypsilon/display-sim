use console_error_panic_hook::set_once as set_panic_hook;

use wasm_bindgen::prelude::*;
use web_sys::console;
use js_sys::ArrayBuffer;

use simulation_program::program;
use simulation_state::AnimationData;
use wasm_error::WasmError;

#[wasm_bindgen]
pub fn main(gl: JsValue, animation: AnimationWasm) {
    set_panic_hook();
    if let Err(e) = program(gl, animation.into_animation_data()) {
        match e {
            WasmError::Js(o) => console::error_2(&"An unexpected error ocurred.".into(), &o),
            WasmError::Str(s) => console::error_2(&"An unexpected error ocurred.".into(), &s.into()),
        };
    }
}

#[wasm_bindgen]
pub struct AnimationWasm{data: AnimationData}

#[wasm_bindgen]
impl AnimationWasm {
    #[wasm_bindgen(constructor)]
    pub fn new(width: u32, height: u32, canvas_width: u32, canvas_height: u32, frame_length: f32, scale_x: f32, stretch: bool) -> AnimationWasm {
        AnimationWasm{ data: AnimationData {
            steps: Vec::new(),
            width,
            height,
            canvas_width,
            canvas_height,
            frame_length,
            scale_x,
            stretch,
            current_frame: 1,
            last_frame_change: -100.0,
            needs_buffer_data_load: true,
        }}
    }

    pub fn add(&mut self, frame: ArrayBuffer) {
        self.data.steps.push(frame);
        self.data.current_frame = self.data.steps.len() + 1;
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