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

use core::app_events::{FakeEventDispatcher};
use core::general_types::Size2D;
use core::simulation_context::SimulationContext;
use core::simulation_core_state::{AnimationStep, Input, Resources, VideoInputResources};
use core::simulation_core_ticker::SimulationCoreTicker;
use render::background_render::BackgroundRender;
use render::blur_render::BlurRender;
use render::internal_resolution_render::InternalResolutionRender;
use render::pixels_render::PixelsRender;
use render::render_types::TextureBufferStack;
use render::rgb_render::RgbRender;
use render::simulation_draw::SimulationDrawer;
use render::simulation_render_state::{Materials, VideoInputMaterials};
use render::stubs::{WebGl2RenderingContext, WebResult};
use std::time::SystemTime;

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
                needs_buffer_data_load: true,
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
        let gl = WebGl2RenderingContext {};
        let mut materials = Materials {
            main_buffer_stack: TextureBufferStack::new(&gl),
            bg_buffer_stack: TextureBufferStack::new(&gl),
            pixels_render: PixelsRender::new(&gl, self.1)?,
            blur_render: BlurRender::new(&gl)?,
            internal_resolution_render: InternalResolutionRender::new(&gl)?,
            rgb_render: RgbRender::new(&gl)?,
            background_render: BackgroundRender::new(&gl)?,
            screenshot_pixels: None,
            gl,
        };

        let now = SystemTime::now();
        let mut input = Input::new(0.0);
        let mut ctx: SimulationContext<FakeEventDispatcher> = SimulationContext::default();
        for _ in 0..times {
            SimulationCoreTicker::new(&mut ctx, &mut res, &mut input).tick(now.elapsed().map_err(|e| e.to_string())?.as_millis() as f64 * 0.05);
            if res.quit {
                println!("User closed the simulation.");
                return Ok(());
            }
            if !res.drawable {
                continue;
            }
            SimulationDrawer::new(&mut ctx, &mut materials, &res).draw()?;
        }
        Ok(())
    }
}
