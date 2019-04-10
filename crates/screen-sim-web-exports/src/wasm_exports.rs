use console_error_panic_hook::set_once as set_panic_hook;

use js_sys::Uint8Array;
use wasm_bindgen::prelude::{wasm_bindgen, JsValue};

use crate::web_entrypoint::{print_error, web_entrypoint};
use core::general_types::Size2D;
use core::simulation_core_state::{AnimationStep, Resources, VideoInputResources};
use render::simulation_render_state::VideoInputMaterials;
use std::cell::RefCell;
use std::rc::Rc;

#[wasm_bindgen]
pub fn load_simulation_resources() -> ResourcesWasm {
    ResourcesWasm {
        data: Rc::new(RefCell::new(Resources::default())),
    }
}

#[wasm_bindgen]
pub fn run_program(gl: JsValue, res: &ResourcesWasm, video_input: VideoInputWasm) {
    set_panic_hook();
    if let Err(e) = web_entrypoint(gl, res.data.clone(), video_input.resources, video_input.materials) {
        print_error(e);
    }
}

#[wasm_bindgen]
pub struct ResourcesWasm {
    data: Rc<RefCell<Resources>>,
}

#[wasm_bindgen]
pub struct VideoInputWasm {
    resources: VideoInputResources,
    materials: VideoInputMaterials,
}

#[wasm_bindgen]
#[allow(clippy::too_many_arguments)]
pub fn new_video_input_wasm(
    image_width: u32,
    image_height: u32,
    background_width: u32,
    background_height: u32,
    canvas_width: u32,
    canvas_height: u32,
    pixel_width: f32,
    stretch: bool,
    max_texture_size: i32,
) -> VideoInputWasm {
    VideoInputWasm {
        resources: VideoInputResources {
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
            max_texture_size,
            steps: Vec::new(),
            pixel_width,
            stretch,
            current_frame: 0,
            last_frame_change: -1000.0,
            needs_buffer_data_load: true,
        },
        materials: VideoInputMaterials::default(),
    }
}

#[wasm_bindgen]
pub fn add_buffer_to_video_input(video_input: &mut VideoInputWasm, buffer: Uint8Array, delay: u32) {
    video_input.resources.steps.push(AnimationStep { delay });
    let mut pixels = vec![0; (video_input.resources.image_size.width * video_input.resources.image_size.height * 4) as usize].into_boxed_slice();
    buffer.copy_to(&mut *pixels);
    video_input.materials.buffers.push(pixels);
}
