use crate::app_events::AppEventDispatcher;
use crate::camera::{CameraData, CameraDirection, CameraSystem};
use crate::general_types::{get_3_f32color_from_int, NextEnumVariant};
use crate::pixels_shadow::SHADOWS_LEN;
use crate::simulation_context::SimulationContext;
use crate::simulation_core_state::{
    event_kind, ColorChannels, Filters, IncDec, Input, InputEventValue, PixelsGeometryKind, Resources, ScreenCurvatureKind, ScreenLayeringKind,
    PIXEL_MANIPULATION_BASE_SPEED, TURNING_BASE_SPEED,
};
use derive_new::new;
use enum_len_trait::EnumLen;
use num_traits::{FromPrimitive, ToPrimitive};
use std::cmp::{PartialEq, PartialOrd};
use std::fmt::Display;
use std::ops::{Add, Div, Mul, Sub};

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
            Some(value)
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

        self.update_filters();
        self.update_speeds();
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

    fn update_filters(&mut self) {
        if self.input.reset_filters {
            self.res.filters = Filters::new(PIXEL_MANIPULATION_BASE_SPEED);
            self.res.filters.cur_pixel_width = self.res.initial_parameters.initial_pixel_width;
            self.res
                .filters
                .internal_resolution
                .initialize(self.res.video.viewport_size, self.res.video.max_texture_size);
            self.change_frontend_input_values();
            self.ctx.dispatcher.dispatch_top_message("All filter options have been reset.");
            return;
        }
        self.update_filter_source_colors();
        self.update_filter_blur();
        self.update_filter_lpp();
        self.update_filter_pixel_shape();
        self.update_filter_internal_resolution();
        self.update_filter_misc_enums();
    }

    fn update_filter_source_colors(&mut self) {
        let ctx = &self.ctx;
        FilterParams::new(ctx, &mut self.res.filters.extra_bright, self.input.bright.clone())
            .set_progression(0.01 * self.dt * self.res.filters.change_speed)
            .set_event_value(read_event_value!(self, PixelBrighttness, PIXEL_BRIGHTNESS))
            .set_min(-1.0)
            .set_max(1.0)
            .set_trigger_handler(|x| ctx.dispatcher.dispatch_change_pixel_brightness(x))
            .sum();
        FilterParams::new(ctx, &mut self.res.filters.extra_contrast, self.input.contrast.clone())
            .set_progression(0.01 * self.dt * self.res.filters.change_speed)
            .set_event_value(read_event_value!(self, PixelContrast, PIXEL_CONTRAST))
            .set_min(0.0)
            .set_max(20.0)
            .set_trigger_handler(|x| ctx.dispatcher.dispatch_change_pixel_contrast(x))
            .sum();
        if let InputEventValue::LightColor(light_color) = self.input.custom_event.get_value(event_kind::LIGHT_COLOR) {
            self.res.filters.light_color = light_color;
            self.ctx.dispatcher.dispatch_top_message("Light Color changed.");
        }
        if let InputEventValue::BrightnessColor(brightness_color) = self.input.custom_event.get_value(event_kind::LIGHT_COLOR) {
            self.res.filters.brightness_color = brightness_color;
            self.ctx.dispatcher.dispatch_top_message("Brightness Color changed.");
        }
    }

    fn update_filter_blur(&mut self) {
        let ctx = &self.ctx;
        FilterParams::new(ctx, &mut self.res.filters.blur_passes, self.input.blur.to_is_just_pressed())
            .set_progression(1)
            .set_event_value(read_event_value!(self, BlurLevel, BLUR_LEVEL))
            .set_min(0)
            .set_max(100)
            .set_trigger_handler(|x| {
                ctx.dispatcher.dispatch_top_message(&format!("Blur level: {}", x));
                ctx.dispatcher.dispatch_change_blur_level(x)
            })
            .sum();
    }

    fn update_filter_misc_enums(&mut self) {
        let ctx = &self.ctx;
        let next_texture_interpolation = self.input.next_texture_interpolation.to_is_just_pressed();
        FilterParams::new(ctx, &mut self.res.filters.texture_interpolation, next_texture_interpolation)
            .set_trigger_handler(|x| ctx.dispatcher.dispatch_texture_interpolation(x))
            .iterate_variant();
        let next_screen_curvature_kind = self.input.next_screen_curvature_type.to_is_just_pressed();
        FilterParams::new(ctx, &mut self.res.filters.screen_curvature_kind, next_screen_curvature_kind)
            .set_trigger_handler(|x| {
                ctx.dispatcher.dispatch_top_message(&format!("Screen curvature: {}.", x));
                ctx.dispatcher.dispatch_screen_curvature(x);
            })
            .iterate_variant();
        FilterParams::new(ctx, &mut self.res.filters.layering_kind, self.input.next_layering_kind.to_is_just_pressed())
            .set_trigger_handler(|x| {
                ctx.dispatcher.dispatch_top_message(&format!("Layering kind: {}.", x));
                ctx.dispatcher.dispatch_screen_layering_type(x);
            })
            .iterate_variant();
        let next_color_representation_kind = self.input.next_color_representation_kind.to_is_just_pressed();
        FilterParams::new(ctx, &mut self.res.filters.color_channels, next_color_representation_kind)
            .set_trigger_handler(|x| {
                ctx.dispatcher.dispatch_top_message(&format!("Pixel color representation: {}.", x));
                ctx.dispatcher.dispatch_color_representation(x);
            })
            .iterate_variant();
    }

    // lines per pixel
    fn update_filter_lpp(&mut self) {
        let ctx = &self.ctx;
        FilterParams::new(ctx, &mut self.res.filters.lines_per_pixel, self.input.lpp.to_is_just_pressed())
            .set_progression(1)
            .set_event_value(read_event_value!(self, LinersPerPixel, LINES_PER_PIXEL))
            .set_min(1)
            .set_max(20)
            .set_trigger_handler(|x| {
                ctx.dispatcher.dispatch_top_message(&format!("Lines per pixel: {}", x));
                ctx.dispatcher.dispatch_change_lines_per_pixel(x)
            })
            .sum();
    }

    fn update_filter_internal_resolution(&mut self) {
        if self.input.next_internal_resolution.any_just_released() {
            if self.input.next_internal_resolution.increase.is_just_released() {
                self.res.filters.internal_resolution.increase();
            }
            if self.input.next_internal_resolution.decrease.is_just_released() {
                self.res.filters.internal_resolution.decrease();
            }
            if self.res.filters.internal_resolution.minimum_reached {
                self.ctx.dispatcher.dispatch_top_message("Minimum internal resolution has been reached.");
            } else if self.res.filters.internal_resolution.maximium_reached {
                self.ctx.dispatcher.dispatch_top_message("Maximum internal resolution has been reached.");
            } else {
                self.ctx.dispatcher.dispatch_internal_resolution(&self.res.filters.internal_resolution);
            }
        }
    }

    fn update_filter_pixel_shape(&mut self) {
        let ctx = &self.ctx;
        FilterParams::new(
            ctx,
            &mut self.res.filters.pixels_geometry_kind,
            self.input.next_pixel_geometry_kind.to_is_just_pressed(),
        )
        .set_trigger_handler(|x| {
            ctx.dispatcher.dispatch_top_message(&format!("Pixel geometry: {}.", x));
            ctx.dispatcher.dispatch_pixel_geometry(x);
        })
        .iterate_variant();

        if self.input.next_pixels_shadow_shape_kind.any_just_pressed() {
            if self.input.next_pixels_shadow_shape_kind.increase.is_just_pressed() {
                self.res.filters.pixel_shadow_shape_kind += 1;
                if self.res.filters.pixel_shadow_shape_kind >= SHADOWS_LEN {
                    self.res.filters.pixel_shadow_shape_kind = 0;
                }
            } else {
                if self.res.filters.pixel_shadow_shape_kind == 0 {
                    self.res.filters.pixel_shadow_shape_kind = SHADOWS_LEN;
                }
                self.res.filters.pixel_shadow_shape_kind -= 1;
            }
            self.ctx
                .dispatcher
                .dispatch_top_message(&format!("Showing next pixel shadow: {}.", self.res.filters.pixel_shadow_shape_kind));
            self.ctx.dispatcher.dispatch_pixel_shadow_shape(self.res.filters.pixel_shadow_shape_kind);
        }

        FilterParams::new(ctx, &mut self.res.filters.pixel_shadow_height, self.input.next_pixels_shadow_height.clone())
            .set_progression(self.dt * 0.3)
            .set_event_value(read_event_value!(self, PixelShadowHeight, PIXEL_SHADOW_HEIGHT))
            .set_min(0.0)
            .set_max(1.0)
            .set_trigger_handler(|x| ctx.dispatcher.dispatch_pixel_shadow_height(x))
            .sum();
        let pixel_velocity = self.dt * self.res.filters.change_speed;
        FilterParams::new(ctx, &mut self.res.filters.cur_pixel_vertical_gap, self.input.pixel_vertical_gap.clone())
            .set_progression(pixel_velocity * 0.00125)
            .set_event_value(read_event_value!(self, PixelVerticalGap, PIXEL_VERTICAL_GAP))
            .set_min(0.0)
            .set_trigger_handler(|x| ctx.dispatcher.dispatch_change_pixel_vertical_gap(x))
            .sum();
        FilterParams::new(ctx, &mut self.res.filters.cur_pixel_horizontal_gap, self.input.pixel_horizontal_gap.clone())
            .set_progression(pixel_velocity * 0.00125)
            .set_event_value(read_event_value!(self, PixelHorizontalGap, PIXEL_HORIZONTAL_GAP))
            .set_min(0.0)
            .set_trigger_handler(|x| ctx.dispatcher.dispatch_change_pixel_horizontal_gap(x))
            .sum();
        FilterParams::new(ctx, &mut self.res.filters.cur_pixel_width, self.input.pixel_width.clone())
            .set_progression(pixel_velocity * 0.005)
            .set_event_value(read_event_value!(self, PixelWidth, PIXEL_WIDTH))
            .set_min(0.0)
            .set_trigger_handler(|x| ctx.dispatcher.dispatch_change_pixel_width(x))
            .sum();
        FilterParams::new(ctx, &mut self.res.filters.cur_pixel_spread, self.input.pixel_spread.clone())
            .set_progression(pixel_velocity * 0.005)
            .set_event_value(read_event_value!(self, PixelSpread, PIXEL_SPREAD))
            .set_min(0.0)
            .set_trigger_handler(|x| ctx.dispatcher.dispatch_change_pixel_spread(x))
            .sum();
    }

    fn update_speeds(&mut self) {
        let ctx = &self.ctx;
        FilterParams::new(ctx, &mut self.res.camera.turning_speed, self.input.turn_speed.to_is_just_pressed())
            .set_progression(2.0)
            .set_min(0.007_812_5 * TURNING_BASE_SPEED)
            .set_max(16_384.0 * TURNING_BASE_SPEED)
            .set_trigger_handler(|x| {
                let speed = (x / TURNING_BASE_SPEED * 1000.0).round() / 1000.0;
                ctx.dispatcher.dispatch_top_message(&format!("Turning camera speed: {}x", speed));
                ctx.dispatcher.dispatch_change_turning_speed(x / TURNING_BASE_SPEED);
            })
            .multiply();
        FilterParams::new(ctx, &mut self.res.filters.change_speed, self.input.filter_speed.to_is_just_pressed())
            .set_progression(2.0)
            .set_min(0.007_812_5 * PIXEL_MANIPULATION_BASE_SPEED)
            .set_max(16_384.0 * PIXEL_MANIPULATION_BASE_SPEED)
            .set_trigger_handler(|x| {
                let speed = (x / PIXEL_MANIPULATION_BASE_SPEED * 1000.0).round() / 1000.0;
                ctx.dispatcher.dispatch_top_message(&format!("Pixel manipulation speed: {}x", speed));
                ctx.dispatcher.dispatch_change_pixel_speed(x / PIXEL_MANIPULATION_BASE_SPEED);
            })
            .multiply();
        FilterParams::new(ctx, &mut self.res.camera.turning_speed, self.input.translation_speed.to_is_just_pressed())
            .set_progression(2.0)
            .set_min(0.007_812_5 * TURNING_BASE_SPEED)
            .set_max(16_384.0 * TURNING_BASE_SPEED)
            .set_trigger_handler(|x| {
                let speed = (x / TURNING_BASE_SPEED * 1000.0).round() / 1000.0;
                ctx.dispatcher.dispatch_top_message(&format!("Turning camera speed: {}x", speed));
                ctx.dispatcher.dispatch_change_turning_speed(x / TURNING_BASE_SPEED);
            })
            .multiply();
        let initial_movement_speed = self.res.initial_parameters.initial_movement_speed;
        FilterParams::new(ctx, &mut self.res.camera.movement_speed, self.input.translation_speed.to_is_just_pressed())
            .set_progression(2.0)
            .set_min(0.007_812_5 * initial_movement_speed)
            .set_max(16_384.0 * initial_movement_speed)
            .set_trigger_handler(|x| {
                let speed = (x / initial_movement_speed * 1000.0).round() / 1000.0;
                ctx.dispatcher.dispatch_top_message(&format!("Translation camera speed: {}x", speed));
                ctx.dispatcher.dispatch_change_movement_speed(x / initial_movement_speed);
            })
            .multiply();
        if self.input.reset_speeds {
            self.res.camera.turning_speed = TURNING_BASE_SPEED;
            self.res.camera.movement_speed = initial_movement_speed;
            self.res.filters.change_speed = PIXEL_MANIPULATION_BASE_SPEED;
            self.ctx.dispatcher.dispatch_top_message("All speeds have been reset.");
            self.change_frontend_input_values();
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
        self.ctx
            .dispatcher
            .dispatch_change_pixel_horizontal_gap(self.res.filters.cur_pixel_horizontal_gap);
        self.ctx.dispatcher.dispatch_change_pixel_vertical_gap(self.res.filters.cur_pixel_vertical_gap);
        self.ctx.dispatcher.dispatch_change_pixel_width(self.res.filters.cur_pixel_width);
        self.ctx.dispatcher.dispatch_change_pixel_spread(self.res.filters.cur_pixel_spread);
        self.ctx.dispatcher.dispatch_change_pixel_brightness(self.res.filters.extra_bright);
        self.ctx.dispatcher.dispatch_change_pixel_contrast(self.res.filters.extra_contrast);
        self.ctx.dispatcher.dispatch_change_light_color(self.res.filters.light_color);
        self.ctx.dispatcher.dispatch_change_brightness_color(self.res.filters.brightness_color);
        self.ctx.dispatcher.dispatch_change_camera_zoom(self.res.camera.zoom);
        self.ctx.dispatcher.dispatch_change_camera_movement_mode(self.res.camera.locked_mode);
        self.ctx.dispatcher.dispatch_change_blur_level(self.res.filters.blur_passes);
        self.ctx.dispatcher.dispatch_change_lines_per_pixel(self.res.filters.lines_per_pixel);
        self.ctx.dispatcher.dispatch_color_representation(self.res.filters.color_channels);
        self.ctx.dispatcher.dispatch_pixel_geometry(self.res.filters.pixels_geometry_kind);
        self.ctx.dispatcher.dispatch_pixel_shadow_shape(self.res.filters.pixel_shadow_shape_kind);
        self.ctx.dispatcher.dispatch_pixel_shadow_height(self.res.filters.pixel_shadow_height);
        self.ctx.dispatcher.dispatch_screen_layering_type(self.res.filters.layering_kind);
        self.ctx.dispatcher.dispatch_screen_curvature(self.res.filters.screen_curvature_kind);
        self.ctx.dispatcher.dispatch_internal_resolution(&self.res.filters.internal_resolution);
        self.ctx.dispatcher.dispatch_texture_interpolation(self.res.filters.texture_interpolation);
        self.ctx
            .dispatcher
            .dispatch_change_pixel_speed(self.res.filters.change_speed / PIXEL_MANIPULATION_BASE_SPEED);
        self.ctx
            .dispatcher
            .dispatch_change_turning_speed(self.res.camera.turning_speed / TURNING_BASE_SPEED);
        self.ctx
            .dispatcher
            .dispatch_change_movement_speed(self.res.camera.movement_speed / self.res.initial_parameters.initial_movement_speed);
    }

    fn update_outputs(&mut self) {
        self.update_output_filter_source_colors();
        self.update_output_filter_curvature();
        self.update_output_filter_layering_kind();
        
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

    fn update_output_filter_layering_kind(&mut self) {
        let output = &mut self.res.output;
        let filters = &self.res.filters;

        let mut solid_color_weight = 1.0;
        match filters.layering_kind {
            ScreenLayeringKind::ShadowOnly => {
                output.showing_foreground = true;
                output.showing_background = false;
            }
            ScreenLayeringKind::SolidOnly => {
                output.showing_foreground = false;
                output.showing_background = true;
            }
            ScreenLayeringKind::DiffuseOnly => {
                output.showing_foreground = false;
                output.showing_background = true;
            }
            ScreenLayeringKind::ShadowWithSolidBackground75 => {
                output.showing_foreground = true;
                output.showing_background = true;
                solid_color_weight = 0.75;
            }
            ScreenLayeringKind::ShadowWithSolidBackground50 => {
                output.showing_foreground = true;
                output.showing_background = true;
                solid_color_weight = 0.5;
            }
            ScreenLayeringKind::ShadowWithSolidBackground25 => {
                output.showing_foreground = true;
                output.showing_background = true;
                solid_color_weight = 0.25;
            }
        };

        for i in 0..3 {
            output.light_color_background[i] *= solid_color_weight;
        }

        output.is_background_diffuse = output.showing_foreground
            || if let ScreenLayeringKind::DiffuseOnly = filters.layering_kind {
                true
            } else {
                false
            };
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

        let by_lpp = 1.0 / (filters.lines_per_pixel as f32);
        let vl_offset_beginning = -(filters.lines_per_pixel as f32 - 1.0) / 2.0;

        output.pixel_scale_background.resize_with(filters.lines_per_pixel, Default::default);
        output.pixel_offset_background.resize_with(filters.lines_per_pixel, Default::default);
        for vl_idx in 0..filters.lines_per_pixel {
            let pixel_offset = &mut output.pixel_offset_background[vl_idx];
            let pixel_scale = &mut output.pixel_scale_background[vl_idx];

            *pixel_offset = [0.0, 0.0, 0.0];
            *pixel_scale = [
                (filters.cur_pixel_vertical_gap + 1.0) / filters.cur_pixel_width,
                filters.cur_pixel_horizontal_gap + 1.0,
                (filters.cur_pixel_vertical_gap + filters.cur_pixel_vertical_gap) * 0.5 + 1.0,
            ];
            if filters.lines_per_pixel > 1 {
                let vl_cur_offset = vl_offset_beginning + vl_idx as f32;
                pixel_offset[0] = (pixel_offset[0] + vl_cur_offset * filters.cur_pixel_width) * by_lpp;
                pixel_scale[0] *= filters.lines_per_pixel as f32;
            }
        }

        output.pixel_scale_foreground.resize_with(filters.lines_per_pixel, Default::default);
        output.pixel_offset_foreground.resize_with(filters.lines_per_pixel, Default::default);
        for vl_idx in 0..filters.lines_per_pixel {
            for color_idx in 0..output.color_splits {
                let pixel_offset = &mut output.pixel_offset_foreground[vl_idx][color_idx];
                let pixel_scale = &mut output.pixel_scale_foreground[vl_idx][color_idx];
                *pixel_offset = output.pixel_offset_background[vl_idx];
                *pixel_scale = output.pixel_scale_background[vl_idx];
                match filters.color_channels {
                    ColorChannels::Combined => {}
                    _ => match filters.color_channels {
                        ColorChannels::SplitHorizontal => {
                            pixel_offset[0] +=
                                by_lpp * (color_idx as f32 - 1.0) * (1.0 / 3.0) * filters.cur_pixel_width / (filters.cur_pixel_vertical_gap + 1.0);
                            pixel_scale[0] *= output.color_splits as f32;
                        }
                        ColorChannels::Overlapping => {
                            pixel_offset[0] +=
                                by_lpp * (color_idx as f32 - 1.0) * (1.0 / 3.0) * filters.cur_pixel_width / (filters.cur_pixel_vertical_gap + 1.0);
                            pixel_scale[0] *= 1.5;
                        }
                        ColorChannels::SplitVertical => {
                            pixel_offset[1] += (color_idx as f32 - 1.0) * (1.0 / 3.0) / (filters.cur_pixel_horizontal_gap + 1.0);
                            pixel_scale[1] *= output.color_splits as f32;
                        }
                        _ => unreachable!(),
                    },
                }
            }
        }
    }
}

struct FilterParams<'a, T, TriggerHandler: Fn(T), Dispatcher: AppEventDispatcher> {
    var: &'a mut T,
    event_value: Option<T>,
    incdec: IncDec<bool>,
    velocity: Option<T>,
    min: Option<T>,
    max: Option<T>,
    ctx: &'a SimulationContext<Dispatcher>,
    trigger_handler: Option<TriggerHandler>,
}

impl<'a, T, TriggerHandler: Fn(T), Dispatcher: AppEventDispatcher> FilterParams<'a, T, TriggerHandler, Dispatcher> {
    pub fn new(ctx: &'a SimulationContext<Dispatcher>, var: &'a mut T, incdec: IncDec<bool>) -> Self {
        FilterParams {
            ctx,
            var,
            incdec,
            velocity: None,
            event_value: None,
            min: None,
            max: None,
            trigger_handler: None,
        }
    }
    pub fn set_event_value(mut self, event_value: Option<T>) -> Self {
        self.event_value = event_value;
        self
    }
    pub fn set_trigger_handler(mut self, trigger_handler: TriggerHandler) -> Self {
        self.trigger_handler = Some(trigger_handler);
        self
    }
}

impl<'a, T: PartialOrd + PartialEq, TriggerHandler: Fn(T), Dispatcher: AppEventDispatcher> FilterParams<'a, T, TriggerHandler, Dispatcher> {
    pub fn set_progression(mut self, velocity: T) -> Self {
        self.velocity = Some(velocity);
        self
    }
    pub fn set_min(mut self, min: T) -> Self {
        self.min = Some(min);
        self
    }
    pub fn set_max(mut self, max: T) -> Self {
        self.max = Some(max);
        self
    }
}

trait EnumFilter {
    fn iterate_variant(self);
}

impl<'a, T, TriggerHandler, Dispatcher> EnumFilter for FilterParams<'a, T, TriggerHandler, Dispatcher>
where
    T: FromPrimitive + ToPrimitive + EnumLen + Copy,
    TriggerHandler: Fn(T),
    Dispatcher: AppEventDispatcher,
{
    fn iterate_variant(self) {
        let mut changed = false;
        if self.incdec.increase {
            self.var.next_enum_variant();
            changed = true;
        }
        if self.incdec.decrease {
            self.var.previous_enum_variant();
            changed = true;
        }
        if let Some(val) = self.event_value {
            *self.var = val;
            changed = true;
        }
        if changed {
            if let Some(ref handler) = self.trigger_handler {
                handler(*self.var);
            }
        }
    }
}

trait SumFilter {
    fn sum(self);
}

impl<'a, T, TriggerHandler, Dispatcher> SumFilter for FilterParams<'a, T, TriggerHandler, Dispatcher>
where
    T: Display + Add<Output = T> + Sub<Output = T> + PartialOrd + PartialEq + Copy + Default,
    TriggerHandler: Fn(T),
    Dispatcher: AppEventDispatcher,
{
    fn sum(self) {
        operate_filter(self, |a, b| a + b, |a, b| a - b)
    }
}

trait MultiplyFilter {
    fn multiply(self);
}

impl<'a, T, TriggerHandler, Dispatcher> MultiplyFilter for FilterParams<'a, T, TriggerHandler, Dispatcher>
where
    T: Display + Mul<Output = T> + Div<Output = T> + PartialOrd + PartialEq + Copy + Default,
    TriggerHandler: Fn(T),
    Dispatcher: AppEventDispatcher,
{
    fn multiply(self) {
        operate_filter(self, |a, b| a * b, |a, b| a / b)
    }
}

fn operate_filter<T, TriggerHandler, Dispatcher>(params: FilterParams<T, TriggerHandler, Dispatcher>, inc_op: impl Fn(T, T) -> T, dec_op: impl Fn(T, T) -> T)
where
    T: Display + PartialOrd + PartialEq + Copy + Default,
    TriggerHandler: Fn(T),
    Dispatcher: AppEventDispatcher,
{
    let last_value = *params.var;
    let velocity = if let Some(velocity) = params.velocity { velocity } else { Default::default() };
    if params.incdec.increase {
        *params.var = inc_op(*params.var, velocity);
    }
    if params.incdec.decrease {
        *params.var = dec_op(*params.var, velocity);
    }
    if let Some(val) = params.event_value {
        *params.var = val;
    }
    if last_value != *params.var {
        if let Some(min) = params.min {
            if *params.var < min {
                *params.var = min;
                params.ctx.dispatcher.dispatch_top_message(&format!("Minimum value is {}", min));
            }
        }
        if let Some(max) = params.max {
            if *params.var > max {
                *params.var = max;
                params.ctx.dispatcher.dispatch_top_message(&format!("Maximum value is {}", max));
            }
        }
        if let Some(ref handler) = params.trigger_handler {
            handler(*params.var);
        }
    }
}
