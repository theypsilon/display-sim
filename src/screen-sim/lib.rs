#![allow(clippy::useless_attribute)]
#![allow(clippy::identity_op)]
#![allow(clippy::float_cmp)]

extern crate derive_new;

mod action_bindings;
mod app_events;
mod background_render;
mod blur_render;
mod boolean_button;
mod camera;
mod console;
mod dispatch_event;
mod general_types;
mod internal_resolution;
mod internal_resolution_render;
mod pixels_render;
mod pixels_shadow;
mod render_types;
mod rgb_render;
mod shaders;
mod simulation_context;
mod simulation_draw;
mod simulation_main_functions;
mod simulation_state;
mod simulation_update;
mod wasm_error;
pub mod wasm_exports;
mod web_entrypoint;
mod web_events;
mod web_utils;
