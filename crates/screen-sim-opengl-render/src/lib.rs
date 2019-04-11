#![allow(clippy::identity_op)]

pub mod opengl_hooks;

mod web {
    pub use crate::opengl_hooks::*;
}

mod error {
    pub use crate::opengl_hooks::*;
}


include!(concat!(env!("OUT_DIR"), "/screen-sim-web-render-modules.rs"));