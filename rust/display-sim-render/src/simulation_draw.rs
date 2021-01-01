/* Copyright (c) 2019-2021 José manuel Barroso Galindo <theypsilon@gmail.com>
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

use crate::error::AppResult;
use crate::pixels_render::PixelsUniform;
use crate::simulation_render_state::Materials;
use core::simulation_context::SimulationContext;
use core::simulation_core_state::Resources;
use core::ui_controller::{color_channels::ColorChannelsOptions, texture_interpolation::TextureInterpolationOptions};

use glow::GlowSafeAdapter;

pub struct SimulationDrawer<'a> {
    #[allow(dead_code)]
    ctx: &'a dyn SimulationContext,
    materials: &'a mut Materials,
    res: &'a Resources,
}

impl<'a> SimulationDrawer<'a> {
    pub fn new(ctx: &'a dyn SimulationContext, materials: &'a mut Materials, res: &'a Resources) -> Self {
        materials.gl.enable(glow::DEPTH_TEST);
        SimulationDrawer { ctx, materials, res }
    }

    pub fn draw(&mut self) -> AppResult<()> {
        if !self.res.video.drawing_activation {
            return Ok(());
        }

        let filters = &self.res.controllers;
        let output = &self.res.main.render;

        let materials = &mut self.materials;
        let gl = &materials.gl;

        let resolution_width = filters.internal_resolution.width();
        let resolution_height = filters.internal_resolution.height();

        let viewport_width = self.res.video.viewport_size.width;
        let viewport_height = self.res.video.viewport_size.height;

        if self.res.video.needs_buffer_data_load {
            materials.pixels_render.load_image(&self.res.video);
        }

        materials.main_buffer_stack.set_depthbuffer(output.pixel_have_depth)?;
        materials.main_buffer_stack.set_resolution(resolution_width, resolution_height)?;
        materials.main_buffer_stack.set_interpolation(match filters.texture_interpolation.value {
            TextureInterpolationOptions::Linear => glow::LINEAR,
            TextureInterpolationOptions::Nearest => glow::NEAREST,
        })?;

        materials.main_buffer_stack.push()?;
        materials.main_buffer_stack.push()?;
        materials.main_buffer_stack.bind_current()?;

        gl.clear_color(0.0, 0.0, 0.0, 0.0);
        gl.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);

        let view = self.res.camera.get_view();
        let position = self.res.camera.get_position();

        let projection = if self.res.screenshot_trigger.is_triggered {
            self.res.camera.get_projection(resolution_width as f32, resolution_height as f32)
        } else {
            self.res.camera.get_projection(viewport_width as f32, viewport_height as f32)
        };

        for hl_idx in 0..filters.horizontal_lpp.value {
            for vl_idx in 0..filters.vertical_lpp.value {
                for color_idx in 0..output.color_splits {
                    if let ColorChannelsOptions::Overlapping = filters.color_channels.value {
                        materials.main_buffer_stack.push()?;
                        materials.main_buffer_stack.bind_current()?;
                        if vl_idx == 0 && hl_idx == 0 {
                            gl.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);
                        }
                    }
                    materials.pixels_render.render(PixelsUniform {
                        shadow_kind: filters.pixel_shadow_shape_kind.value.value,
                        geometry_kind: filters.pixels_geometry_kind.value,
                        view: &matrix_to_16_f32(view),
                        projection: &matrix_to_16_f32(projection),
                        ambient_strength: output.ambient_strength,
                        contrast_factor: filters.extra_contrast.value,
                        light_color: &output.light_color[color_idx],
                        extra_light: &output.extra_light,
                        light_pos: &vec_to_3_f32(position),
                        screen_curvature: output.screen_curvature_factor,
                        pixel_spread: &output.pixel_spread,
                        pixel_scale: &output
                            .pixel_scale_foreground
                            .get(vl_idx * filters.horizontal_lpp.value + hl_idx)
                            .expect("Bad pixel_scale_foreground")[color_idx],
                        pixel_pulse: output.pixels_pulse,
                        pixel_offset: &output
                            .pixel_offset_foreground
                            .get(vl_idx * filters.horizontal_lpp.value + hl_idx)
                            .expect("Bad pixel_offset_foreground")[color_idx],
                        rgb_red: &output.rgb_red,
                        rgb_green: &output.rgb_green,
                        rgb_blue: &output.rgb_blue,
                        color_gamma: output.color_gamma,
                        color_noise: output.color_noise,
                        time: output.time as f32,
                        height_modifier_factor: output.height_modifier_factor,
                    });
                }
                if let ColorChannelsOptions::Overlapping = filters.color_channels.value {
                    materials.main_buffer_stack.pop()?;
                    materials.main_buffer_stack.pop()?;
                    materials.main_buffer_stack.pop()?;
                }
            }
        }

        if let ColorChannelsOptions::Overlapping = filters.color_channels.value {
            materials.main_buffer_stack.bind_current()?;
            gl.active_texture(glow::TEXTURE0 + 0);
            gl.bind_texture(glow::TEXTURE_2D, materials.main_buffer_stack.get_nth(1)?.texture());
            gl.active_texture(glow::TEXTURE0 + 1);
            gl.bind_texture(glow::TEXTURE_2D, materials.main_buffer_stack.get_nth(2)?.texture());
            gl.active_texture(glow::TEXTURE0 + 2);
            gl.bind_texture(glow::TEXTURE_2D, materials.main_buffer_stack.get_nth(3)?.texture());

            materials.rgb_render.render();

            gl.active_texture(glow::TEXTURE0 + 0);
        }

        materials.main_buffer_stack.push()?;
        materials.main_buffer_stack.bind_current()?;
        gl.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);

        if output.showing_background {
            materials.bg_buffer_stack.set_resolution(1920 / 2, 1080 / 2)?;
            materials.bg_buffer_stack.set_depthbuffer(false)?;
            materials.bg_buffer_stack.set_interpolation(glow::LINEAR)?;
            materials.bg_buffer_stack.push()?;
            materials.bg_buffer_stack.bind_current()?;
            gl.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);
            for hl_idx in 0..filters.horizontal_lpp.value {
                for vl_idx in 0..filters.vertical_lpp.value {
                    materials.pixels_render.render(PixelsUniform {
                        shadow_kind: 0,
                        geometry_kind: filters.pixels_geometry_kind.value,
                        view: &matrix_to_16_f32(view),
                        projection: &matrix_to_16_f32(projection),
                        ambient_strength: output.ambient_strength,
                        contrast_factor: filters.extra_contrast.value,
                        light_color: &output.light_color_background,
                        extra_light: &[0.0, 0.0, 0.0],
                        light_pos: &vec_to_3_f32(position),
                        pixel_spread: &output.pixel_spread,
                        pixel_scale: &output.pixel_scale_background[vl_idx * filters.horizontal_lpp.value + hl_idx],
                        screen_curvature: output.screen_curvature_factor,
                        pixel_pulse: output.pixels_pulse,
                        pixel_offset: &output.pixel_offset_background[vl_idx * filters.horizontal_lpp.value + hl_idx],
                        rgb_red: &output.rgb_red,
                        rgb_green: &output.rgb_green,
                        rgb_blue: &output.rgb_blue,
                        color_gamma: output.color_gamma,
                        color_noise: output.color_noise,
                        time: output.time as f32,
                        height_modifier_factor: 0.0,
                    });
                }
            }
            let source = (*materials.bg_buffer_stack.get_current()?).clone();
            let target = materials.main_buffer_stack.get_current()?;
            materials.blur_render.render(&mut materials.bg_buffer_stack, &source, &target, 6)?;
            materials.bg_buffer_stack.pop()?;
        }
        materials.main_buffer_stack.pop()?;
        materials.main_buffer_stack.pop()?;
        materials.main_buffer_stack.bind_current()?;
        gl.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);

        gl.active_texture(glow::TEXTURE0 + 0);
        gl.bind_texture(glow::TEXTURE_2D, materials.main_buffer_stack.get_nth(1)?.texture());
        gl.active_texture(glow::TEXTURE0 + 1);
        gl.bind_texture(glow::TEXTURE_2D, materials.main_buffer_stack.get_nth(2)?.texture());
        materials.background_render.render();
        gl.active_texture(glow::TEXTURE0 + 0);

        if filters.blur_passes.value > 0 {
            let target = materials.main_buffer_stack.get_current()?.clone();
            materials
                .blur_render
                .render(&mut materials.main_buffer_stack, &target, &target, filters.blur_passes.value)?;
        }

        materials.screenshot_pixels = None;

        if self.res.screenshot_trigger.is_triggered {
            let pixels: Box<[u8]> = vec![0; (resolution_width * resolution_height * 4) as usize].into_boxed_slice();
            materials.screenshot_pixels = Some(pixels);
            match materials.screenshot_pixels {
                Some(ref mut pixels) => self.ctx.dispatcher().dispatch_screenshot(resolution_width, resolution_height, pixels)?,
                None => return Err("Screenshot failed because a bad bug right here.".into()),
            }
            materials.main_buffer_stack.pop()?;
            materials.main_buffer_stack.assert_no_stack()?;
        } else {
            materials.main_buffer_stack.pop()?;
            materials.main_buffer_stack.assert_no_stack()?;

            gl.bind_framebuffer(glow::FRAMEBUFFER, None);
            gl.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);

            gl.viewport(0, 0, viewport_width as i32, viewport_height as i32);

            materials.internal_resolution_render.render(materials.main_buffer_stack.get_nth(1)?.texture());
        }

        check_error(&gl, line!())?;

        Ok(())
    }
}

fn check_error(gl: &GlowSafeAdapter<glow::Context>, line: u32) -> AppResult<()> {
    let error = gl.get_error();
    if error != glow::NO_ERROR {
        return Err(format!("{} on line: {}", error, line).into());
    }
    Ok(())
}

fn matrix_to_16_f32(matrix: glm::TMat4<f32>) -> [f32; 16] {
    [
        matrix[(0, 0)],
        matrix[(1, 0)],
        matrix[(2, 0)],
        matrix[(3, 0)],
        matrix[(0, 1)],
        matrix[(1, 1)],
        matrix[(2, 1)],
        matrix[(3, 1)],
        matrix[(0, 2)],
        matrix[(1, 2)],
        matrix[(2, 2)],
        matrix[(3, 2)],
        matrix[(0, 3)],
        matrix[(1, 3)],
        matrix[(2, 3)],
        matrix[(3, 3)],
    ]
}

fn vec_to_3_f32(vec: glm::Vec3) -> [f32; 3] {
    [vec.x, vec.y, vec.z]
}
