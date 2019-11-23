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
use crate::web_entrypoint::{print_error, web_load, web_run_frame, web_unload, InputOutput};
use app_error::AppResult;
use core::general_types::Size2D;
use core::simulation_core_state::{AnimationStep, FiltersPreset, Resources, VideoInputResources};
use render::simulation_render_state::VideoInputMaterials;
use std::str::FromStr;

#[wasm_bindgen]
pub struct WasmApp {
    res: Resources,
    io: Option<InputOutput>,
}

#[wasm_bindgen]
impl WasmApp {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        set_panic_hook();
        WasmApp {
            res: Resources::default(),
            io: None,
        }
    }

    #[wasm_bindgen]
    pub fn load(&mut self, webgl: JsValue, event_bus: JsValue, video_input: VideoInputConfig) {
        if let Some(_) = self.io {
            console!(error. "State already initialized!");
            return;
        }
        match web_load(&mut self.res, webgl, event_bus, video_input.resources, video_input.materials) {
            Ok(io) => self.io = Some(io),
            Err(e) => print_error(e),
        }
    }

    #[wasm_bindgen]
    pub fn run_frame(&mut self) -> bool {
        if let Some(ref mut io) = self.io {
            match web_run_frame(&mut self.res, io) {
                Ok(condition) => condition,
                Err(e) => {
                    print_error(e);
                    false
                }
            }
        } else {
            console!(error. "State not yet initialized!");
            false
        }
    }

    #[wasm_bindgen]
    pub fn unload(&mut self) {
        if let Some(io) = self.io.take() {
            handle_result(web_unload(io));
        }
    }
}

fn handle_result(result: AppResult<()>) {
    if let Err(e) = result {
        print_error(e);
    }
}

#[wasm_bindgen]
pub struct VideoInputConfig {
    resources: VideoInputResources,
    materials: VideoInputMaterials,
}

#[wasm_bindgen]
impl VideoInputConfig {
    #[wasm_bindgen(constructor)]
    pub fn new(image_width: u32, image_height: u32, canvas_width: u32, canvas_height: u32) -> VideoInputConfig {
        VideoInputConfig {
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
    pub fn set_max_texture_size(&mut self, max_texture_size: i32) {
        self.resources.max_texture_size = max_texture_size;
    }

    #[wasm_bindgen]
    pub fn set_drawing_activation(&mut self, activation: bool) {
        self.resources.drawing_activation = activation;
    }
}
