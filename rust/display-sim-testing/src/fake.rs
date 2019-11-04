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

use core::app_events::FakeEventDispatcher;
use core::general_types::Size2D;
use core::simulation_context::{ConcreteSimulationContext, FakeRngGenerator};
use core::simulation_core_state::{AnimationStep, FiltersPreset, Input, Resources, VideoInputResources};
use core::simulation_core_ticker::SimulationCoreTicker;
use render::background_render::BackgroundRender;
use render::blur_render::BlurRender;
use render::error::WebResult;
use render::internal_resolution_render::InternalResolutionRender;
use render::pixels_render::PixelsRender;
use render::render_types::TextureBufferStack;
use render::rgb_render::RgbRender;
use render::simulation_draw::SimulationDrawer;
use render::simulation_render_state::{Materials, VideoInputMaterials};

use render::glow_test_stub::new_glow_stub;
use std::rc::Rc;
use std::time::SystemTime;

pub fn main() -> Result<(), String> {
    println!("Running 1.000.000.000.000.000 iterations!!\nTip: Better stop it at some point manually ;)");
    FakeVideoInput::default().iterate_times(1_000_000_000_000_000).map_err(|e| format!("{:?}", e))
}

pub struct FakeVideoInput(VideoInputResources, VideoInputMaterials);

impl Default for FakeVideoInput {
    fn default() -> FakeVideoInput {
        FakeVideoInput(
            VideoInputResources {
                steps: vec![AnimationStep { delay: 60 }],
                max_texture_size: 16000,
                image_size: Size2D { width: 256, height: 240 },
                background_size: Size2D { width: 256, height: 240 },
                viewport_size: Size2D { width: 256, height: 240 },
                pixel_width: 1.0,
                stretch: false,
                current_frame: 0,
                last_frame_change: 0.0,
                preset: FiltersPreset::default(),
                needs_buffer_data_load: true,
                drawing_activation: true,
            },
            VideoInputMaterials {
                buffers: vec![Box::new([0; 256 * 224 * 4 * 4])],
            },
        )
    }
}

impl FakeVideoInput {
    pub fn iterate_times(self, times: u128) -> WebResult<()> {
        let mut res = Resources::default();
        res.initialize(self.0, 0.0);
        let gl = Rc::new(new_glow_stub());
        let mut materials = Materials {
            main_buffer_stack: TextureBufferStack::new(gl.clone()),
            bg_buffer_stack: TextureBufferStack::new(gl.clone()),
            pixels_render: PixelsRender::new(gl.clone(), self.1)?,
            blur_render: BlurRender::new(gl.clone())?,
            internal_resolution_render: InternalResolutionRender::new(gl.clone())?,
            rgb_render: RgbRender::new(gl.clone())?,
            background_render: BackgroundRender::new(gl.clone())?,
            screenshot_pixels: None,
            gl,
        };

        let now = SystemTime::now();
        let mut input = Input::new(0.0);
        let ctx = ConcreteSimulationContext::new(FakeEventDispatcher {}, FakeRngGenerator {});
        for _ in 0..times {
            SimulationCoreTicker::new(&ctx, &mut res, &mut input).tick(now.elapsed().map_err(|e| e.to_string())?.as_millis() as f64 * 0.05);
            if res.quit {
                println!("User closed the simulation.");
                return Ok(());
            }
            if !res.drawable {
                continue;
            }
            SimulationDrawer::new(&ctx, &mut materials, &res).draw()?;
        }
        Ok(())
    }
}
