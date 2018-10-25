extern crate wasm_bindgen;
extern crate js_sys;
extern crate web_sys;
extern crate nalgebra_glm as glm;
extern crate console_error_panic_hook;

pub mod wasm_exports;
mod wasm_error;
mod boolean_button;
mod simulation_program;
mod camera;
mod dispatch_event;
mod web_utils;
mod shaders;
mod pixels_render;
mod blur_render;
mod event_listeners;
mod simulation_state;