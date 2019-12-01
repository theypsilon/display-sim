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

use crate::action_bindings::on_button_action;
use crate::camera::{CameraData, CameraDirection, CameraLockMode, CameraSystem};
use crate::filter_params::FilterParams;
use crate::general_types::{get_3_f32color_from_int, get_int_from_3_f32color, Size2D};
use crate::input_types::{Input, InputEventValue};
use crate::internal_resolution::InternalResolution;
use crate::math::gcd;
use crate::pixels_shadow::ShadowShape;
use crate::simulation_context::SimulationContext;
use crate::simulation_core_state::{
    ColorChannels, Filters, FiltersPreset, InitialParameters, LatestCustomScalingChange, PixelsGeometryKind, Resources, ScalingMethod, ScreenCurvatureKind,
    TextureInterpolation, MOVEMENT_BASE_SPEED, MOVEMENT_SPEED_FACTOR, PIXEL_MANIPULATION_BASE_SPEED, TURNING_BASE_SPEED,
};
use app_error::AppResult;
use derive_new::new;
use std::str::FromStr;

#[derive(new)]
pub struct SimulationCoreTicker<'a> {
    pub ctx: &'a dyn SimulationContext,
    pub res: &'a mut Resources,
    pub input: &'a mut Input,
}

impl<'a> SimulationCoreTicker<'a> {
    pub fn tick(&mut self, now: f64) -> AppResult<()> {
        self.pre_process_input(now);
        SimulationUpdater::new(self.ctx, self.res, self.input).update()?;
        self.post_process_input();
        Ok(())
    }

    fn pre_process_input(&mut self, now: f64) {
        self.input.now = now;

        for value in self.input.custom_event.consume_values() {
            match value {
                InputEventValue::Keyboard { pressed, key } => {
                    if let Some(not_used) = on_button_action(&mut self.input, key.to_lowercase().as_ref(), pressed) {
                        self.ctx.dispatcher().dispatch_log(format!("Ignored key: {} {}", not_used, pressed));
                    }
                }
                InputEventValue::MouseClick(pressed) => assert_eq!(on_button_action(&mut self.input, "mouse_click", pressed).is_none(), true),
                InputEventValue::MouseMove { x, y } => {
                    self.input.mouse_position_x = x;
                    self.input.mouse_position_y = y;
                }
                InputEventValue::MouseWheel(wheel) => {
                    if self.input.canvas_focused {
                        self.input.mouse_scroll_y = wheel
                    }
                }
                InputEventValue::BlurredWindow => *self.input = Input::new(now),

                InputEventValue::FilterPreset(preset) => self.input.event_filter_preset = Some(preset),
                InputEventValue::PixelBrighttness(pixel_brighttness) => self.input.event_pixel_brighttness = Some(pixel_brighttness),
                InputEventValue::PixelContrast(pixel_contrast) => self.input.event_pixel_contrast = Some(pixel_contrast),
                InputEventValue::LightColor(light_color) => self.input.event_light_color = Some(light_color),
                InputEventValue::BrightnessColor(brightness_color) => self.input.event_brightness_color = Some(brightness_color),
                InputEventValue::BlurLevel(blur_level) => self.input.event_blur_level = Some(blur_level),
                InputEventValue::VerticalLpp(vertical_lpp) => self.input.event_vertical_lpp = Some(vertical_lpp),
                InputEventValue::HorizontalLpp(horizontal_lpp) => self.input.event_horizontal_lpp = Some(horizontal_lpp),
                InputEventValue::BacklightPercent(backlight_percent) => self.input.event_backlight_percent = Some(backlight_percent),
                InputEventValue::PixelShadowHeight(pixel_shadow_height) => self.input.event_pixel_shadow_height = Some(pixel_shadow_height),
                InputEventValue::PixelVerticalGap(pixel_vertical_gap) => self.input.event_pixel_vertical_gap = Some(pixel_vertical_gap),
                InputEventValue::PixelHorizontalGap(pixel_horizontal_gap) => self.input.event_pixel_horizontal_gap = Some(pixel_horizontal_gap),
                InputEventValue::PixelWidth(pixel_width) => self.input.event_pixel_width = Some(pixel_width),
                InputEventValue::PixelSpread(pixel_spread) => self.input.event_pixel_spread = Some(pixel_spread),
                InputEventValue::Camera(camera) => self.input.event_camera = Some(camera),
                InputEventValue::CustomScalingResolutionWidth(width) => self.input.event_scaling_resolution_width = Some(width),
                InputEventValue::CustomScalingResolutionHeight(width) => self.input.event_scaling_resolution_height = Some(width),
                InputEventValue::CustomScalingAspectRatioX(width) => self.input.event_scaling_aspect_ratio_x = Some(width),
                InputEventValue::CustomScalingAspectRatioY(width) => self.input.event_scaling_aspect_ratio_y = Some(width),
                InputEventValue::CustomScalingStretchNearest(flag) => self.input.event_custom_scaling_stretch_nearest = Some(flag),
                InputEventValue::ViewportResize(width, height) => self.input.event_viewport_resize = Some(Size2D { width, height }),
                InputEventValue::None => {}
            };
        }

        self.input.get_tracked_buttons().iter_mut().for_each(|button| button.track());
    }

    fn post_process_input(&mut self) {
        self.input.mouse_scroll_y = 0.0;
        self.input.mouse_position_x = 0;
        self.input.mouse_position_y = 0;
        self.input.custom_event.reset();
        self.input.reset_filters = false;
        self.input.reset_position = false;
        self.input.reset_speeds = false;

        self.input.get_options_to_be_noned().iter_mut().for_each(|opt| opt.set_none());
    }
}

struct SimulationUpdater<'a> {
    ctx: &'a dyn SimulationContext,
    res: &'a mut Resources,
    input: &'a Input,
    dt: f32,
}

impl<'a> SimulationUpdater<'a> {
    pub fn new(ctx: &'a dyn SimulationContext, res: &'a mut Resources, input: &'a Input) -> Self {
        SimulationUpdater {
            dt: ((input.now - res.timers.last_time) / 1000.0) as f32,
            ctx,
            res,
            input,
        }
    }

    pub fn update(&mut self) -> AppResult<()> {
        if self.res.resetted {
            self.change_frontend_input_values();
        }

        if let Some(viewport) = self.input.event_viewport_resize {
            self.ctx.dispatcher().dispatch_log(format!("viewport:resize: {:?}", viewport));
            self.res.video.viewport_size = viewport;
            self.res.scaling.scaling_initialized = false;
        }

        self.update_timers();

        self.update_animation_buffer();

        if self.input.esc.is_just_pressed() {
            self.ctx.dispatcher().dispatch_exiting_session();
            self.res.quit = true;
            return Ok(());
        }

        if self.input.space.is_just_pressed() {
            self.ctx.dispatcher().dispatch_toggle_info_panel();
        }

        self.update_speeds();
        self.update_scaling();
        self.update_filters()?;
        self.update_camera();
        self.update_screenshot();
        if self.res.filters.preset_kind == FiltersPreset::DemoFlight1 {
            self.update_demo();
        }

        self.update_outputs();

        self.res.resetted = false;
        self.res.drawable = self.res.screenshot_trigger.is_triggered || self.res.screenshot_trigger.delay <= 0;

        Ok(())
    }

    fn update_screenshot(&mut self) {
        self.res.screenshot_trigger.is_triggered = false;
        if self.res.screenshot_trigger.delay > 0 {
            self.res.screenshot_trigger.delay -= 1;
        } else if self.input.screenshot.is_just_released() {
            self.res.screenshot_trigger.is_triggered = true;
            //let multiplier = self.res.filters.internal_resolution.multiplier as f32;
            self.res.screenshot_trigger.delay = 120; //(2.0 * multiplier * multiplier * (1.0 / self.dt)) as i32; // 2 seconds aprox.
            if self.res.screenshot_trigger.delay as f32 * self.dt > 2.0 {
                self.ctx.dispatcher().dispatch_top_message("Screenshot about to be downloaded, please wait.");
            }
        }
    }

    fn update_scaling(&mut self) {
        let ctx = &self.ctx;
        let input = &self.input;
        let mut changed = false;
        FilterParams::new(*ctx, &mut self.res.scaling.scaling_method, input.scaling_method.to_just_pressed())
            .set_trigger_handler(|x: &ScalingMethod| {
                changed = true;
                ctx.dispatcher().dispatch_scaling_method(*x)
            })
            .process_options();

        changed = changed
            || match self.res.scaling.scaling_method {
                ScalingMethod::Custom => self.update_custom_scaling(),
                _ => false,
            };

        self.res.scaling.scaling_initialized = self.res.scaling.scaling_initialized && !changed;
    }

    fn update_custom_scaling(&mut self) -> bool {
        let ctx = &self.ctx;
        let scaling = &mut self.res.scaling;
        let input = &self.input;
        let pixel_velocity = self.dt * self.res.speed.filter_speed;

        let mut changed = false;
        let mut custom_change = scaling.custom_change;

        if let Some(stretch_nearest) = input.event_custom_scaling_stretch_nearest {
            changed = true;
            scaling.custom_stretch = stretch_nearest;
            ctx.dispatcher().dispatch_custom_scaling_stretch_nearest(stretch_nearest);
        }

        changed = changed
            || FilterParams::new(*ctx, &mut scaling.pixel_width, input.pixel_width)
                .set_progression(pixel_velocity * 0.005)
                .set_event_value(input.event_pixel_width)
                .set_min(0.001)
                .set_trigger_handler(|x| {
                    ctx.dispatcher().dispatch_change_pixel_width(x);
                    custom_change = LatestCustomScalingChange::PixelSize;
                })
                .process_with_sums();
        changed = changed
            || FilterParams::new(*ctx, &mut scaling.custom_resolution.width, input.scaling_resolution_width.to_just_pressed())
                .set_progression(1.0)
                .set_event_value(input.event_scaling_resolution_width)
                .set_min(1.0)
                .set_max(100_000.0)
                .set_trigger_handler(|x| ctx.dispatcher().dispatch_scaling_resolution_width(x as u32))
                .process_with_sums();
        changed = changed
            || FilterParams::new(*ctx, &mut scaling.custom_resolution.height, input.scaling_resolution_height.to_just_pressed())
                .set_progression(1.0)
                .set_event_value(input.event_scaling_resolution_height)
                .set_min(1.0)
                .set_max(100_000.0)
                .set_trigger_handler(|x| ctx.dispatcher().dispatch_scaling_resolution_height(x as u32))
                .process_with_sums();
        changed = changed
            || FilterParams::new(*ctx, &mut scaling.custom_aspect_ratio.width, input.scaling_aspect_ratio_x.to_just_pressed())
                .set_progression(1.0)
                .set_event_value(input.event_scaling_aspect_ratio_x)
                .set_min(1.0)
                .set_max(1920.0 * 4.0)
                .set_trigger_handler(|x| {
                    ctx.dispatcher().dispatch_scaling_aspect_ratio_x(x);
                    custom_change = LatestCustomScalingChange::AspectRatio;
                })
                .process_with_sums();
        changed = changed
            || FilterParams::new(*ctx, &mut scaling.custom_aspect_ratio.height, input.scaling_aspect_ratio_y.to_just_pressed())
                .set_progression(1.0)
                .set_event_value(input.event_scaling_aspect_ratio_y)
                .set_min(1.0)
                .set_max(1080.0 * 4.0)
                .set_trigger_handler(|x| {
                    ctx.dispatcher().dispatch_scaling_aspect_ratio_y(x);
                    custom_change = LatestCustomScalingChange::AspectRatio;
                })
                .process_with_sums();

        scaling.custom_change = custom_change;

        changed
    }

    fn update_timers(&mut self) {
        let ellapsed = self.input.now - self.res.timers.last_second;
        self.res.timers.last_time = self.input.now;

        if ellapsed >= 1_000.0 {
            let fps = self.res.timers.frame_count as f32;
            self.ctx.dispatcher().dispatch_fps(fps);
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
            self.ctx.dispatcher().dispatch_top_message("All speeds have been reset.");
            self.change_frontend_input_values();
        }
        let ctx = &self.ctx;
        let input = &self.input;
        FilterParams::new(*ctx, &mut self.res.camera.turning_speed, input.turn_speed.to_just_pressed())
            .set_progression(2.0)
            .set_min(0.007_812_5 * TURNING_BASE_SPEED)
            .set_max(16_384.0 * TURNING_BASE_SPEED)
            .set_trigger_handler(|x| ctx.dispatcher().dispatch_change_turning_speed(x / TURNING_BASE_SPEED))
            .process_with_multiplications();
        FilterParams::new(*ctx, &mut self.res.speed.filter_speed, input.filter_speed.to_just_pressed())
            .set_progression(2.0)
            .set_min(0.007_812_5 * PIXEL_MANIPULATION_BASE_SPEED)
            .set_max(16_384.0 * PIXEL_MANIPULATION_BASE_SPEED)
            .set_trigger_handler(|x| ctx.dispatcher().dispatch_change_pixel_speed(x / PIXEL_MANIPULATION_BASE_SPEED))
            .process_with_multiplications();
        FilterParams::new(*ctx, &mut self.res.camera.turning_speed, input.translation_speed.to_just_pressed())
            .set_progression(2.0)
            .set_min(0.007_812_5 * TURNING_BASE_SPEED)
            .set_max(16_384.0 * TURNING_BASE_SPEED)
            .set_trigger_handler(|x| ctx.dispatcher().dispatch_change_turning_speed(x / TURNING_BASE_SPEED))
            .process_with_multiplications();
        FilterParams::new(*ctx, &mut self.res.camera.movement_speed, input.translation_speed.to_just_pressed())
            .set_progression(2.0)
            .set_min(0.007_812_5 * initial_movement_speed)
            .set_max(16_384.0 * initial_movement_speed)
            .set_trigger_handler(|x| ctx.dispatcher().dispatch_change_movement_speed(x / initial_movement_speed))
            .process_with_multiplications();
    }

    fn update_filters(&mut self) -> AppResult<()> {
        self.update_filter_presets_from_event()?;
        if self.input.reset_filters {
            self.res.filters = Filters::default();
            self.change_frontend_input_values();
            self.ctx.dispatcher().dispatch_top_message("All filter options have been reset.");
            return Ok(());
        }
        if let Some(light_color) = self.input.event_light_color {
            self.res.filters.light_color = light_color;
            self.ctx.dispatcher().dispatch_top_message("Light Color changed.");
        }
        if let Some(brightness_color) = self.input.event_light_color {
            self.res.filters.brightness_color = brightness_color;
            self.ctx.dispatcher().dispatch_top_message("Brightness Color changed.");
        }

        let ctx = &self.ctx;
        let filters = &mut self.res.filters;
        let input = &self.input;

        let mut changed = false;

        changed = changed
            || FilterParams::new(*ctx, &mut filters.extra_bright, input.bright)
                .set_progression(0.01 * self.dt * self.res.speed.filter_speed)
                .set_event_value(input.event_pixel_brighttness)
                .set_min(-1.0)
                .set_max(1.0)
                .set_trigger_handler(|x| ctx.dispatcher().dispatch_change_pixel_brightness(x))
                .process_with_sums();
        changed = changed
            || FilterParams::new(*ctx, &mut filters.extra_contrast, input.contrast)
                .set_progression(0.01 * self.dt * self.res.speed.filter_speed)
                .set_event_value(input.event_pixel_contrast)
                .set_min(0.0)
                .set_max(20.0)
                .set_trigger_handler(|x| ctx.dispatcher().dispatch_change_pixel_contrast(x))
                .process_with_sums();
        changed = changed
            || FilterParams::new(*ctx, &mut filters.blur_passes, input.blur.to_just_pressed())
                .set_progression(1)
                .set_event_value(input.event_blur_level)
                .set_min(0)
                .set_max(100)
                .set_trigger_handler(|x| ctx.dispatcher().dispatch_change_blur_level(x))
                .process_with_sums();
        changed = changed
            || FilterParams::new(*ctx, &mut filters.texture_interpolation, input.next_texture_interpolation.to_just_pressed())
                .set_trigger_handler(|x: &TextureInterpolation| ctx.dispatcher().dispatch_texture_interpolation(*x))
                .process_options();
        changed = changed
            || FilterParams::new(*ctx, &mut filters.screen_curvature_kind, input.next_screen_curvature_type.to_just_pressed())
                .set_trigger_handler(|x: &ScreenCurvatureKind| ctx.dispatcher().dispatch_screen_curvature(*x))
                .process_options();
        changed = changed
            || FilterParams::new(*ctx, &mut filters.backlight_presence, input.backlight_percent)
                .set_progression(0.01 * self.dt * self.res.speed.filter_speed)
                .set_event_value(input.event_backlight_percent)
                .set_min(0.0)
                .set_max(1.0)
                .set_trigger_handler(|x| ctx.dispatcher().dispatch_backlight_presence(x))
                .process_with_sums();
        changed = changed
            || FilterParams::new(*ctx, &mut filters.color_channels, input.next_color_representation_kind.to_just_pressed())
                .set_trigger_handler(|x: &ColorChannels| ctx.dispatcher().dispatch_color_representation(*x))
                .process_options();

        filters.internal_resolution.set_max_texture_size(self.res.video.max_texture_size);
        if FilterParams::new(*ctx, &mut filters.internal_resolution, input.next_internal_resolution.to_just_pressed())
            .set_trigger_handler(|x| ctx.dispatcher().dispatch_internal_resolution(x))
            .process_options()
        {
            self.res.scaling.scaling_initialized = false;
            changed = true;
        }
        changed = changed
            || FilterParams::new(*ctx, &mut filters.vertical_lpp, input.vertical_lpp.to_just_pressed())
                .set_progression(1)
                .set_event_value(input.event_vertical_lpp)
                .set_min(1)
                .set_max(20)
                .set_trigger_handler(|x| ctx.dispatcher().dispatch_change_vertical_lpp(x))
                .process_with_sums();
        changed = changed
            || FilterParams::new(*ctx, &mut filters.horizontal_lpp, input.horizontal_lpp.to_just_pressed())
                .set_progression(1)
                .set_event_value(input.event_horizontal_lpp)
                .set_min(1)
                .set_max(20)
                .set_trigger_handler(|x| ctx.dispatcher().dispatch_change_horizontal_lpp(x))
                .process_with_sums();

        let pixel_velocity = self.dt * self.res.speed.filter_speed;
        changed = changed
            || FilterParams::new(*ctx, &mut filters.pixels_geometry_kind, input.next_pixel_geometry_kind.to_just_pressed())
                .set_trigger_handler(|x: &PixelsGeometryKind| ctx.dispatcher().dispatch_pixel_geometry(*x))
                .process_options();
        changed = changed
            || FilterParams::new(*ctx, &mut filters.pixel_shadow_shape_kind, input.next_pixel_shadow_shape_kind.to_just_pressed())
                .set_trigger_handler(|x: &ShadowShape| ctx.dispatcher().dispatch_pixel_shadow_shape(*x))
                .process_options();
        changed = changed
            || FilterParams::new(*ctx, &mut filters.pixel_shadow_height, input.next_pixels_shadow_height)
                .set_progression(self.dt * 0.3)
                .set_event_value(input.event_pixel_shadow_height)
                .set_min(0.0)
                .set_max(1.0)
                .set_trigger_handler(|x| ctx.dispatcher().dispatch_pixel_shadow_height(x))
                .process_with_sums();
        changed = changed
            || FilterParams::new(*ctx, &mut filters.cur_pixel_vertical_gap, input.pixel_vertical_gap)
                .set_progression(pixel_velocity * 0.00125)
                .set_event_value(input.event_pixel_vertical_gap)
                .set_min(0.0)
                .set_trigger_handler(|x| ctx.dispatcher().dispatch_change_pixel_vertical_gap(x))
                .process_with_sums();
        changed = changed
            || FilterParams::new(*ctx, &mut filters.cur_pixel_horizontal_gap, input.pixel_horizontal_gap)
                .set_progression(pixel_velocity * 0.00125)
                .set_event_value(input.event_pixel_horizontal_gap)
                .set_min(0.0)
                .set_trigger_handler(|x| ctx.dispatcher().dispatch_change_pixel_horizontal_gap(x))
                .process_with_sums();
        changed = changed
            || FilterParams::new(*ctx, &mut filters.cur_pixel_spread, input.pixel_spread)
                .set_progression(pixel_velocity * 0.005)
                .set_event_value(input.event_pixel_spread)
                .set_min(0.0)
                .set_trigger_handler(|x| ctx.dispatcher().dispatch_change_pixel_spread(x))
                .process_with_sums();

        if changed {
            if self.res.filters.preset_kind != FiltersPreset::Custom && self.res.filters.preset_kind != FiltersPreset::DemoFlight1 {
                ctx.dispatcher().dispatch_change_preset_selected(&FiltersPreset::Custom.to_string());
                self.res.filters.preset_kind = FiltersPreset::Custom;
            } else if self.res.filters.preset_kind == FiltersPreset::Custom {
                self.res.custom_is_changed = true;
            }
        }

        Ok(())
    }

    fn update_filter_presets_from_event(&mut self) -> AppResult<()> {
        let preset = if let Some(preset) = &self.input.event_filter_preset {
            preset
        } else {
            return Ok(());
        };
        let preset = FromStr::from_str(preset)?;
        if self.res.filters.preset_kind == preset {
            return Ok(());
        }
        if self.res.filters.preset_kind == FiltersPreset::Custom && self.res.custom_is_changed {
            self.res.saved_filters = Some(self.res.filters.clone());
        }
        if self.res.filters.preset_kind == FiltersPreset::DemoFlight1 {
            self.res.camera = self.res.demo_1.camera_backup.clone();
        }
        self.res.filters = self.res.filters.preset_factory(preset, &self.res.saved_filters);
        if self.res.filters.preset_kind == FiltersPreset::DemoFlight1 {
            self.res.demo_1.needs_initialization = true;
        }
        if self.res.filters.preset_kind == FiltersPreset::Custom {
            self.res.custom_is_changed = false;
        }
        self.change_frontend_input_values();
        Ok(())
    }

    fn update_camera(&mut self) {
        if self.input.reset_position {
            self.res.scaling.scaling_initialized = false;
            self.ctx.dispatcher().dispatch_top_message("The camera have been reset.");
        }

        if self.input.next_camera_movement_mode.increase.is_just_pressed() || self.input.next_camera_movement_mode.decrease.is_just_pressed() {
            self.res.camera.locked_mode = match self.res.camera.locked_mode {
                CameraLockMode::FreeFlight => CameraLockMode::LockOnDisplay,
                CameraLockMode::LockOnDisplay => CameraLockMode::FreeFlight,
            };
            self.ctx.dispatcher().dispatch_change_camera_movement_mode(self.res.camera.locked_mode);
            self.ctx
                .dispatcher()
                .dispatch_top_message(&format!("Camera movement: {}.", &self.res.camera.locked_mode.to_string()));
        }

        let mut camera = CameraSystem::new(&mut self.res.camera, self.ctx.dispatcher());

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

        if self.input.mouse_click.is_just_pressed() && self.input.canvas_focused {
            self.ctx.dispatcher().dispatch_request_pointer_lock();
        } else if self.input.mouse_click.is_activated() && self.input.canvas_focused {
            camera.drag(self.input.mouse_position_x, self.input.mouse_position_y);
        } else if self.input.mouse_click.is_just_released() {
            self.ctx.dispatcher().dispatch_exit_pointer_lock();
        }

        if self.input.camera_zoom.increase {
            camera.change_zoom(self.dt * -100.0, self.ctx.dispatcher());
        } else if self.input.camera_zoom.decrease {
            camera.change_zoom(self.dt * 100.0, self.ctx.dispatcher());
        } else if self.input.mouse_scroll_y != 0.0 {
            camera.change_zoom(self.input.mouse_scroll_y, self.ctx.dispatcher());
        }

        if let Some(change) = self.input.event_camera {
            camera.handle_camera_change(change);
        }

        camera.update_view(self.dt)
    }

    fn change_frontend_input_values(&self) {
        let dispatcher = self.ctx.dispatcher();
        dispatcher.enable_extra_messages(false);
        dispatcher.dispatch_change_pixel_horizontal_gap(self.res.filters.cur_pixel_horizontal_gap);
        dispatcher.dispatch_change_pixel_vertical_gap(self.res.filters.cur_pixel_vertical_gap);
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
        dispatcher.dispatch_scaling_method(self.res.scaling.scaling_method);
        dispatcher.dispatch_scaling_resolution_width(self.res.scaling.custom_resolution.width as u32);
        dispatcher.dispatch_scaling_resolution_height(self.res.scaling.custom_resolution.height as u32);
        dispatcher.dispatch_scaling_aspect_ratio_x(self.res.scaling.custom_aspect_ratio.width);
        dispatcher.dispatch_scaling_aspect_ratio_y(self.res.scaling.custom_aspect_ratio.height);
        dispatcher.dispatch_custom_scaling_stretch_nearest(self.res.scaling.custom_stretch);
        dispatcher.dispatch_change_pixel_width(self.res.scaling.pixel_width);
        // This one shouldn't be needed because it's always coming from frontend to backend.
        //dispatcher.dispatch_change_preset_selected(&self.res.filters.preset_kind.to_string());
        dispatcher.enable_extra_messages(true);
    }

    fn update_demo(&mut self) {
        if self.res.demo_1.needs_initialization {
            self.res.demo_1.needs_initialization = false;
            self.res.demo_1.camera_backup = self.res.camera.clone();
            self.res.camera.locked_mode = CameraLockMode::FreeFlight;
            self.res.demo_1.movement_target = glm::vec3(0.0, 0.0, 0.0);
            self.res.demo_1.movement_speed = glm::vec3(0.0, 0.0, 0.0);
            self.res.camera.set_position(glm::vec3(0.0, 0.0, 0.0));
            self.res.camera.direction = glm::vec3(0.0, 1.0, 0.0);
            self.res.camera.axis_up = glm::vec3(0.0, 0.0, 1.0);
            self.res.demo_1.color_target = glm::make_vec3(&get_3_f32color_from_int(self.res.filters.light_color));
            self.res.demo_1.color_position = self.res.demo_1.color_target;
        }
        {
            // moving position
            let movement_position = self.res.camera.get_position();
            let mut movement_route = self.res.demo_1.movement_target - movement_position;
            if glm::length(&movement_route).abs() <= std::f32::EPSILON {
                movement_route = glm::vec3(1.0, 0.0, 0.0);
            }
            let movement_force = movement_route.normalize() * self.dt * 1.2;
            self.res.demo_1.movement_speed += movement_force;
            if glm::length(&self.res.demo_1.movement_speed).abs() > self.res.demo_1.movement_max_speed {
                self.res.demo_1.movement_speed = self.res.demo_1.movement_speed.normalize() * self.res.demo_1.movement_max_speed;
            }
            self.res.camera.set_position(movement_position + self.res.demo_1.movement_speed);
            if glm::length(&movement_route).abs() <= 15.0 {
                let rnd_x = self.ctx.random().next() - 0.5;
                let rnd_y = self.ctx.random().next() - 0.5;
                let rnd_z = self.ctx.random().next() - 0.5;
                self.res.demo_1.movement_target = glm::vec3(
                    0.5 * rnd_x * self.res.video.image_size.width as f32 + self.res.video.image_size.width as f32 * if rnd_x > 0.0 { 0.75 } else { -0.75 },
                    0.5 * rnd_y * self.res.video.image_size.height as f32 + self.res.video.image_size.height as f32 * if rnd_y > 0.0 { 0.75 } else { -0.75 },
                    2.0 * rnd_z * self.res.initial_parameters.initial_position_z,
                );
                if self.res.demo_1.movement_target.z < 0.0 && self.ctx.random().next() > 0.3 {
                    self.res.demo_1.movement_target.z = -self.res.demo_1.movement_target.z;
                }
                self.res.demo_1.movement_max_speed = self.ctx.random().next() * 0.6 + 0.3;
                if self.ctx.random().next() < 0.33 {
                    self.res.filters.color_channels = ColorChannels::Overlapping;
                } else {
                    self.res.filters.color_channels = ColorChannels::Combined;
                }
                if self.ctx.random().next() < 0.33 {
                    self.res.filters.pixels_geometry_kind = PixelsGeometryKind::Squares;
                } else {
                    self.res.filters.pixels_geometry_kind = PixelsGeometryKind::Cubes;
                }
            }
            CameraSystem::new(&mut self.res.camera, self.ctx.dispatcher()).look_at(glm::vec3(0.0, 0.0, 0.0));
        }
        {
            // moving color
            let color_route = self.res.demo_1.color_target - self.res.demo_1.color_position;
            let is_void_route = color_route == glm::vec3(0.0, 0.0, 0.0);
            if !is_void_route {
                self.res.demo_1.color_position += color_route.normalize() * self.dt * 0.1;
                self.res.filters.light_color = get_int_from_3_f32color(&self.res.demo_1.color_position.into());
                self.ctx.dispatcher().dispatch_change_light_color(self.res.filters.light_color);
            }
            if is_void_route || glm::length(&color_route).abs() <= 0.15 {
                let rnd_r = self.ctx.random().next() * 0.6 + 0.4;
                let rnd_g = self.ctx.random().next() * 0.6 + 0.4;
                let rnd_b = self.ctx.random().next() * 0.6 + 0.4;
                self.res.demo_1.color_target = glm::vec3(rnd_r, rnd_g, rnd_b);
            }
        }
        {
            // spreading
            let spread_change = self.dt * 0.03 * self.res.filters.cur_pixel_spread * self.res.filters.cur_pixel_spread;
            if self.res.demo_1.spreading {
                self.res.filters.cur_pixel_spread += spread_change;
                if self.res.filters.cur_pixel_spread > 1000.0 {
                    self.res.demo_1.spreading = false;
                }
            } else {
                self.res.filters.cur_pixel_spread -= spread_change;
                if self.res.filters.cur_pixel_spread <= 0.5 {
                    self.res.demo_1.spreading = true;
                    self.res.filters.cur_pixel_spread = 0.5;
                }
            }
        }
    }

    fn update_outputs(&mut self) {
        self.update_output_scaling();
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

    fn update_output_scaling(&mut self) {
        if self.res.scaling.scaling_initialized {
            return;
        }
        self.res.scaling.scaling_initialized = true;

        let stretch;
        let ar_x;
        let ar_y;
        let image_width;
        let image_height;
        let pixel_width;
        match self.res.scaling.scaling_method {
            ScalingMethod::AutoDetect => {
                let (message, ar) = calculate_aspect_ratio_from_image_size(self.res.video.image_size);
                let ar = simplify_ar(ar);
                ar_x = ar.0;
                ar_y = ar.1;
                image_width = self.res.video.image_size.width;
                image_height = self.res.video.image_size.height;
                pixel_width = (ar_x / ar_y) / (image_width as f32 / image_height as f32);
                stretch = false;
                self.ctx.dispatcher().dispatch_top_message(&format!("Automatic scaling: {}", message));
            }
            ScalingMethod::SquaredPixels => {
                let ar = simplify_ar(self.res.video.image_size.to_f32().to_tuple());
                ar_x = ar.0;
                ar_y = ar.1;
                image_width = self.res.video.image_size.width;
                image_height = self.res.video.image_size.height;
                pixel_width = 1.0;
                stretch = false;
            }
            ScalingMethod::FullImage4By3 => {
                ar_x = 4.0;
                ar_y = 3.0;
                image_width = self.res.video.image_size.width;
                image_height = self.res.video.image_size.height;
                pixel_width = (ar_x / ar_y) / (image_width as f32 / image_height as f32);
                stretch = false;
            }
            ScalingMethod::StretchToBothEdges => {
                let ar = simplify_ar(self.res.video.viewport_size.to_f32().to_tuple());
                ar_x = ar.0;
                ar_y = ar.1;
                image_width = self.res.video.image_size.width;
                image_height = self.res.video.image_size.height;
                pixel_width = (ar_x / ar_y) / (image_width as f32 / image_height as f32);
                stretch = true;
            }
            ScalingMethod::StretchToNearestEdge => {
                let (message, ar) = calculate_aspect_ratio_from_image_size(self.res.video.image_size);
                let ar = simplify_ar(ar);
                ar_x = ar.0;
                ar_y = ar.1;
                image_width = self.res.video.image_size.width;
                image_height = self.res.video.image_size.height;
                pixel_width = (ar_x / ar_y) / (image_width as f32 / image_height as f32);
                stretch = true;
                self.ctx.dispatcher().dispatch_top_message(&format!("Nearest edge with: {}", message));
            }
            ScalingMethod::Custom => {
                stretch = self.res.scaling.custom_stretch;
                image_width = self.res.scaling.custom_resolution.width as u32;
                image_height = self.res.scaling.custom_resolution.height as u32;
                let custom_resolution_ratio = self.res.scaling.custom_resolution.width / self.res.scaling.custom_resolution.height;
                match self.res.scaling.custom_change {
                    LatestCustomScalingChange::AspectRatio => {
                        ar_x = self.res.scaling.custom_aspect_ratio.width;
                        ar_y = self.res.scaling.custom_aspect_ratio.height;
                        pixel_width = (ar_x / ar_y) / custom_resolution_ratio;
                    }
                    LatestCustomScalingChange::PixelSize => {
                        pixel_width = self.res.scaling.pixel_width;
                        ar_x = pixel_width * custom_resolution_ratio;
                        ar_y = 1.0;
                    }
                }
            }
        }

        self.ctx.dispatcher().dispatch_change_pixel_width(pixel_width);
        self.ctx.dispatcher().dispatch_scaling_aspect_ratio_x(ar_x);
        self.ctx.dispatcher().dispatch_scaling_aspect_ratio_y(ar_y);
        self.ctx.dispatcher().dispatch_scaling_resolution_width(image_width);
        self.ctx.dispatcher().dispatch_scaling_resolution_height(image_height);
        self.ctx.dispatcher().dispatch_custom_scaling_stretch_nearest(stretch);

        self.res.scaling.pixel_width = pixel_width;

        let z = {
            let background_size = Size2D {
                width: image_width as f32,
                height: image_height as f32,
            };
            calculate_far_away_position(background_size, &self.res.filters.internal_resolution, self.res.scaling.pixel_width, stretch)
        };
        let mut camera = CameraData::new(MOVEMENT_BASE_SPEED * z / MOVEMENT_SPEED_FACTOR, TURNING_BASE_SPEED);
        camera.set_position(glm::vec3(0.0, 0.0, z));
        self.res.initial_parameters = InitialParameters {
            initial_position_z: z,
            initial_movement_speed: camera.movement_speed,
        };
        self.res.camera = camera;
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
        let scaling = &self.res.scaling;

        output.pixel_spread = [(1.0 + filters.cur_pixel_spread) * scaling.pixel_width, 1.0 + filters.cur_pixel_spread];
        output.pixel_scale_base = [
            (filters.cur_pixel_vertical_gap + 1.0) / scaling.pixel_width,
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
                *pixel_scale = [(0.0 + 1.0) / scaling.pixel_width, 0.0 + 1.0, (0.0 + 0.0) * 0.5 + 1.0];
                if filters.vertical_lpp > 1 {
                    let vl_cur_offset = vl_offset_beginning + vl_idx as f32;
                    pixel_offset[0] = (pixel_offset[0] + vl_cur_offset * scaling.pixel_width) * by_vertical_lpp;
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
                        (filters.cur_pixel_vertical_gap + 1.0) / scaling.pixel_width,
                        filters.cur_pixel_horizontal_gap + 1.0,
                        (filters.cur_pixel_vertical_gap + filters.cur_pixel_vertical_gap) * 0.5 + 1.0,
                    ];
                    if filters.vertical_lpp > 1 {
                        let vl_cur_offset = vl_offset_beginning + vl_idx as f32;
                        pixel_offset[0] = (pixel_offset[0] + vl_cur_offset * scaling.pixel_width) * by_vertical_lpp;
                        pixel_scale[0] *= filters.vertical_lpp as f32;
                    }
                    if filters.horizontal_lpp > 1 {
                        let hl_cur_offset = hl_offset_beginning + hl_idx as f32;
                        pixel_offset[1] = (pixel_offset[1] + hl_cur_offset) * by_horizontal_lpp;
                        pixel_scale[1] *= filters.horizontal_lpp as f32;
                        if filters.horizontal_lpp % 2 == 0 && hl_idx % 2 == 1 {
                            pixel_offset[0] += 0.5 * scaling.pixel_width * by_vertical_lpp;
                        }
                    }
                    match filters.color_channels {
                        ColorChannels::Combined => {}
                        _ => match filters.color_channels {
                            ColorChannels::SplitHorizontal => {
                                pixel_offset[0] +=
                                    by_vertical_lpp * (color_idx as f32 - 1.0) * (1.0 / 3.0) * scaling.pixel_width / (filters.cur_pixel_vertical_gap + 1.0);
                                pixel_scale[0] *= output.color_splits as f32;
                            }
                            ColorChannels::Overlapping => {
                                pixel_offset[0] +=
                                    by_vertical_lpp * (color_idx as f32 - 1.0) * (1.0 / 3.0) * scaling.pixel_width / (filters.cur_pixel_vertical_gap + 1.0);
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

fn simplify_ar(ar: (f32, f32)) -> (f32, f32) {
    if ar.0.fract() == 0.0 && ar.1.fract() == 0.0 {
        let a = ar.0.trunc() as u32;
        let b = ar.1.trunc() as u32;
        let gcd = gcd(a, b);
        ((a / gcd) as f32, (b / gcd) as f32)
    } else {
        (ar.0 / ar.1, 1.0)
    }
}

fn calculate_aspect_ratio_from_image_size(image_size: Size2D<u32>) -> (&'static str, (f32, f32)) {
    if image_size.height == 102 {
        ("1.57:1 (Atari Lynx) on full image.", (1.57, 1.0))
    } else if image_size.height == 144 {
        ("1.11:1 (Game Boy) on full image.", (1.11, 1.0))
    } else if image_size.height == 152 {
        ("21:20 (Neo Geo Pocket) on full image.", (21.0, 20.0))
    } else if image_size.height == 160 {
        ("3:2 (Game Boy Advance) on full image.", (3.0, 2.0))
    } else if image_size.height == 192 {
        ("4:3 (Nintendo DS) on full image.", (4.0, 3.0))
    } else if image_size.height == 272 {
        ("44:25 (PSP) on full image.", (44.0, 25.0))
    } else if image_size.height == 544 {
        ("44:25 (PS Vita) on full image.", (44.0, 25.0))
    } else if image_size.height > 540 {
        ("Squared pixels.", (image_size.width as f32, image_size.height as f32))
    } else {
        ("4:3 on full image.", (4.0, 3.0))
    }
}

fn calculate_far_away_position(bg_size: Size2D<f32>, internal_resolution: &InternalResolution, pixel_width: f32, stretch: bool) -> f32 {
    let resolution_width = internal_resolution.width() as f32;
    let resolution_height = internal_resolution.height() as f32;

    let virtual_resolution_width = resolution_width / pixel_width;

    let width_ratio = virtual_resolution_width / bg_size.width;
    let height_ratio = resolution_height / bg_size.height;

    let is_height_bounded = width_ratio > height_ratio;

    let bound_ratio = if is_height_bounded { height_ratio } else { width_ratio };
    let bound_resolution = if is_height_bounded { resolution_height } else { virtual_resolution_width };

    bound_resolution * if is_height_bounded { 1.2076 } else { 0.68 * pixel_width } / if stretch { bound_ratio } else { bound_ratio.floor() }

    /*
    @TODO: Honestly, I'm not sure where did I take these numbers from but they seem to work fine with that formula.
        It's a bit sad to admit it, but I think I took them by meassuring screenshots from the framebuffer,
        and moving the camera back and forth between meassures until alignment was pixel perfect for 8k/4k/1080p/720p.
        I just meassured those resolutions now and they seem to work fine for 4:3 and 21:9 images in a 16:9 screen.

        Interesting mathematical fact: 0.68 * squared(4/3) = 1.2076 = 0.68 * 16/9
    */
}
