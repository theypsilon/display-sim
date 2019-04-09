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
    #[cfg(feature = "webgl_api")]
    pub use web_sys::*;

    #[cfg(feature = "webgl_stubs")]
    pub use webgl_stubs::*;

    #[cfg(feature = "webgl_to_sdl2")]
    pub use webgl_to_sdl2::*;
}

mod error {
    #[cfg(feature = "webgl_api")]
    pub use web_error::*;

    #[cfg(feature = "webgl_stubs")]
    pub use webgl_stubs::*;

    #[cfg(feature = "webgl_to_sdl2")]
    pub use webgl_to_sdl2::*;
}
