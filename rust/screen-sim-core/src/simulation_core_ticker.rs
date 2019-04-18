use crate::app_events::AppEventDispatcher;
use crate::boolean_button::BooleanButton;
use crate::camera::{CameraData, CameraDirection, CameraSystem};
use crate::general_types::{get_3_f32color_from_int, NextEnumVariant};
use crate::pixels_shadow::SHADOWS_LEN;
use crate::simulation_context::SimulationContext;
use crate::simulation_core_state::{
    ColorChannels, CustomInputEvent, Filters, IncDec, Input, InputEventValue, PixelsGeometryKind, Resources, ScreenCurvatureKind, ScreenLayeringKind,
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
        self.input.custom_event.kind = String::new();
    }
}

struct SimulationUpdater<'a, T: AppEventDispatcher> {
    ctx: &'a mut SimulationContext<T>,
    res: &'a mut Resources,
    input: &'a Input,
    dt: f32,
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
        self.update_filter_curvature();
        self.update_filter_source_colors();
        self.update_filter_blur();
        self.update_filter_lpp();
        self.update_filter_pixel_shape();
        self.update_filter_layering_kind();
        self.update_filter_color_representation();
        self.update_filter_internal_resolution();
        self.update_filter_texture_interpolation();
    }

    fn update_filter_source_colors(&mut self) {
        if self.input.bright.increase {
            self.res.filters.extra_bright += 0.01 * self.dt * self.res.filters.change_speed;
        }
        if self.input.bright.decrease {
            self.res.filters.extra_bright -= 0.01 * self.dt * self.res.filters.change_speed;
        }
        if self.input.bright.increase || self.input.bright.decrease {
            if self.res.filters.extra_bright < -1.0 {
                self.res.filters.extra_bright = -1.0;
                self.ctx.dispatcher.dispatch_top_message("Minimum value is -1.0");
            } else if self.res.filters.extra_bright > 1.0 {
                self.res.filters.extra_bright = 1.0;
                self.ctx.dispatcher.dispatch_top_message("Maximum value is +1.0");
            } else {
                self.ctx.dispatcher.dispatch_change_pixel_brightness(self.res);
            }
        }
        if self.input.contrast.increase {
            self.res.filters.extra_contrast += 0.01 * self.dt * self.res.filters.change_speed;
        }
        if self.input.contrast.decrease {
            self.res.filters.extra_contrast -= 0.01 * self.dt * self.res.filters.change_speed;
        }
        if self.input.contrast.increase || self.input.contrast.decrease {
            if self.res.filters.extra_contrast < 0.0 {
                self.res.filters.extra_contrast = 0.0;
                self.ctx.dispatcher.dispatch_top_message("Minimum value is 0.0");
            } else if self.res.filters.extra_contrast > 20.0 {
                self.res.filters.extra_contrast = 20.0;
                self.ctx.dispatcher.dispatch_top_message("Maximum value is 20.0");
            } else {
                self.ctx.dispatcher.dispatch_change_pixel_contrast(self.res);
            }
        }

        let (color_pick, color_variable) = match self.input.custom_event.value {
            InputEventValue::PixelBrighttness(brightness) => {
                self.res.filters.extra_bright = brightness;
                return;
            }
            InputEventValue::PixelContrast(contrast) => {
                self.res.filters.extra_contrast = contrast;
                return;
            }
            InputEventValue::LightColor(light_color) => (light_color, &mut self.res.filters.light_color),
            InputEventValue::BrightnessColor(brightness_color) => (brightness_color, &mut self.res.filters.brightness_color),
            _ => return,
        };
        if color_pick != *color_variable {
            *color_variable = color_pick;
            self.ctx.dispatcher.dispatch_top_message("Color changed.");
        }
    }

    fn update_filter_blur(&mut self) {
        let last_blur_passes = self.res.filters.blur_passes;
        if self.input.blur.increase.is_just_pressed() {
            self.res.filters.blur_passes += 1;
        }
        if self.input.blur.decrease.is_just_pressed() {
            if self.res.filters.blur_passes > 0 {
                self.res.filters.blur_passes -= 1;
            } else {
                self.ctx.dispatcher.dispatch_top_message("Minimum value is 0");
            }
        }
        if let InputEventValue::BlurLevel(blur_passes) = self.input.custom_event.value {
            self.res.filters.blur_passes = blur_passes;
            self.ctx.dispatcher.dispatch_change_blur_level(self.res);
        }
        if self.res.filters.blur_passes > 100 {
            self.res.filters.blur_passes = 100;
            self.ctx.dispatcher.dispatch_top_message("Maximum value is 100");
        }
        if last_blur_passes != self.res.filters.blur_passes {
            self.ctx
                .dispatcher
                .dispatch_top_message(&format!("Blur level: {}", self.res.filters.blur_passes));
            self.ctx.dispatcher.dispatch_change_blur_level(self.res);
        }
    }

    // lines per pixel
    fn update_filter_lpp(&mut self) {
        let last_lpp = self.res.filters.lines_per_pixel;
        if self.input.lpp.increase.is_just_pressed() {
            self.res.filters.lines_per_pixel += 1;
        }
        if self.input.lpp.decrease.is_just_pressed() && self.res.filters.lines_per_pixel > 0 {
            self.res.filters.lines_per_pixel -= 1;
        }
        if let InputEventValue::LinersPerPixel(lpp) = self.input.custom_event.value {
            self.res.filters.lines_per_pixel = lpp;
            self.ctx.dispatcher.dispatch_change_lines_per_pixel(self.res);
        }
        if self.res.filters.lines_per_pixel < 1 {
            self.res.filters.lines_per_pixel = 1;
            self.ctx.dispatcher.dispatch_top_message("Minimum value is 1");
        } else if self.res.filters.lines_per_pixel > 20 {
            self.res.filters.lines_per_pixel = 20;
            self.ctx.dispatcher.dispatch_top_message("Maximum value is 20");
        }
        if last_lpp != self.res.filters.lines_per_pixel {
            self.ctx
                .dispatcher
                .dispatch_top_message(&format!("Lines per pixel: {}.", self.res.filters.lines_per_pixel));
            self.ctx.dispatcher.dispatch_change_lines_per_pixel(self.res);
        }
    }

    fn update_filter_curvature(&mut self) {
        if self.input.next_screen_curvature_type.any_just_pressed() {
            if self.input.next_screen_curvature_type.increase.is_just_pressed() {
                self.res.filters.screen_curvature_kind.next_enum_variant();
            } else {
                self.res.filters.screen_curvature_kind.previous_enum_variant();
            }
            self.ctx
                .dispatcher
                .dispatch_top_message(&format!("Screen curvature: {}.", self.res.filters.screen_curvature_kind));
            self.ctx.dispatcher.dispatch_screen_curvature(self.res);
        }
    }

    fn update_filter_layering_kind(&mut self) {
        if self.input.next_layering_kind.any_just_pressed() {
            if self.input.next_layering_kind.increase.is_just_pressed() {
                self.res.filters.layering_kind.next_enum_variant();
            } else {
                self.res.filters.layering_kind.previous_enum_variant();
            }
            self.ctx
                .dispatcher
                .dispatch_top_message(&format!("Layering kind: {}.", self.res.filters.layering_kind));
            self.ctx.dispatcher.dispatch_screen_layering_type(self.res);
        }
    }

    fn update_filter_color_representation(&mut self) {
        if self.input.next_color_representation_kind.any_just_pressed() {
            if self.input.next_color_representation_kind.increase.is_just_pressed() {
                self.res.filters.color_channels.next_enum_variant();
            } else {
                self.res.filters.color_channels.previous_enum_variant();
            }
            self.ctx
                .dispatcher
                .dispatch_top_message(&format!("Pixel color representation: {}.", self.res.filters.color_channels));
            self.ctx.dispatcher.dispatch_color_representation(self.res);
        }
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
                self.ctx.dispatcher.dispatch_internal_resolution(self.res);
            }
        }
    }

    fn update_filter_texture_interpolation(&mut self) {
        if self.input.next_texture_interpolation.any_just_pressed() {
            if self.input.next_texture_interpolation.increase.is_just_pressed() {
                self.res.filters.texture_interpolation.next_enum_variant();
            }
            if self.input.next_texture_interpolation.decrease.is_just_pressed() {
                self.res.filters.texture_interpolation.previous_enum_variant();
            }
            self.ctx.dispatcher.dispatch_texture_interpolation(self.res);
        }
    }

    fn update_filter_pixel_shape(&mut self) {
        if self.input.next_pixel_geometry_kind.any_just_pressed() {
            if self.input.next_pixel_geometry_kind.increase.is_just_pressed() {
                self.res.filters.pixels_geometry_kind.next_enum_variant();
            } else {
                self.res.filters.pixels_geometry_kind.previous_enum_variant();
            }
            self.ctx
                .dispatcher
                .dispatch_top_message(&format!("Pixel geometry: {}.", self.res.filters.pixels_geometry_kind));
            self.ctx.dispatcher.dispatch_pixel_geometry(self.res);
        }

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
            self.ctx.dispatcher.dispatch_pixel_shadow_shape(self.res);
        }

        let received_pixel_shadow_height = if let InputEventValue::PixelShadowHeight(height) = self.input.custom_event.value {
            Some(height)
        } else {
            None
        };

        if self.input.next_pixels_shadow_height_factor.any_active() || received_pixel_shadow_height.is_some() {
            if self.input.next_pixels_shadow_height_factor.increase {
                self.res.filters.pixel_shadow_height_factor += self.dt * 0.3;
            }
            if self.input.next_pixels_shadow_height_factor.decrease {
                self.res.filters.pixel_shadow_height_factor -= self.dt * 0.3;
            }
            if let Some(height) = received_pixel_shadow_height {
                self.res.filters.pixel_shadow_height_factor = height;
            }
            if self.res.filters.pixel_shadow_height_factor < 0.0 {
                self.res.filters.pixel_shadow_height_factor = 0.0;
                self.ctx.dispatcher.dispatch_top_message("Minimum value is 0.0");
            }
            if self.res.filters.pixel_shadow_height_factor > 1.0 {
                self.res.filters.pixel_shadow_height_factor = 1.0;
                self.ctx.dispatcher.dispatch_top_message("Maximum value is 1.0");
            }
            self.ctx.dispatcher.dispatch_pixel_shadow_height(self.res);
        }

        let pixel_velocity = self.dt * self.res.filters.change_speed;
        let ctx = &self.ctx;
        change_pixel_sizes(
            ctx,
            &self.input.custom_event,
            self.input.pixel_vertical_gap.clone(),
            &mut self.res.filters.cur_pixel_vertical_gap,
            pixel_velocity * 0.00125,
            |n| ctx.dispatcher.dispatch_change_pixel_vertical_gap(n),
            "event_kind:pixel_vertical_gap",
        );
        change_pixel_sizes(
            ctx,
            &self.input.custom_event,
            self.input.pixel_horizontal_gap.clone(),
            &mut self.res.filters.cur_pixel_horizontal_gap,
            pixel_velocity * 0.00125,
            |n| ctx.dispatcher.dispatch_change_pixel_horizontal_gap(n),
            "event_kind:pixel_horizontal_gap",
        );
        change_pixel_sizes(
            ctx,
            &self.input.custom_event,
            self.input.pixel_width.clone(),
            &mut self.res.filters.cur_pixel_width,
            pixel_velocity * 0.005,
            |n| ctx.dispatcher.dispatch_change_pixel_width(n),
            "event_kind:pixel_width",
        );
        change_pixel_sizes(
            ctx,
            &self.input.custom_event,
            self.input.pixel_spread.clone(),
            &mut self.res.filters.cur_pixel_spread,
            pixel_velocity * 0.005,
            |n| ctx.dispatcher.dispatch_change_pixel_spread(n),
            "event_kind:pixel_spread",
        );

        fn change_pixel_sizes<T: AppEventDispatcher>(
            ctx: &SimulationContext<T>,
            custom_event: &CustomInputEvent,
            controller: IncDec<bool>,
            cur_size: &mut f32,
            velocity: f32,
            dispatch_update: impl Fn(f32),
            event_kind: &str,
        ) {
            let before_size = *cur_size;
            if controller.increase {
                *cur_size += velocity;
            }
            if controller.decrease {
                *cur_size -= velocity;
            }
            if custom_event.kind.as_ref() as &str == event_kind {
                *cur_size = custom_event.get_f32();
            }
            if *cur_size != before_size {
                if *cur_size < 0.0 {
                    *cur_size = 0.0;
                    ctx.dispatcher.dispatch_top_message("Minimum value is 0.0");
                }
                let size = *cur_size;
                dispatch_update(size);
            }
        }
    }

    fn update_speeds(&mut self) {
        let ctx = &self.ctx;
        change_speed(
            self.ctx,
            &self.input.turn_speed,
            &mut self.res.camera.turning_speed,
            TURNING_BASE_SPEED,
            "Turning camera speed: ",
            |n| ctx.dispatcher.dispatch_change_turning_speed(n),
        );

        change_speed(
            self.ctx,
            &self.input.filter_speed,
            &mut self.res.filters.change_speed,
            PIXEL_MANIPULATION_BASE_SPEED,
            "Pixel manipulation speed: ",
            |n| ctx.dispatcher.dispatch_change_pixel_speed(n),
        );

        change_speed(
            self.ctx,
            &self.input.translation_speed,
            &mut self.res.camera.turning_speed,
            TURNING_BASE_SPEED,
            "Turning camera speed: ",
            |n| ctx.dispatcher.dispatch_change_turning_speed(n),
        );
        change_speed(
            self.ctx,
            &self.input.translation_speed,
            &mut self.res.camera.movement_speed,
            self.res.initial_parameters.initial_movement_speed,
            "Translation camera speed: ",
            |n| ctx.dispatcher.dispatch_change_movement_speed(n),
        );

        fn change_speed<T: AppEventDispatcher>(
            ctx: &SimulationContext<T>,
            speed: &IncDec<BooleanButton>,
            cur_speed: &mut f32,
            base_speed: f32,
            top_message: &str,
            dispatch_update: impl Fn(f32),
        ) {
            let before_speed = *cur_speed;
            if speed.increase.is_just_pressed() && *cur_speed < 10000.0 {
                *cur_speed *= 2.0;
            }
            if speed.decrease.is_just_pressed() && *cur_speed > 0.01 {
                *cur_speed /= 2.0;
            }
            if *cur_speed != before_speed {
                let speed = (*cur_speed / base_speed * 1000.0).round() / 1000.0;
                ctx.dispatcher.dispatch_top_message(&format!("{}{}x", top_message, speed));
                dispatch_update(*cur_speed / base_speed);
            }
        }

        if self.input.reset_speeds {
            self.res.camera.turning_speed = TURNING_BASE_SPEED;
            self.res.camera.movement_speed = self.res.initial_parameters.initial_movement_speed;
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

        if let InputEventValue::Camera(change) = self.input.custom_event.value {
            camera.handle_camera_change(change);
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
        self.ctx.dispatcher.dispatch_change_pixel_brightness(self.res);
        self.ctx.dispatcher.dispatch_change_pixel_contrast(self.res);
        self.ctx.dispatcher.dispatch_change_light_color(self.res);
        self.ctx.dispatcher.dispatch_change_brightness_color(self.res);
        self.ctx.dispatcher.dispatch_change_camera_zoom(self.res.camera.zoom);
        self.ctx.dispatcher.dispatch_change_camera_movement_mode(self.res.camera.locked_mode);
        self.ctx.dispatcher.dispatch_change_blur_level(self.res);
        self.ctx.dispatcher.dispatch_change_lines_per_pixel(self.res);
        self.ctx.dispatcher.dispatch_color_representation(self.res);
        self.ctx.dispatcher.dispatch_pixel_geometry(self.res);
        self.ctx.dispatcher.dispatch_pixel_shadow_shape(self.res);
        self.ctx.dispatcher.dispatch_pixel_shadow_height(self.res);
        self.ctx.dispatcher.dispatch_screen_layering_type(self.res);
        self.ctx.dispatcher.dispatch_screen_curvature(self.res);
        self.ctx.dispatcher.dispatch_internal_resolution(self.res);
        self.ctx.dispatcher.dispatch_texture_interpolation(self.res);
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

        let (ambient_strength, pixel_have_depth) = match self.res.filters.pixels_geometry_kind {
            PixelsGeometryKind::Squares => (1.0, false),
            PixelsGeometryKind::Cubes => (0.5, true),
        };
        self.res.output.ambient_strength = ambient_strength;
        self.res.output.pixel_have_depth = pixel_have_depth;
        self.res.output.height_modifier_factor = 1.0 - self.res.filters.pixel_shadow_height_factor;

        self.update_output_pixel_scale_gap_offset();
    }

    fn update_output_filter_source_colors(&mut self) {
        self.res.output.color_splits = match self.res.filters.color_channels {
            ColorChannels::Combined => 1,
            _ => 3,
        };
        self.res.output.light_color_background = get_3_f32color_from_int(self.res.filters.light_color);
        for i in 0..self.res.output.color_splits {
            let mut light_color = self.res.output.light_color_background.clone();
            match self.res.filters.color_channels {
                ColorChannels::Combined => {}
                _ => {
                    light_color[(i + 0) % 3] *= 1.0;
                    light_color[(i + 1) % 3] = 0.0;
                    light_color[(i + 2) % 3] = 0.0;
                }
            }
            self.res.output.light_color[i] = light_color;
        }
        self.res.output.extra_light = get_3_f32color_from_int(self.res.filters.brightness_color);
        for light in self.res.output.extra_light.iter_mut() {
            *light *= self.res.filters.extra_bright;
        }
    }

    fn update_output_filter_curvature(&mut self) {
        self.res.output.screen_curvature_factor = match self.res.filters.screen_curvature_kind {
            ScreenCurvatureKind::Curved1 => 0.15,
            ScreenCurvatureKind::Curved2 => 0.3,
            ScreenCurvatureKind::Curved3 => 0.45,
            _ => 0.0,
        };

        if let ScreenCurvatureKind::Pulse = self.res.filters.screen_curvature_kind {
            self.res.output.pixels_pulse += self.dt * 0.3;
        } else {
            self.res.output.pixels_pulse = 0.0;
        }
    }

    fn update_output_filter_layering_kind(&mut self) {
        let mut solid_color_weight = 1.0;
        match self.res.filters.layering_kind {
            ScreenLayeringKind::ShadowOnly => {
                self.res.output.showing_foreground = true;
                self.res.output.showing_background = false;
            }
            ScreenLayeringKind::SolidOnly => {
                self.res.output.showing_foreground = false;
                self.res.output.showing_background = true;
            }
            ScreenLayeringKind::DiffuseOnly => {
                self.res.output.showing_foreground = false;
                self.res.output.showing_background = true;
            }
            ScreenLayeringKind::ShadowWithSolidBackground75 => {
                self.res.output.showing_foreground = true;
                self.res.output.showing_background = true;
                solid_color_weight = 0.75;
            }
            ScreenLayeringKind::ShadowWithSolidBackground50 => {
                self.res.output.showing_foreground = true;
                self.res.output.showing_background = true;
                solid_color_weight = 0.5;
            }
            ScreenLayeringKind::ShadowWithSolidBackground25 => {
                self.res.output.showing_foreground = true;
                self.res.output.showing_background = true;
                solid_color_weight = 0.25;
            }
        };

        for i in 0..3 {
            self.res.output.light_color_background[i] *= solid_color_weight;
        }

        self.res.output.is_background_diffuse = self.res.output.showing_foreground
            || if let ScreenLayeringKind::DiffuseOnly = self.res.filters.layering_kind {
                true
            } else {
                false
            };
    }

    fn update_output_pixel_scale_gap_offset(&mut self) {
        self.res.output.pixel_spread = [
            (1.0 + self.res.filters.cur_pixel_spread) * self.res.filters.cur_pixel_width,
            1.0 + self.res.filters.cur_pixel_spread,
        ];
        self.res.output.pixel_scale_base = [
            (self.res.filters.cur_pixel_vertical_gap + 1.0) / self.res.filters.cur_pixel_width,
            self.res.filters.cur_pixel_horizontal_gap + 1.0,
            (self.res.filters.cur_pixel_vertical_gap + self.res.filters.cur_pixel_vertical_gap) * 0.5 + 1.0,
        ];

        let by_lpp = 1.0 / (self.res.filters.lines_per_pixel as f32);
        let vl_offset_beginning = -(self.res.filters.lines_per_pixel as f32 - 1.0) / 2.0;

        self.res
            .output
            .pixel_scale_background
            .resize_with(self.res.filters.lines_per_pixel, Default::default);
        self.res
            .output
            .pixel_offset_background
            .resize_with(self.res.filters.lines_per_pixel, Default::default);
        for vl_idx in 0..self.res.filters.lines_per_pixel {
            let pixel_offset = &mut self.res.output.pixel_offset_background[vl_idx];
            let pixel_scale = &mut self.res.output.pixel_scale_background[vl_idx];

            *pixel_offset = [0.0, 0.0, 0.0];
            *pixel_scale = [
                (self.res.filters.cur_pixel_vertical_gap + 1.0) / self.res.filters.cur_pixel_width,
                self.res.filters.cur_pixel_horizontal_gap + 1.0,
                (self.res.filters.cur_pixel_vertical_gap + self.res.filters.cur_pixel_vertical_gap) * 0.5 + 1.0,
            ];
            if self.res.filters.lines_per_pixel > 1 {
                let vl_cur_offset = vl_offset_beginning + vl_idx as f32;
                pixel_offset[0] = (pixel_offset[0] + vl_cur_offset * self.res.filters.cur_pixel_width) * by_lpp;
                pixel_scale[0] *= self.res.filters.lines_per_pixel as f32;
            }
        }

        self.res
            .output
            .pixel_scale_foreground
            .resize_with(self.res.filters.lines_per_pixel, Default::default);
        self.res
            .output
            .pixel_offset_foreground
            .resize_with(self.res.filters.lines_per_pixel, Default::default);
        for vl_idx in 0..self.res.filters.lines_per_pixel {
            for color_idx in 0..self.res.output.color_splits {
                let pixel_offset = &mut self.res.output.pixel_offset_foreground[vl_idx][color_idx];
                let pixel_scale = &mut self.res.output.pixel_scale_foreground[vl_idx][color_idx];
                *pixel_offset = self.res.output.pixel_offset_background[vl_idx];
                *pixel_scale = self.res.output.pixel_scale_background[vl_idx];
                match self.res.filters.color_channels {
                    ColorChannels::Combined => {}
                    _ => match self.res.filters.color_channels {
                        ColorChannels::SplitHorizontal => {
                            pixel_offset[0] += by_lpp * (color_idx as f32 - 1.0) * (1.0 / 3.0) * self.res.filters.cur_pixel_width
                                / (self.res.filters.cur_pixel_vertical_gap + 1.0);
                            pixel_scale[0] *= self.res.output.color_splits as f32;
                        }
                        ColorChannels::Overlapping => {
                            pixel_offset[0] += by_lpp * (color_idx as f32 - 1.0) * (1.0 / 3.0) * self.res.filters.cur_pixel_width
                                / (self.res.filters.cur_pixel_vertical_gap + 1.0);
                            pixel_scale[0] *= 1.5;
                        }
                        ColorChannels::SplitVertical => {
                            pixel_offset[1] += (color_idx as f32 - 1.0) * (1.0 / 3.0) / (self.res.filters.cur_pixel_horizontal_gap + 1.0);
                            pixel_scale[1] *= self.res.output.color_splits as f32;
                        }
                        _ => unreachable!(),
                    },
                }
            }
        }
    }
}
