use console_error_panic_hook::set_once as set_panic_hook;

use js_sys::ArrayBuffer;
use wasm_bindgen::prelude::{wasm_bindgen, JsValue};

use crate::general_types::Size2D;
use crate::simulation_state::{AnimationMaterials, AnimationResources, AnimationStep, Resources};
use crate::web_entrypoint::{print_error, web_entrypoint};
use std::cell::RefCell;
use std::rc::Rc;

#[wasm_bindgen]
pub fn load_simulation_resources() -> ResourcesWasm {
    ResourcesWasm {
        data: Rc::new(RefCell::new(Resources::new())),
    }
}

#[wasm_bindgen]
pub fn run_program(gl: JsValue, res: &ResourcesWasm, animation: AnimationWasm) {
    set_panic_hook();
    if let Err(e) = web_entrypoint(gl, res.data.clone(), animation.resources, animation.materials) {
        print_error(e);
    }
}

#[wasm_bindgen]
pub struct ResourcesWasm {
    data: Rc<RefCell<Resources>>,
}

#[wasm_bindgen]
pub struct AnimationWasm {
    resources: AnimationResources,
    materials: AnimationMaterials,
}

#[wasm_bindgen]
impl AnimationWasm {
    #[allow(clippy::too_many_arguments)]
    #[wasm_bindgen(constructor)]
    pub fn new(image_width: u32, image_height: u32, background_width: u32, background_height: u32, canvas_width: u32, canvas_height: u32, pixel_width: f32, stretch: bool) -> AnimationWasm {
        AnimationWasm {
            resources: AnimationResources {
                image_size: Size2D {
                    width: image_width,
                    height: image_height,
                },
                background_size: Size2D {
                    width: background_width,
                    height: background_height,
                },
                viewport_size: Size2D {
                    width: canvas_width,
                    height: canvas_height,
                },
                steps: Vec::new(),
                pixel_width,
                stretch,
                current_frame: 0,
                last_frame_change: -100.0,
                needs_buffer_data_load: true,
            },
            materials: AnimationMaterials { buffers: Vec::new() },
        }
    }

    pub fn add(&mut self, buffer: ArrayBuffer, delay: u32) {
        self.resources.steps.push(AnimationStep { delay });
        self.materials.buffers.push(buffer);
    }
}
