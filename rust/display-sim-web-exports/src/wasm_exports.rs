/* Copyright (c) 2019 José manuel Barroso Galindo <theypsilon@gmail.com>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>. */

use console_error_panic_hook::set_once as set_panic_hook;

use js_sys::Uint8Array;
use wasm_bindgen::prelude::{wasm_bindgen, JsValue};

use crate::console;
use crate::web_entrypoint::{print_error, web_entrypoint};
use core::general_types::Size2D;
use core::simulation_core_state::{AnimationStep, FiltersPreset, Resources, VideoInputResources};
use render::simulation_render_state::VideoInputMaterials;
use std::cell::RefCell;
use std::rc::Rc;
use std::str::FromStr;

#[wasm_bindgen]
pub fn load_simulation_resources() -> ResourcesWasm {
    ResourcesWasm {
        data: Rc::new(RefCell::new(Resources::default())),
    }
}

#[wasm_bindgen]
pub fn run_program(canvas: JsValue, res: &ResourcesWasm, video_input: VideoInputWasm) {
    set_panic_hook();
    if let Err(e) = web_entrypoint(canvas, res.data.clone(), video_input.resources, video_input.materials) {
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
impl VideoInputWasm {
    #[wasm_bindgen(constructor)]
    pub fn new(image_width: u32, image_height: u32, canvas_width: u32, canvas_height: u32) -> VideoInputWasm {
        VideoInputWasm {
            resources: VideoInputResources {
                image_size: Size2D {
                    width: image_width,
                    height: image_height,
                },
                background_size: Size2D {
                    width: image_width,
                    height: image_height,
                },
                viewport_size: Size2D {
                    width: canvas_width,
                    height: canvas_height,
                },
                preset: None,
                max_texture_size: 8192,
                steps: Vec::new(),
                pixel_width: 1.0,
                stretch: false,
                current_frame: 0,
                last_frame_change: -1000.0,
                needs_buffer_data_load: true,
                drawing_activation: true,
            },
            materials: VideoInputMaterials::default(),
        }
    }

    #[wasm_bindgen]
    pub fn set_background_size(&mut self, width: u32, height: u32) {
        self.resources.background_size.width = width;
        self.resources.background_size.height = height;
    }

    #[wasm_bindgen]
    pub fn add_picture_frame(&mut self, buffer: Uint8Array, delay: u32) {
        self.resources.steps.push(AnimationStep { delay });
        let mut pixels = vec![0; (self.resources.image_size.width * self.resources.image_size.height * 4) as usize].into_boxed_slice();
        buffer.copy_to(&mut *pixels);
        self.materials.buffers.push(pixels);
    }

    #[wasm_bindgen]
    pub fn set_pixel_width(&mut self, pixel_width: f32) {
        self.resources.pixel_width = pixel_width;
    }

    #[wasm_bindgen]
    pub fn set_preset(&mut self, preset: JsValue) {
        match preset.as_string() {
            Some(preset) => {
                if let Ok(preset) = FiltersPreset::from_str(preset.as_str()) {
                    self.resources.preset = Some(preset);
                } else {
                    console!(error. "Input preset is not a valid preset.");
                }
            }
            None => console!(error. "Input preset is not a valid string."),
        };
    }

    #[wasm_bindgen]
    pub fn stretch(&mut self) {
        self.resources.stretch = true;
    }

    #[wasm_bindgen]
    pub fn set_max_texture_size(&mut self, max_texture_size: i32) {
        self.resources.max_texture_size = max_texture_size;
    }

    #[wasm_bindgen]
    pub fn set_drawing_activation(&mut self, activation: bool) {
        self.resources.drawing_activation = activation;
    }
}
