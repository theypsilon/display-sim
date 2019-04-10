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

#[macro_use]
extern crate cfg_if;

mod web {
    cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            pub use web_sys::*;
        } else if #[cfg(test)] {
            pub use webgl_stubs::*;
        } else if #[cfg(not(test))] {
            pub use webgl_to_sdl2::*;
        }
    }
}

mod error {
    cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            pub use web_error::*;
        } else if #[cfg(test)] {
            pub use webgl_stubs::*;
        } else if #[cfg(not(test))] {
            pub use webgl_to_sdl2::*;
        }
    }
}
