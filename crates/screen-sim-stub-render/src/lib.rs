#![allow(clippy::identity_op)]

pub mod stubs;

mod web {
    pub use crate::stubs::*;
}

mod error {
    pub use crate::stubs::*;
}


include!(concat!(env!("OUT_DIR"), "/screen-sim-webgl-render-modules.rs"));