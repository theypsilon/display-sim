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

mod js {
    #[cfg(feature = "web")]
    pub use js_sys::*;

    #[cfg(feature = "native")]
    pub use native_stub::*;
}

mod web {
    #[cfg(feature = "web")]
    pub use web_sys::*;

    #[cfg(feature = "native")]
    pub use native_stub::*;
}

mod error {
    #[cfg(feature = "web")]
    pub use web_error::*;

    #[cfg(feature = "native")]
    pub use native_stub::*;
}

mod bindgen {
    #[cfg(feature = "web")]
    pub use wasm_bindgen::*;

    #[cfg(feature = "native")]
    pub use native_stub::*;
}