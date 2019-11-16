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

use crate::error::AppResult;
use crate::pixels_render::PixelsUniform;
use crate::simulation_render_state::Materials;
use core::simulation_context::SimulationContext;
use core::simulation_core_state::{ColorChannels, Resources, TextureInterpolation};

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

        let gl = &self.materials.gl;

        if self.res.video.needs_buffer_data_load {
            self.materials.pixels_render.load_image(&self.res.video);
        }

        self.materials.main_buffer_stack.set_depthbuffer(self.res.output.pixel_have_depth);

        self.materials
            .main_buffer_stack
            .set_resolution(self.res.filters.internal_resolution.width(), self.res.filters.internal_resolution.height());

        self.materials
            .main_buffer_stack
            .set_interpolation(match self.res.filters.texture_interpolation {
                TextureInterpolation::Linear => glow::LINEAR,
                TextureInterpolation::Nearest => glow::NEAREST,
            });

        self.materials.main_buffer_stack.push()?;
        self.materials.main_buffer_stack.push()?;
        self.materials.main_buffer_stack.bind_current()?;

        gl.clear_color(0.0, 0.0, 0.0, 0.0);
        gl.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);

        let view = self.res.camera.get_view();
        let position = self.res.camera.get_position();
        let projection = self
            .res
            .camera
            .get_projection(self.res.video.viewport_size.width as f32, self.res.video.viewport_size.height as f32);

        for hl_idx in 0..self.res.filters.horizontal_lpp {
            for vl_idx in 0..self.res.filters.vertical_lpp {
                for color_idx in 0..self.res.output.color_splits {
                    if let ColorChannels::Overlapping = self.res.filters.color_channels {
                        self.materials.main_buffer_stack.push()?;
                        self.materials.main_buffer_stack.bind_current()?;
                        if vl_idx == 0 && hl_idx == 0 {
                            gl.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);
                        }
                    }
                    self.materials.pixels_render.render(PixelsUniform {
                        shadow_kind: self.res.filters.pixel_shadow_shape_kind.value,
                        geometry_kind: self.res.filters.pixels_geometry_kind,
                        view: &matrix_to_16_f32(view),
                        projection: &matrix_to_16_f32(projection),
                        ambient_strength: self.res.output.ambient_strength,
                        contrast_factor: self.res.filters.extra_contrast,
                        light_color: &self.res.output.light_color[color_idx],
                        extra_light: &self.res.output.extra_light,
                        light_pos: &vec_to_3_f32(position),
                        screen_curvature: self.res.output.screen_curvature_factor,
                        pixel_spread: &self.res.output.pixel_spread,
                        pixel_scale: &self
                            .res
                            .output
                            .pixel_scale_foreground
                            .get(vl_idx * self.res.filters.horizontal_lpp + hl_idx)
                            .expect("Bad pixel_scale_foreground")[color_idx],
                        pixel_pulse: self.res.output.pixels_pulse,
                        pixel_offset: &self
                            .res
                            .output
                            .pixel_offset_foreground
                            .get(vl_idx * self.res.filters.horizontal_lpp + hl_idx)
                            .expect("Bad pixel_offset_foreground")[color_idx],
                        height_modifier_factor: self.res.output.height_modifier_factor,
                    });
                }
                if let ColorChannels::Overlapping = self.res.filters.color_channels {
                    self.materials.main_buffer_stack.pop()?;
                    self.materials.main_buffer_stack.pop()?;
                    self.materials.main_buffer_stack.pop()?;
                }
            }
        }

        if let ColorChannels::Overlapping = self.res.filters.color_channels {
            self.materials.main_buffer_stack.bind_current()?;
            gl.active_texture(glow::TEXTURE0 + 0);
            gl.bind_texture(glow::TEXTURE_2D, self.materials.main_buffer_stack.get_nth(1)?.texture());
            gl.active_texture(glow::TEXTURE0 + 1);
            gl.bind_texture(glow::TEXTURE_2D, self.materials.main_buffer_stack.get_nth(2)?.texture());
            gl.active_texture(glow::TEXTURE0 + 2);
            gl.bind_texture(glow::TEXTURE_2D, self.materials.main_buffer_stack.get_nth(3)?.texture());

            self.materials.rgb_render.render();

            gl.active_texture(glow::TEXTURE0 + 0);
        }

        self.materials.main_buffer_stack.push()?;
        self.materials.main_buffer_stack.bind_current()?;
        gl.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);

        if self.res.output.showing_background {
            self.materials.bg_buffer_stack.set_resolution(1920 / 2, 1080 / 2);
            self.materials.bg_buffer_stack.set_depthbuffer(false);
            self.materials.bg_buffer_stack.set_interpolation(glow::LINEAR);
            self.materials.bg_buffer_stack.push()?;
            self.materials.bg_buffer_stack.bind_current()?;
            gl.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);
            for hl_idx in 0..self.res.filters.horizontal_lpp {
                for vl_idx in 0..self.res.filters.vertical_lpp {
                    self.materials.pixels_render.render(PixelsUniform {
                        shadow_kind: 0,
                        geometry_kind: self.res.filters.pixels_geometry_kind,
                        view: &matrix_to_16_f32(view),
                        projection: &matrix_to_16_f32(projection),
                        ambient_strength: self.res.output.ambient_strength,
                        contrast_factor: self.res.filters.extra_contrast,
                        light_color: &self.res.output.light_color_background,
                        extra_light: &[0.0, 0.0, 0.0],
                        light_pos: &vec_to_3_f32(position),
                        pixel_spread: &self.res.output.pixel_spread,
                        pixel_scale: &self.res.output.pixel_scale_background[vl_idx * self.res.filters.horizontal_lpp + hl_idx],
                        screen_curvature: self.res.output.screen_curvature_factor,
                        pixel_pulse: self.res.output.pixels_pulse,
                        pixel_offset: &self.res.output.pixel_offset_background[vl_idx * self.res.filters.horizontal_lpp + hl_idx],
                        height_modifier_factor: 0.0,
                    });
                }
            }
            let source = (*self.materials.bg_buffer_stack.get_current()?).clone();
            let target = self.materials.main_buffer_stack.get_current()?;
            self.materials.blur_render.render(&mut self.materials.bg_buffer_stack, &source, &target, 6)?;
            self.materials.bg_buffer_stack.pop()?;
        }
        self.materials.main_buffer_stack.pop()?;
        self.materials.main_buffer_stack.pop()?;
        self.materials.main_buffer_stack.bind_current()?;
        gl.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);

        gl.active_texture(glow::TEXTURE0 + 0);
        gl.bind_texture(glow::TEXTURE_2D, self.materials.main_buffer_stack.get_nth(1)?.texture());
        gl.active_texture(glow::TEXTURE0 + 1);
        gl.bind_texture(glow::TEXTURE_2D, self.materials.main_buffer_stack.get_nth(2)?.texture());
        self.materials.background_render.render();
        gl.active_texture(glow::TEXTURE0 + 0);

        if self.res.filters.blur_passes > 0 {
            let target = self.materials.main_buffer_stack.get_current()?.clone();
            self.materials
                .blur_render
                .render(&mut self.materials.main_buffer_stack, &target, &target, self.res.filters.blur_passes)?;
        }

        self.materials.screenshot_pixels = None;

        if self.res.screenshot_trigger.is_triggered {
            let width = self.res.filters.internal_resolution.width();
            let height = self.res.filters.internal_resolution.height();
            let pixels: Box<[u8]> = vec![0; (width * height * 4) as usize].into_boxed_slice();
            self.materials.screenshot_pixels = Some(pixels);
            self.ctx.dispatcher().fire_screenshot(
                width,
                height,
                self.materials.screenshot_pixels.as_mut().expect("Screenshot bug"),
                self.res.filters.internal_resolution.multiplier,
            );
        }

        self.materials.main_buffer_stack.pop()?;
        self.materials.main_buffer_stack.assert_no_stack()?;

        gl.bind_framebuffer(glow::FRAMEBUFFER, None);
        gl.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);
        gl.viewport(0, 0, self.res.video.viewport_size.width as i32, self.res.video.viewport_size.height as i32);

        self.materials
            .internal_resolution_render
            .render(self.materials.main_buffer_stack.get_nth(1)?.texture());

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
