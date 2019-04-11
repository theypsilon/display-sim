#![allow(clippy::identity_op)]

pub mod background_render;
pub mod blur_render;
pub mod internal_resolution_render;
pub mod pixels_render;
pub mod render_types;
pub mod rgb_render;
mod shaders;
pub mod simulation_draw;
pub mod simulation_render_state;

mod web {
    pub use web_sys::*;
}

mod error {
    pub use web_error::*;
}