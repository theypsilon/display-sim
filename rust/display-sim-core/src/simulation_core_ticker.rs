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

use crate::app_events::AppEventDispatcher;
use crate::camera::{CameraData, CameraDirection, CameraSystem};
use crate::filter_params::FilterParams;
use crate::general_types::get_3_f32color_from_int;
use crate::pixels_shadow::ShadowShape;
use crate::simulation_context::SimulationContext;
use crate::simulation_core_state::{
    event_kind, ColorChannels, Filters, Input, InputEventValue, PixelsGeometryKind, Resources, ScreenCurvatureKind, TextureInterpolation,
    PIXEL_MANIPULATION_BASE_SPEED, TURNING_BASE_SPEED,
};
use derive_new::new;

#[derive(new)]
pub struct SimulationCoreTicker<'a, T: AppEventDispatcher> {
    ctx: &'a mut SimulationContext<T>,
    res: &'a mut Resources,
    input: &'a mut Input,
}

impl<'a, T: AppEventDispatcher> SimulationCoreTicker<'a, T> {
    pub fn tick(&mut self, now: f64) {
        self.pre_process_input(now);
        SimulationUpdater::new(self.ctx, self.res, self.input).update();
        self.post_process_input();
    }

    fn pre_process_input(&mut self, now: f64) {
        self.input.now = now;
        self.input.get_mut_fields_booleanbutton().iter_mut().for_each(|button| button.track_input());
        self.input
            .get_mut_fields_incdec_booleanbutton_()
            .iter_mut()
            .for_each(|incdec| incdec.get_mut_fields_t().iter_mut().for_each(|button| button.track_input()));
    }

    fn post_process_input(&mut self) {
        self.input.mouse_scroll_y = 0.0;
        self.input.mouse_position_x = 0;
        self.input.mouse_position_y = 0;
        self.input.custom_event.reset();
        self.input.reset_filters = false;
        self.input.reset_position = false;
        self.input.reset_speeds = false;
    }
}

struct SimulationUpdater<'a, T: AppEventDispatcher> {
    ctx: &'a mut SimulationContext<T>,
    res: &'a mut Resources,
    input: &'a Input,
    dt: f32,
}

macro_rules! read_event_value {
    ($this:ident, $variant:ident, $kind:ident) => {
        if let InputEventValue::$variant(value) = $this.input.custom_event.get_value(event_kind::$kind) {
            Some(*value)
        } else {
            None
        }
    };
}

impl<'a, T: AppEventDispatcher> SimulationUpdater<'a, T> {
    pub fn new(ctx: &'a mut SimulationContext<T>, res: &'a mut Resources, input: &'a Input) -> SimulationUpdater<'a, T> {
        SimulationUpdater {
            dt: ((input.now - res.timers.last_time) / 1000.0) as f32,
            ctx,
            res,
            input,
        }
    }

    pub fn update(&mut self) {
        if self.res.resetted {
            //self.res.filters.internal_resolution.set_resolution(4320);
            self.change_frontend_input_values();
        }
        self.update_timers();

        self.update_animation_buffer();

        if self.input.esc.is_just_pressed() {
            self.ctx.dispatcher.dispatch_exiting_session();
            self.res.quit = true;
            return;
        }

        if self.input.space.is_just_pressed() {
            self.ctx.dispatcher.dispatch_toggle_info_panel();
        }

        self.update_speeds();
        self.update_filters();
        self.update_camera();
        self.update_screenshot();

        self.update_outputs();

        self.res.resetted = false;
        self.res.drawable = self.res.screenshot_trigger.is_triggered || self.res.screenshot_trigger.delay <= 0;
    }

    fn update_screenshot(&mut self) {
        self.res.screenshot_trigger.is_triggered = false;
        if self.res.screenshot_trigger.delay > 0 {
            self.res.screenshot_trigger.delay -= 1;
        } else if self.input.screenshot.is_just_released() {
            self.res.screenshot_trigger.is_triggered = true;
            let multiplier = self.res.filters.internal_resolution.multiplier as f32;
            self.res.screenshot_trigger.delay = (2.0 * multiplier * multiplier * (1.0 / self.dt)) as i32; // 2 seconds aprox.
            if self.res.screenshot_trigger.delay as f32 * self.dt > 2.0 {
                self.ctx.dispatcher.dispatch_top_message("Screenshot about to be downloaded, please wait.");
            }
        }
    }

    fn update_timers(&mut self) {
        let ellapsed = self.input.now - self.res.timers.last_second;
        self.res.timers.last_time = self.input.now;

        if ellapsed >= 1_000.0 {
            let fps = self.res.timers.frame_count as f32;
            self.ctx.dispatcher.dispatch_fps(fps);
            self.res.timers.last_second = self.input.now;
            self.res.timers.frame_count = 0;
        } else {
            self.res.timers.frame_count += 1;
        }
    }

    fn update_animation_buffer(&mut self) {
        self.res.video.needs_buffer_data_load = self.res.resetted;
        let next_frame_update = self.res.video.last_frame_change + 0.001 * f64::from(self.res.video.steps[self.res.video.current_frame].delay);
        if self.input.now >= next_frame_update {
            self.res.video.last_frame_change = next_frame_update;
            let last_frame = self.res.video.current_frame;
            self.res.video.current_frame += 1;
            if self.res.video.current_frame >= self.res.video.steps.len() {
                self.res.video.current_frame = 0;
            }
            if last_frame != self.res.video.current_frame {
                self.res.video.needs_buffer_data_load = true;
            }
        }
    }

    fn update_speeds(&mut self) {
        let initial_movement_speed = self.res.initial_parameters.initial_movement_speed;
        if self.input.reset_speeds {
            self.res.camera.turning_speed = TURNING_BASE_SPEED;
            self.res.camera.movement_speed = initial_movement_speed;
            self.res.speed.filter_speed = PIXEL_MANIPULATION_BASE_SPEED;
            self.ctx.dispatcher.dispatch_top_message("All speeds have been reset.");
            self.change_frontend_input_values();
        }
        let ctx = &self.ctx;
        let input = &self.input;
        FilterParams::new(ctx, &mut self.res.camera.turning_speed, input.turn_speed.to_just_pressed())
            .set_progression(2.0)
            .set_min(0.007_812_5 * TURNING_BASE_SPEED)
            .set_max(16_384.0 * TURNING_BASE_SPEED)
            .set_trigger_handler(|x| ctx.dispatcher.dispatch_change_turning_speed(x / TURNING_BASE_SPEED))
            .process_with_multiplications();
        FilterParams::new(ctx, &mut self.res.speed.filter_speed, input.filter_speed.to_just_pressed())
            .set_progression(2.0)
            .set_min(0.007_812_5 * PIXEL_MANIPULATION_BASE_SPEED)
            .set_max(16_384.0 * PIXEL_MANIPULATION_BASE_SPEED)
            .set_trigger_handler(|x| ctx.dispatcher.dispatch_change_pixel_speed(x / PIXEL_MANIPULATION_BASE_SPEED))
            .process_with_multiplications();
        FilterParams::new(ctx, &mut self.res.camera.turning_speed, input.translation_speed.to_just_pressed())
            .set_progression(2.0)
            .set_min(0.007_812_5 * TURNING_BASE_SPEED)
            .set_max(16_384.0 * TURNING_BASE_SPEED)
            .set_trigger_handler(|x| ctx.dispatcher.dispatch_change_turning_speed(x / TURNING_BASE_SPEED))
            .process_with_multiplications();
        FilterParams::new(ctx, &mut self.res.camera.movement_speed, input.translation_speed.to_just_pressed())
            .set_progression(2.0)
            .set_min(0.007_812_5 * initial_movement_speed)
            .set_max(16_384.0 * initial_movement_speed)
            .set_trigger_handler(|x| ctx.dispatcher.dispatch_change_movement_speed(x / initial_movement_speed))
            .process_with_multiplications();
    }

    fn update_filters(&mut self) {
        if let InputEventValue::FilterPreset(preset) = self.input.custom_event.get_value(event_kind::FILTER_PRESET) {
            if self.res.filters.preset_name == "Custom" {
                self.res.saved_filters = Some(self.res.filters.clone());
            }
            self.res.filters = match preset.as_ref() {
                "sharp-1" => self.res.filters.preset_sharp_1(),
                "crt-aperture-grille-1" => self.res.filters.preset_crt_aperture_grille_1(),
                "crt-shadow-mask-1" => self.res.filters.preset_crt_shadow_mask_1(),
                "crt-shadow-mask-2" => self.res.filters.preset_crt_shadow_mask_2(),
                _ => {
                    if let Some(ref saved_filters) = self.res.saved_filters {
                        saved_filters.clone()
                    } else {
                        return;
                    }
                }
            };
            self.change_frontend_input_values();
        }
        if self.input.reset_filters {
            self.res.filters = Filters::default();
            self.res.filters.cur_pixel_width = self.res.initial_parameters.initial_pixel_width;
            self.res
                .filters
                .internal_resolution
                .initialize(self.res.video.viewport_size, self.res.video.max_texture_size);
            self.change_frontend_input_values();
            self.ctx.dispatcher.dispatch_top_message("All filter options have been reset.");
            return;
        }
        if let InputEventValue::LightColor(light_color) = self.input.custom_event.get_value(event_kind::LIGHT_COLOR) {
            self.res.filters.light_color = *light_color;
            self.ctx.dispatcher.dispatch_top_message("Light Color changed.");
        }
        if let InputEventValue::BrightnessColor(brightness_color) = self.input.custom_event.get_value(event_kind::LIGHT_COLOR) {
            self.res.filters.brightness_color = *brightness_color;
            self.ctx.dispatcher.dispatch_top_message("Brightness Color changed.");
        }

        let ctx = &self.ctx;
        let filters = &mut self.res.filters;
        let input = &self.input;

        let mut changed = false;

        FilterParams::new(ctx, &mut filters.extra_bright, input.bright)
            .set_progression(0.01 * self.dt * self.res.speed.filter_speed)
            .set_event_value(read_event_value!(self, PixelBrighttness, PIXEL_BRIGHTNESS))
            .set_min(-1.0)
            .set_max(1.0)
            .set_trigger_handler(|x| {
                changed = true;
                ctx.dispatcher.dispatch_change_pixel_brightness(x);
            })
            .process_with_sums();
        FilterParams::new(ctx, &mut filters.extra_contrast, input.contrast)
            .set_progression(0.01 * self.dt * self.res.speed.filter_speed)
            .set_event_value(read_event_value!(self, PixelContrast, PIXEL_CONTRAST))
            .set_min(0.0)
            .set_max(20.0)
            .set_trigger_handler(|x| {
                changed = true;
                ctx.dispatcher.dispatch_change_pixel_contrast(x);
            })
            .process_with_sums();
        FilterParams::new(ctx, &mut filters.blur_passes, input.blur.to_just_pressed())
            .set_progression(1)
            .set_event_value(read_event_value!(self, BlurLevel, BLUR_LEVEL))
            .set_min(0)
            .set_max(100)
            .set_trigger_handler(|x| {
                changed = true;
                ctx.dispatcher.dispatch_change_blur_level(x);
            })
            .process_with_sums();
        FilterParams::new(ctx, &mut filters.texture_interpolation, input.next_texture_interpolation.to_just_pressed())
            .set_trigger_handler(|x: &TextureInterpolation| {
                changed = true;
                ctx.dispatcher.dispatch_texture_interpolation(*x);
            })
            .process_options();
        FilterParams::new(ctx, &mut filters.screen_curvature_kind, input.next_screen_curvature_type.to_just_pressed())
            .set_trigger_handler(|x: &ScreenCurvatureKind| {
                changed = true;
                ctx.dispatcher.dispatch_screen_curvature(*x);
            })
            .process_options();
        FilterParams::new(ctx, &mut filters.backlight_presence, input.backlight_percent)
            .set_progression(0.01 * self.dt * self.res.speed.filter_speed)
            .set_event_value(read_event_value!(self, BacklightPercent, BACKLIGHT_PERCENT))
            .set_min(0.0)
            .set_max(1.0)
            .set_trigger_handler(|x| {
                changed = true;
                ctx.dispatcher.dispatch_backlight_presence(x);
            })
            .process_with_sums();
        FilterParams::new(ctx, &mut filters.color_channels, input.next_color_representation_kind.to_just_pressed())
            .set_trigger_handler(|x: &ColorChannels| {
                changed = true;
                ctx.dispatcher.dispatch_color_representation(*x);
            })
            .process_options();
        FilterParams::new(ctx, &mut filters.internal_resolution, input.next_internal_resolution.to_just_pressed())
            .set_trigger_handler(|x| {
                changed = true;
                ctx.dispatcher.dispatch_internal_resolution(x);
            })
            .process_options();
        FilterParams::new(ctx, &mut filters.vertical_lpp, input.vertical_lpp.to_just_pressed())
            .set_progression(1)
            .set_event_value(read_event_value!(self, VerticalLpp, VERTICAL_LPP))
            .set_min(1)
            .set_max(20)
            .set_trigger_handler(|x| {
                changed = true;
                ctx.dispatcher.dispatch_change_vertical_lpp(x);
            })
            .process_with_sums();
        FilterParams::new(ctx, &mut filters.horizontal_lpp, input.horizontal_lpp.to_just_pressed())
            .set_progression(1)
            .set_event_value(read_event_value!(self, HorizontalLpp, HORIZONTAL_LPP))
            .set_min(1)
            .set_max(20)
            .set_trigger_handler(|x| {
                changed = true;
                ctx.dispatcher.dispatch_change_horizontal_lpp(x);
            })
            .process_with_sums();

        let pixel_velocity = self.dt * self.res.speed.filter_speed;
        FilterParams::new(ctx, &mut filters.pixels_geometry_kind, input.next_pixel_geometry_kind.to_just_pressed())
            .set_trigger_handler(|x: &PixelsGeometryKind| {
                changed = true;
                ctx.dispatcher.dispatch_pixel_geometry(*x);
            })
            .process_options();
        FilterParams::new(ctx, &mut filters.pixel_shadow_shape_kind, input.next_pixel_shadow_shape_kind.to_just_pressed())
            .set_trigger_handler(|x: &ShadowShape| {
                changed = true;
                ctx.dispatcher.dispatch_pixel_shadow_shape(*x);
            })
            .process_options();
        FilterParams::new(ctx, &mut filters.pixel_shadow_height, input.next_pixels_shadow_height)
            .set_progression(self.dt * 0.3)
            .set_event_value(read_event_value!(self, PixelShadowHeight, PIXEL_SHADOW_HEIGHT))
            .set_min(0.0)
            .set_max(1.0)
            .set_trigger_handler(|x| {
                changed = true;
                ctx.dispatcher.dispatch_pixel_shadow_height(x);
            })
            .process_with_sums();
        FilterParams::new(ctx, &mut filters.cur_pixel_vertical_gap, input.pixel_vertical_gap)
            .set_progression(pixel_velocity * 0.00125)
            .set_event_value(read_event_value!(self, PixelVerticalGap, PIXEL_VERTICAL_GAP))
            .set_min(0.0)
            .set_trigger_handler(|x| {
                changed = true;
                ctx.dispatcher.dispatch_change_pixel_vertical_gap(x);
            })
            .process_with_sums();
        FilterParams::new(ctx, &mut filters.cur_pixel_horizontal_gap, input.pixel_horizontal_gap)
            .set_progression(pixel_velocity * 0.00125)
            .set_event_value(read_event_value!(self, PixelHorizontalGap, PIXEL_HORIZONTAL_GAP))
            .set_min(0.0)
            .set_trigger_handler(|x| {
                changed = true;
                ctx.dispatcher.dispatch_change_pixel_horizontal_gap(x);
            })
            .process_with_sums();
        FilterParams::new(ctx, &mut filters.cur_pixel_width, input.pixel_width)
            .set_progression(pixel_velocity * 0.005)
            .set_event_value(read_event_value!(self, PixelWidth, PIXEL_WIDTH))
            .set_min(0.0)
            .set_trigger_handler(|x| {
                changed = true;
                ctx.dispatcher.dispatch_change_pixel_width(x);
            })
            .process_with_sums();
        FilterParams::new(ctx, &mut filters.cur_pixel_spread, input.pixel_spread)
            .set_progression(pixel_velocity * 0.005)
            .set_event_value(read_event_value!(self, PixelSpread, PIXEL_SPREAD))
            .set_min(0.0)
            .set_trigger_handler(|x| {
                changed = true;
                ctx.dispatcher.dispatch_change_pixel_spread(x);
            })
            .process_with_sums();

        if changed && self.res.filters.preset_name != "Custom" {
            ctx.dispatcher.dispatch_custom_preset();
            self.res.filters.preset_name = "Custom".into();
        }
    }

    fn update_camera(&mut self) {
        if self.input.reset_position {
            let initial_position = glm::vec3(0.0, 0.0, self.res.initial_parameters.initial_position_z);
            self.res.camera = CameraData::new(self.res.camera.movement_speed, self.res.camera.turning_speed);
            self.res.camera.set_position(initial_position);
            self.ctx.dispatcher.dispatch_top_message("The camera have been reset.");
        }

        if self.input.next_camera_movement_mode.increase.is_just_pressed() || self.input.next_camera_movement_mode.decrease.is_just_pressed() {
            self.res.camera.locked_mode = !self.res.camera.locked_mode;
            self.ctx.dispatcher.dispatch_change_camera_movement_mode(self.res.camera.locked_mode)
        }

        let mut camera = CameraSystem::new(&mut self.res.camera, &self.ctx.dispatcher);

        if self.input.walk_left {
            camera.advance(CameraDirection::Left, self.dt);
        }
        if self.input.walk_right {
            camera.advance(CameraDirection::Right, self.dt);
        }
        if self.input.walk_up {
            camera.advance(CameraDirection::Up, self.dt);
        }
        if self.input.walk_down {
            camera.advance(CameraDirection::Down, self.dt);
        }
        if self.input.walk_forward {
            camera.advance(CameraDirection::Forward, self.dt);
        }
        if self.input.walk_backward {
            camera.advance(CameraDirection::Backward, self.dt);
        }

        if self.input.turn_left {
            camera.turn(CameraDirection::Left, self.dt);
        }
        if self.input.turn_right {
            camera.turn(CameraDirection::Right, self.dt);
        }
        if self.input.turn_up {
            camera.turn(CameraDirection::Up, self.dt);
        }
        if self.input.turn_down {
            camera.turn(CameraDirection::Down, self.dt);
        }

        if self.input.rotate_left {
            camera.rotate(CameraDirection::Left, self.dt);
        }
        if self.input.rotate_right {
            camera.rotate(CameraDirection::Right, self.dt);
        }

        if self.input.mouse_click.is_just_pressed() {
            self.ctx.dispatcher.dispatch_request_pointer_lock();
        } else if self.input.mouse_click.is_activated() {
            camera.drag(self.input.mouse_position_x, self.input.mouse_position_y);
        } else if self.input.mouse_click.is_just_released() {
            self.ctx.dispatcher.dispatch_exit_pointer_lock();
        }

        if self.input.camera_zoom.increase {
            camera.change_zoom(self.dt * -100.0, &self.ctx.dispatcher);
        } else if self.input.camera_zoom.decrease {
            camera.change_zoom(self.dt * 100.0, &self.ctx.dispatcher);
        } else if self.input.mouse_scroll_y != 0.0 {
            camera.change_zoom(self.input.mouse_scroll_y, &self.ctx.dispatcher);
        }

        for event_value in self.input.custom_event.get_values() {
            if let InputEventValue::Camera(change) = *event_value {
                camera.handle_camera_change(change);
            }
        }

        camera.update_view(self.dt)
    }

    fn change_frontend_input_values(&self) {
        let dispatcher = &self.ctx.dispatcher;
        dispatcher.enable_extra_messages(false);
        dispatcher.dispatch_change_pixel_horizontal_gap(self.res.filters.cur_pixel_horizontal_gap);
        dispatcher.dispatch_change_pixel_vertical_gap(self.res.filters.cur_pixel_vertical_gap);
        dispatcher.dispatch_change_pixel_width(self.res.filters.cur_pixel_width);
        dispatcher.dispatch_change_pixel_spread(self.res.filters.cur_pixel_spread);
        dispatcher.dispatch_change_pixel_brightness(self.res.filters.extra_bright);
        dispatcher.dispatch_change_pixel_contrast(self.res.filters.extra_contrast);
        dispatcher.dispatch_change_light_color(self.res.filters.light_color);
        dispatcher.dispatch_change_brightness_color(self.res.filters.brightness_color);
        dispatcher.dispatch_change_camera_zoom(self.res.camera.zoom);
        dispatcher.dispatch_change_camera_movement_mode(self.res.camera.locked_mode);
        dispatcher.dispatch_change_blur_level(self.res.filters.blur_passes);
        dispatcher.dispatch_change_vertical_lpp(self.res.filters.vertical_lpp);
        dispatcher.dispatch_change_horizontal_lpp(self.res.filters.horizontal_lpp);
        dispatcher.dispatch_color_representation(self.res.filters.color_channels);
        dispatcher.dispatch_pixel_geometry(self.res.filters.pixels_geometry_kind);
        dispatcher.dispatch_pixel_shadow_shape(self.res.filters.pixel_shadow_shape_kind);
        dispatcher.dispatch_pixel_shadow_height(self.res.filters.pixel_shadow_height);
        dispatcher.dispatch_backlight_presence(self.res.filters.backlight_presence);
        dispatcher.dispatch_screen_curvature(self.res.filters.screen_curvature_kind);
        dispatcher.dispatch_internal_resolution(&self.res.filters.internal_resolution);
        dispatcher.dispatch_texture_interpolation(self.res.filters.texture_interpolation);
        dispatcher.dispatch_change_pixel_speed(self.res.speed.filter_speed / PIXEL_MANIPULATION_BASE_SPEED);
        dispatcher.dispatch_change_turning_speed(self.res.camera.turning_speed / TURNING_BASE_SPEED);
        dispatcher.dispatch_change_movement_speed(self.res.camera.movement_speed / self.res.initial_parameters.initial_movement_speed);
        dispatcher.enable_extra_messages(true);
    }

    fn update_outputs(&mut self) {
        self.update_output_filter_source_colors();
        self.update_output_filter_curvature();
        self.update_output_filter_backlight();

        let output = &mut self.res.output;
        let filters = &self.res.filters;

        let (ambient_strength, pixel_have_depth) = match filters.pixels_geometry_kind {
            PixelsGeometryKind::Squares => (1.0, false),
            PixelsGeometryKind::Cubes => (0.5, true),
        };
        output.ambient_strength = ambient_strength;
        output.pixel_have_depth = pixel_have_depth;
        output.height_modifier_factor = 1.0 - filters.pixel_shadow_height;

        self.update_output_pixel_scale_gap_offset();
    }

    fn update_output_filter_source_colors(&mut self) {
        let output = &mut self.res.output;
        let filters = &self.res.filters;

        output.color_splits = match filters.color_channels {
            ColorChannels::Combined => 1,
            _ => 3,
        };
        output.light_color_background = get_3_f32color_from_int(filters.light_color);
        for i in 0..output.color_splits {
            let mut light_color = output.light_color_background;
            match filters.color_channels {
                ColorChannels::Combined => {}
                _ => {
                    light_color[(i + 0) % 3] *= 1.0;
                    light_color[(i + 1) % 3] = 0.0;
                    light_color[(i + 2) % 3] = 0.0;
                }
            }
            output.light_color[i] = light_color;
        }
        output.extra_light = get_3_f32color_from_int(filters.brightness_color);
        for light in output.extra_light.iter_mut() {
            *light *= filters.extra_bright;
        }
    }

    fn update_output_filter_curvature(&mut self) {
        let output = &mut self.res.output;
        let filters = &self.res.filters;

        output.screen_curvature_factor = match filters.screen_curvature_kind {
            ScreenCurvatureKind::Curved1 => 0.15,
            ScreenCurvatureKind::Curved2 => 0.3,
            ScreenCurvatureKind::Curved3 => 0.45,
            _ => 0.0,
        };

        if let ScreenCurvatureKind::Pulse = filters.screen_curvature_kind {
            output.pixels_pulse += self.dt * 0.3;
        } else {
            output.pixels_pulse = 0.0;
        }
    }

    fn update_output_filter_backlight(&mut self) {
        let output = &mut self.res.output;
        let filters = &self.res.filters;

        output.showing_background = filters.backlight_presence > 0.0;
        let solid_color_weight = filters.backlight_presence;

        for i in 0..3 {
            output.light_color_background[i] *= solid_color_weight;
        }
    }

    fn update_output_pixel_scale_gap_offset(&mut self) {
        let output = &mut self.res.output;
        let filters = &self.res.filters;

        output.pixel_spread = [(1.0 + filters.cur_pixel_spread) * filters.cur_pixel_width, 1.0 + filters.cur_pixel_spread];
        output.pixel_scale_base = [
            (filters.cur_pixel_vertical_gap + 1.0) / filters.cur_pixel_width,
            filters.cur_pixel_horizontal_gap + 1.0,
            (filters.cur_pixel_vertical_gap + filters.cur_pixel_vertical_gap) * 0.5 + 1.0,
        ];

        let by_vertical_lpp = 1.0 / (filters.vertical_lpp as f32);
        let by_horizontal_lpp = 1.0 / (filters.horizontal_lpp as f32);
        let vl_offset_beginning = -(filters.vertical_lpp as f32 - 1.0) / 2.0;
        let hl_offset_beginning = -(filters.horizontal_lpp as f32 - 1.0) / 2.0;

        let line_passes = filters.vertical_lpp * filters.horizontal_lpp;
        output.pixel_scale_background.resize_with(line_passes, Default::default);
        output.pixel_offset_background.resize_with(line_passes, Default::default);
        for hl_idx in 0..filters.horizontal_lpp {
            for vl_idx in 0..filters.vertical_lpp {
                let pixel_offset = &mut output.pixel_offset_background[vl_idx * filters.horizontal_lpp + hl_idx];
                let pixel_scale = &mut output.pixel_scale_background[vl_idx * filters.horizontal_lpp + hl_idx];

                *pixel_offset = [0.0, 0.0, 0.0];
                *pixel_scale = [(0.0 + 1.0) / filters.cur_pixel_width, 0.0 + 1.0, (0.0 + 0.0) * 0.5 + 1.0];
                if filters.vertical_lpp > 1 {
                    let vl_cur_offset = vl_offset_beginning + vl_idx as f32;
                    pixel_offset[0] = (pixel_offset[0] + vl_cur_offset * filters.cur_pixel_width) * by_vertical_lpp;
                    pixel_scale[0] *= filters.vertical_lpp as f32;
                }
                if filters.horizontal_lpp > 1 {
                    let hl_cur_offset = hl_offset_beginning + hl_idx as f32;
                    pixel_offset[1] = (pixel_offset[1] + hl_cur_offset) * by_horizontal_lpp;
                    pixel_scale[1] *= filters.horizontal_lpp as f32;
                }
            }
        }

        output.pixel_scale_foreground.resize_with(line_passes, Default::default);
        output.pixel_offset_foreground.resize_with(line_passes, Default::default);
        for hl_idx in 0..filters.horizontal_lpp {
            for vl_idx in 0..filters.vertical_lpp {
                for color_idx in 0..output.color_splits {
                    let pixel_offset = &mut output.pixel_offset_foreground[vl_idx * filters.horizontal_lpp + hl_idx][color_idx];
                    let pixel_scale = &mut output.pixel_scale_foreground[vl_idx * filters.horizontal_lpp + hl_idx][color_idx];
                    *pixel_offset = [0.0, 0.0, 0.0];
                    *pixel_scale = [
                        (filters.cur_pixel_vertical_gap + 1.0) / filters.cur_pixel_width,
                        filters.cur_pixel_horizontal_gap + 1.0,
                        (filters.cur_pixel_vertical_gap + filters.cur_pixel_vertical_gap) * 0.5 + 1.0,
                    ];
                    if filters.vertical_lpp > 1 {
                        let vl_cur_offset = vl_offset_beginning + vl_idx as f32;
                        pixel_offset[0] = (pixel_offset[0] + vl_cur_offset * filters.cur_pixel_width) * by_vertical_lpp;
                        pixel_scale[0] *= filters.vertical_lpp as f32;
                    }
                    if filters.horizontal_lpp > 1 {
                        let hl_cur_offset = hl_offset_beginning + hl_idx as f32;
                        pixel_offset[1] = (pixel_offset[1] + hl_cur_offset) * by_horizontal_lpp;
                        pixel_scale[1] *= filters.horizontal_lpp as f32;
                        if filters.horizontal_lpp % 2 == 0 && hl_idx % 2 == 1 {
                            pixel_offset[0] += 0.5 * filters.cur_pixel_width * by_vertical_lpp;
                        }
                    }
                    match filters.color_channels {
                        ColorChannels::Combined => {}
                        _ => match filters.color_channels {
                            ColorChannels::SplitHorizontal => {
                                pixel_offset[0] +=
                                    by_vertical_lpp * (color_idx as f32 - 1.0) * (1.0 / 3.0) * filters.cur_pixel_width / (filters.cur_pixel_vertical_gap + 1.0);
                                pixel_scale[0] *= output.color_splits as f32;
                            }
                            ColorChannels::Overlapping => {
                                pixel_offset[0] +=
                                    by_vertical_lpp * (color_idx as f32 - 1.0) * (1.0 / 3.0) * filters.cur_pixel_width / (filters.cur_pixel_vertical_gap + 1.0);
                                pixel_scale[0] *= 1.5;
                            }
                            ColorChannels::SplitVertical => {
                                pixel_offset[1] += by_horizontal_lpp * (color_idx as f32 - 1.0) * (1.0 / 3.0) / (filters.cur_pixel_horizontal_gap + 1.0);
                                pixel_scale[1] *= output.color_splits as f32;
                            }
                            _ => unreachable!(),
                        },
                    }
                }
            }
        }
    }
}
