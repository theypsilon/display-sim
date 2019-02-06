use console_error_panic_hook::set_once as set_panic_hook;

use wasm_bindgen::prelude::{JsValue, wasm_bindgen};
use js_sys::ArrayBuffer;

use crate::simulation_program::program;
use crate::simulation_state::AnimationData;
use crate::wasm_error::WasmError;
use crate::console;

#[wasm_bindgen]
pub fn main(gl: JsValue, animation: AnimationWasm) {
    set_panic_hook();
    if let Err(e) = program(gl, animation.into_animation_data()) {
        match e {
            WasmError::Js(o) => console!(error. "An unexpected error ocurred.", o),
            WasmError::Str(s) => console!(error. "An unexpected error ocurred.", s),
        };
    }
}

#[wasm_bindgen]
pub struct AnimationWasm{data: AnimationData}

#[wasm_bindgen]
impl AnimationWasm {
    #[wasm_bindgen(constructor)]
    pub fn new(image_width: u32, image_height: u32,
        background_width: u32, background_height: u32,
        canvas_width: u32, canvas_height: u32,
        frame_length: f32, pixel_width: f32, stretch: bool) -> AnimationWasm
    {
        AnimationWasm{ data: AnimationData {
            image_width, image_height,
            background_width, background_height,
            viewport_width: canvas_width, viewport_height: canvas_height,
            steps: Vec::new(),
            frame_length,
            pixel_width,
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