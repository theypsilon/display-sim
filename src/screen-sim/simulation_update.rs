use crate::app_events;
use crate::boolean_button::BooleanButton;
use crate::camera::CameraDirection;
use crate::general_types::NextEnumVariant;
use crate::pixels_shadow::SHADOWS_LEN;
use crate::simulation_state::{
    CustomInputEvent, Filters, IncDec, Input, Resources, ScreenCurvatureKind, ScreenLayeringKind, PIXEL_MANIPULATION_BASE_SPEED, TURNING_BASE_SPEED,
};
use crate::wasm_error::WasmResult;

pub fn update_simulation(res: &mut Resources, input: &Input) -> WasmResult<bool> {
    let dt = update_timers_and_dt(res, input)?;

    update_animation_buffer(res, input);
    update_colors(dt, res, input)?;
    update_blur(res, input)?;
    update_lpp(res, input)?;

    if input.esc.is_just_pressed() {
        app_events::dispatch_exiting_session()?;
        return Ok(false);
    }

    if input.space.is_just_pressed() {
        app_events::dispatch_toggle_info_panel()?;
    }

    update_pixel_pulse(dt, res, input)?;
    update_filters(dt, res, input)?;
    update_speeds(res, input)?;
    update_camera(dt, res, input)?;

    res.launch_screenshot = false;
    if res.screenshot_delay > 0 {
        res.screenshot_delay -= 1;
    } else if input.screenshot.is_just_released() {
        res.launch_screenshot = true;
        res.screenshot_delay = (5.0 * res.filters.internal_resolution.multiplier as f32 * (1.0 / dt)) as i32; // 5 seconds aprox.
    }

    Ok(true)
}

fn update_timers_and_dt(res: &mut Resources, input: &Input) -> WasmResult<f32> {
    let dt: f32 = ((input.now - res.timers.last_time) / 1000.0) as f32;
    let ellapsed = input.now - res.timers.last_second;
    res.timers.last_time = input.now;

    if ellapsed >= 1_000.0 {
        let fps = res.timers.frame_count as f32;
        app_events::dispatch_fps(fps)?;
        res.timers.last_second = input.now;
        res.timers.frame_count = 0;
    } else {
        res.timers.frame_count += 1;
    }
    Ok(dt)
}

fn update_animation_buffer(res: &mut Resources, input: &Input) {
    res.video.needs_buffer_data_load = res.resetted;
    let next_frame_update = res.video.last_frame_change + 0.001 * f64::from(res.video.steps[res.video.current_frame].delay);
    if input.now >= next_frame_update {
        res.video.last_frame_change = next_frame_update;
        let last_frame = res.video.current_frame;
        res.video.current_frame += 1;
        if res.video.current_frame >= res.video.steps.len() {
            res.video.current_frame = 0;
        }
        if last_frame != res.video.current_frame {
            res.video.needs_buffer_data_load = true;
        }
    }
    res.resetted = false;
}

fn update_colors(dt: f32, res: &mut Resources, input: &Input) -> WasmResult<()> {
    if input.bright.increase {
        res.filters.extra_bright += 0.01 * dt * res.filters.change_speed;
    }
    if input.bright.decrease {
        res.filters.extra_bright -= 0.01 * dt * res.filters.change_speed;
    }
    if input.bright.increase || input.bright.decrease {
        if res.filters.extra_bright < -1.0 {
            res.filters.extra_bright = -1.0;
            app_events::dispatch_top_message("Minimum value is -1.0".into())?;
        } else if res.filters.extra_bright > 1.0 {
            res.filters.extra_bright = 1.0;
            app_events::dispatch_top_message("Maximum value is +1.0".into())?;
        } else {
            app_events::dispatch_change_pixel_brightness(&res)?;
        }
    }
    if input.contrast.increase {
        res.filters.extra_contrast += 0.01 * dt * res.filters.change_speed;
    }
    if input.contrast.decrease {
        res.filters.extra_contrast -= 0.01 * dt * res.filters.change_speed;
    }
    if input.contrast.increase || input.contrast.decrease {
        if res.filters.extra_contrast < 0.0 {
            res.filters.extra_contrast = 0.0;
            app_events::dispatch_top_message("Minimum value is 0.0".into())?;
        } else if res.filters.extra_contrast > 20.0 {
            res.filters.extra_contrast = 20.0;
            app_events::dispatch_top_message("Maximum value is 20.0".into())?;
        } else {
            app_events::dispatch_change_pixel_contrast(&res)?;
        }
    }
    let color_variable = match input.custom_event.kind.as_ref() {
        "event_kind:pixel_brightness" => {
            res.filters.extra_bright = input.custom_event.value.as_f64().ok_or("it should be a number")? as f32;
            return Ok(());
        }
        "event_kind:pixel_contrast" => {
            res.filters.extra_contrast = input.custom_event.value.as_f64().ok_or("it should be a number")? as f32;
            return Ok(());
        }
        "event_kind:light_color" => &mut res.filters.light_color,
        "event_kind:brightness_color" => &mut res.filters.brightness_color,
        _ => return Ok(()),
    };

    let color_pick = input.custom_event.value.as_f64().ok_or("it should be a number")? as i32;
    if color_pick != *color_variable {
        *color_variable = color_pick;
        app_events::dispatch_top_message("Color changed.".into())?;
    }

    Ok(())
}

fn update_blur(res: &mut Resources, input: &Input) -> WasmResult<()> {
    let last_blur_passes = res.filters.blur_passes;
    if input.blur.increase.is_just_pressed() {
        res.filters.blur_passes += 1;
    }
    if input.blur.decrease.is_just_pressed() {
        if res.filters.blur_passes > 0 {
            res.filters.blur_passes -= 1;
        } else {
            app_events::dispatch_top_message("Minimum value is 0".into())?;
        }
    }
    if input.custom_event.kind.as_ref() as &str == "event_kind:blur_level" {
        res.filters.blur_passes = input.custom_event.value.as_f64().ok_or("it should be a number")? as usize;
        app_events::dispatch_change_blur_level(res)?;
    }
    if res.filters.blur_passes > 100 {
        res.filters.blur_passes = 100;
        app_events::dispatch_top_message("Maximum value is 100".into())?;
    }
    if last_blur_passes != res.filters.blur_passes {
        app_events::dispatch_top_message(format!("Blur level: {}", res.filters.blur_passes))?;
        app_events::dispatch_change_blur_level(res)?;
    }
    Ok(())
}

// lines per pixel
fn update_lpp(res: &mut Resources, input: &Input) -> WasmResult<()> {
    let last_lpp = res.filters.lines_per_pixel;
    if input.lpp.increase.is_just_pressed() {
        res.filters.lines_per_pixel += 1;
    }
    if input.lpp.decrease.is_just_pressed() && res.filters.lines_per_pixel > 0 {
        res.filters.lines_per_pixel -= 1;
    }
    if input.custom_event.kind.as_ref() as &str == "event_kind:lines_per_pixel" {
        res.filters.lines_per_pixel = input.custom_event.value.as_f64().ok_or("it should be a number")? as usize;
        app_events::dispatch_change_lines_per_pixel(res)?;
    }
    if res.filters.lines_per_pixel < 1 {
        res.filters.lines_per_pixel = 1;
        app_events::dispatch_top_message("Minimum value is 1".into())?;
    } else if res.filters.lines_per_pixel > 20 {
        res.filters.lines_per_pixel = 20;
        app_events::dispatch_top_message("Maximum value is 20".into())?;
    }
    if last_lpp != res.filters.lines_per_pixel {
        app_events::dispatch_top_message(format!("Lines per pixel: {}.", res.filters.lines_per_pixel))?;
        app_events::dispatch_change_lines_per_pixel(res)?;
    }
    Ok(())
}

fn update_pixel_pulse(dt: f32, res: &mut Resources, input: &Input) -> WasmResult<()> {
    if input.next_screen_curvature_type.any_just_pressed() {
        if input.next_screen_curvature_type.increase.is_just_pressed() {
            res.filters.screen_curvature_kind.next_enum_variant()?;
        } else {
            res.filters.screen_curvature_kind.previous_enum_variant()?;
        }
        app_events::dispatch_top_message(format!("Screen curvature: {}.", res.filters.screen_curvature_kind))?;
        app_events::dispatch_screen_curvature(res)?;
    }

    res.output.screen_curvature_factor = match res.filters.screen_curvature_kind {
        ScreenCurvatureKind::Curved1 => 0.15,
        ScreenCurvatureKind::Curved2 => 0.3,
        ScreenCurvatureKind::Curved3 => 0.45,
        _ => 0.0,
    };

    if let ScreenCurvatureKind::Pulse = res.filters.screen_curvature_kind {
        res.output.pixels_pulse += dt * 0.3;
    } else {
        res.output.pixels_pulse = 0.0;
    }
    Ok(())
}

fn update_filters(dt: f32, res: &mut Resources, input: &Input) -> WasmResult<()> {
    if input.reset_filters {
        res.filters = Filters::new(PIXEL_MANIPULATION_BASE_SPEED);
        res.filters.cur_pixel_width = res.initial_parameters.initial_pixel_width;
        change_frontend_input_values(res)?;
        app_events::dispatch_top_message("All filter options have been reset.".into())?;
        return Ok(());
    }

    if input.next_layering_kind.any_just_pressed() {
        if input.next_layering_kind.increase.is_just_pressed() {
            res.filters.layering_kind.next_enum_variant()?;
        } else {
            res.filters.layering_kind.previous_enum_variant()?;
        }
        app_events::dispatch_top_message(format!("Layering kind: {}.", res.filters.layering_kind))?;
        app_events::dispatch_screen_layering_type(res)?;
    }

    match res.filters.layering_kind {
        ScreenLayeringKind::ShadowOnly => {
            res.output.showing_diffuse_foreground = true;
            res.output.showing_solid_background = false;
        }
        ScreenLayeringKind::SolidOnly => {
            res.output.showing_diffuse_foreground = false;
            res.output.showing_solid_background = true;
            res.output.solid_color_weight = 1.0;
        }
        ScreenLayeringKind::DiffuseOnly => {
            res.output.showing_diffuse_foreground = false;
            res.output.showing_solid_background = true;
            res.output.solid_color_weight = 1.0;
        }
        ScreenLayeringKind::ShadowWithSolidBackground75 => {
            res.output.showing_diffuse_foreground = true;
            res.output.showing_solid_background = true;
            res.output.solid_color_weight = 0.75;
        }
        ScreenLayeringKind::ShadowWithSolidBackground50 => {
            res.output.showing_diffuse_foreground = true;
            res.output.showing_solid_background = true;
            res.output.solid_color_weight = 0.5;
        }
        ScreenLayeringKind::ShadowWithSolidBackground25 => {
            res.output.showing_diffuse_foreground = true;
            res.output.showing_solid_background = true;
            res.output.solid_color_weight = 0.25;
        }
    };

    if input.next_color_representation_kind.any_just_pressed() {
        if input.next_color_representation_kind.increase.is_just_pressed() {
            res.filters.color_channels.next_enum_variant()?;
        } else {
            res.filters.color_channels.previous_enum_variant()?;
        }
        app_events::dispatch_top_message(format!("Pixel color representation: {}.", res.filters.color_channels))?;
        app_events::dispatch_color_representation(res)?;
    }

    if input.next_pixel_geometry_kind.any_just_pressed() {
        if input.next_pixel_geometry_kind.increase.is_just_pressed() {
            res.filters.pixels_geometry_kind.next_enum_variant()?;
        } else {
            res.filters.pixels_geometry_kind.previous_enum_variant()?;
        }
        app_events::dispatch_top_message(format!("Pixel geometry: {}.", res.filters.pixels_geometry_kind))?;
        app_events::dispatch_pixel_geometry(res)?;
    }

    if input.next_pixels_shadow_shape_kind.any_just_pressed() {
        if input.next_pixels_shadow_shape_kind.increase.is_just_pressed() {
            res.filters.pixel_shadow_shape_kind += 1;
            if res.filters.pixel_shadow_shape_kind >= SHADOWS_LEN {
                res.filters.pixel_shadow_shape_kind = 0;
            }
        } else {
            if res.filters.pixel_shadow_shape_kind == 0 {
                res.filters.pixel_shadow_shape_kind = SHADOWS_LEN;
            }
            res.filters.pixel_shadow_shape_kind -= 1;
        }
        app_events::dispatch_top_message(format!("Showing next pixel shadow: {}.", res.filters.pixel_shadow_shape_kind))?;
        app_events::dispatch_pixel_shadow_shape(res)?;
    }

    let received_pixel_shadow_height = input.custom_event.kind.as_ref() as &str == "event_kind:pixel_shadow_height";
    if input.next_pixels_shadow_height_factor.any_active() || received_pixel_shadow_height {
        if input.next_pixels_shadow_height_factor.increase {
            res.filters.pixel_shadow_height_factor += dt * 0.3;
        }
        if input.next_pixels_shadow_height_factor.decrease {
            res.filters.pixel_shadow_height_factor -= dt * 0.3;
        }
        if received_pixel_shadow_height {
            res.filters.pixel_shadow_height_factor = input.custom_event.value.as_f64().ok_or("it should be a number")? as f32;
        }
        if res.filters.pixel_shadow_height_factor < 0.0 {
            res.filters.pixel_shadow_height_factor = 0.0;
            app_events::dispatch_top_message("Minimum value is 0.0".into())?;
        }
        if res.filters.pixel_shadow_height_factor > 1.0 {
            res.filters.pixel_shadow_height_factor = 1.0;
            app_events::dispatch_top_message("Maximum value is 1.0".into())?;
        }
        app_events::dispatch_pixel_shadow_height(&res)?;
    }

    if input.next_internal_resolution.any_just_released() {
        if input.next_internal_resolution.increase.is_just_released() {
            res.filters.internal_resolution.increase();
        }
        if input.next_internal_resolution.decrease.is_just_released() {
            res.filters.internal_resolution.decrease();
        }
        if res.filters.internal_resolution.minimum_reached {
            app_events::dispatch_top_message("Minimum internal resolution has been reached.".into())?;
        } else {
            app_events::dispatch_internal_resolution(&res)?;
        }
    }

    if input.next_texture_interpolation.any_just_pressed() {
        if input.next_texture_interpolation.increase.is_just_pressed() {
            res.filters.texture_interpolation.next_enum_variant()?;
        }
        if input.next_texture_interpolation.decrease.is_just_pressed() {
            res.filters.texture_interpolation.previous_enum_variant()?;
        }
        app_events::dispatch_texture_interpolation(&res)?;
    }

    let pixel_velocity = dt * res.filters.change_speed;
    change_pixel_sizes(
        &input.custom_event,
        input.pixel_scale_x.clone(),
        &mut res.filters.cur_pixel_scale_x,
        pixel_velocity * 0.00125,
        app_events::dispatch_change_pixel_vertical_gap,
        "event_kind:pixel_vertical_gap",
    )?;
    change_pixel_sizes(
        &input.custom_event,
        input.pixel_scale_y.clone(),
        &mut res.filters.cur_pixel_scale_y,
        pixel_velocity * 0.00125,
        app_events::dispatch_change_pixel_horizontal_gap,
        "event_kind:pixel_horizontal_gap",
    )?;
    change_pixel_sizes(
        &input.custom_event,
        input.pixel_width.clone(),
        &mut res.filters.cur_pixel_width,
        pixel_velocity * 0.005,
        app_events::dispatch_change_pixel_width,
        "event_kind:pixel_width",
    )?;
    change_pixel_sizes(
        &input.custom_event,
        input.pixel_gap.clone(),
        &mut res.filters.cur_pixel_gap,
        pixel_velocity * 0.005,
        app_events::dispatch_change_pixel_spread,
        "event_kind:pixel_spread",
    )?;

    fn change_pixel_sizes(
        custom_event: &CustomInputEvent,
        controller: IncDec<bool>,
        cur_size: &mut f32,
        velocity: f32,
        dispatch_update: fn(f32) -> WasmResult<()>,
        event_kind: &str,
    ) -> WasmResult<()> {
        let before_size = *cur_size;
        if controller.increase {
            *cur_size += velocity;
        }
        if controller.decrease {
            *cur_size -= velocity;
        }
        if custom_event.kind.as_ref() as &str == event_kind {
            *cur_size = custom_event.value.as_f64().ok_or("it should be a number")? as f32;
        }
        if *cur_size != before_size {
            if *cur_size < 0.0 {
                *cur_size = 0.0;
                app_events::dispatch_top_message("Minimum value is 0.0".into())?;
            }
            let size = *cur_size;
            dispatch_update(size)?;
        }
        Ok(())
    }
    Ok(())
}

fn update_speeds(res: &mut Resources, input: &Input) -> WasmResult<()> {
    change_speed(
        &input.turn_speed,
        &mut res.camera.turning_speed,
        TURNING_BASE_SPEED,
        "Turning camera speed: ",
        app_events::dispatch_change_turning_speed,
    )?;

    change_speed(
        &input.filter_speed,
        &mut res.filters.change_speed,
        PIXEL_MANIPULATION_BASE_SPEED,
        "Pixel manipulation speed: ",
        app_events::dispatch_change_pixel_speed,
    )?;

    change_speed(
        &input.translation_speed,
        &mut res.camera.turning_speed,
        TURNING_BASE_SPEED,
        "Turning camera speed: ",
        app_events::dispatch_change_turning_speed,
    )?;
    change_speed(
        &input.translation_speed,
        &mut res.camera.movement_speed,
        res.initial_parameters.initial_movement_speed,
        "Translation camera speed: ",
        app_events::dispatch_change_movement_speed,
    )?;

    fn change_speed(
        speed: &IncDec<BooleanButton>,
        cur_speed: &mut f32,
        base_speed: f32,
        top_message: &str,
        dispatch_update: fn(f32) -> WasmResult<()>,
    ) -> WasmResult<()> {
        let before_speed = *cur_speed;
        if speed.increase.is_just_pressed() && *cur_speed < 10000.0 {
            *cur_speed *= 2.0;
        }
        if speed.decrease.is_just_pressed() && *cur_speed > 0.01 {
            *cur_speed /= 2.0;
        }
        if *cur_speed != before_speed {
            let speed = (*cur_speed / base_speed * 1000.0).round() / 1000.0;
            app_events::dispatch_top_message(format!("{}{}x", top_message, speed))?;
            dispatch_update(*cur_speed / base_speed)?;
        }
        Ok(())
    }

    if input.reset_speeds {
        res.camera.turning_speed = TURNING_BASE_SPEED;
        res.camera.movement_speed = res.initial_parameters.initial_movement_speed;
        res.filters.change_speed = PIXEL_MANIPULATION_BASE_SPEED;
        app_events::dispatch_top_message("All speeds have been reset.".into())?;
        change_frontend_input_values(res)?;
    }
    Ok(())
}

fn update_camera(dt: f32, res: &mut Resources, input: &Input) -> WasmResult<()> {
    if input.walk_left {
        res.camera.advance(CameraDirection::Left, dt);
    }
    if input.walk_right {
        res.camera.advance(CameraDirection::Right, dt);
    }
    if input.walk_up {
        res.camera.advance(CameraDirection::Up, dt);
    }
    if input.walk_down {
        res.camera.advance(CameraDirection::Down, dt);
    }
    if input.walk_forward {
        res.camera.advance(CameraDirection::Forward, dt);
    }
    if input.walk_backward {
        res.camera.advance(CameraDirection::Backward, dt);
    }

    if input.turn_left {
        res.camera.turn(CameraDirection::Left, dt);
    }
    if input.turn_right {
        res.camera.turn(CameraDirection::Right, dt);
    }
    if input.turn_up {
        res.camera.turn(CameraDirection::Up, dt);
    }
    if input.turn_down {
        res.camera.turn(CameraDirection::Down, dt);
    }

    if input.rotate_left {
        res.camera.rotate(CameraDirection::Left, dt);
    }
    if input.rotate_right {
        res.camera.rotate(CameraDirection::Right, dt);
    }

    if input.mouse_click.is_just_pressed() {
        app_events::dispatch_request_pointer_lock()?;
    } else if input.mouse_click.is_activated() {
        res.camera.drag(input.mouse_position_x, input.mouse_position_y);
    } else if input.mouse_click.is_just_released() {
        app_events::dispatch_exit_pointer_lock()?;
    }

    if input.camera_zoom.increase {
        res.camera.change_zoom(dt * -100.0)?;
    } else if input.camera_zoom.decrease {
        res.camera.change_zoom(dt * 100.0)?;
    } else if input.mouse_scroll_y != 0.0 {
        res.camera.change_zoom(input.mouse_scroll_y)?;
    }

    // @Refactor too much code for too little stuff done in this match.
    match input.custom_event.kind.as_ref() {
        "event_kind:camera_zoom" => {
            res.camera.zoom = input.custom_event.value.as_f64().ok_or("Wrong number")? as f32;
        }

        "event_kind:camera_pos_x" => {
            let mut position = res.camera.get_position();
            position.x = input.custom_event.value.as_f64().ok_or("Wrong number")? as f32;
            res.camera.set_position(position);
        }
        "event_kind:camera_pos_y" => {
            let mut position = res.camera.get_position();
            position.y = input.custom_event.value.as_f64().ok_or("Wrong number")? as f32;
            res.camera.set_position(position);
        }
        "event_kind:camera_pos_z" => {
            let mut position = res.camera.get_position();
            position.z = input.custom_event.value.as_f64().ok_or("Wrong number")? as f32;
            res.camera.set_position(position);
        }

        "event_kind:camera_axis_up_x" => {
            let mut axis_up = res.camera.get_axis_up();
            axis_up.x = input.custom_event.value.as_f64().ok_or("Wrong number")? as f32;
            res.camera.set_axis_up(axis_up);
        }
        "event_kind:camera_axis_up_y" => {
            let mut axis_up = res.camera.get_axis_up();
            axis_up.y = input.custom_event.value.as_f64().ok_or("Wrong number")? as f32;
            res.camera.set_axis_up(axis_up);
        }
        "event_kind:camera_axis_up_z" => {
            let mut axis_up = res.camera.get_axis_up();
            axis_up.z = input.custom_event.value.as_f64().ok_or("Wrong number")? as f32;
            res.camera.set_axis_up(axis_up);
        }

        "event_kind:camera_direction_x" => {
            let mut direction = res.camera.get_direction();
            direction.x = input.custom_event.value.as_f64().ok_or("Wrong number")? as f32;
            res.camera.set_direction(direction);
        }
        "event_kind:camera_direction_y" => {
            let mut direction = res.camera.get_direction();
            direction.y = input.custom_event.value.as_f64().ok_or("Wrong number")? as f32;
            res.camera.set_direction(direction);
        }
        "event_kind:camera_direction_z" => {
            let mut direction = res.camera.get_direction();
            direction.z = input.custom_event.value.as_f64().ok_or("Wrong number")? as f32;
            res.camera.set_direction(direction);
        }

        _ => {}
    }

    if input.reset_position {
        res.camera.set_position(glm::vec3(0.0, 0.0, res.initial_parameters.initial_position_z));
        res.camera.set_direction(glm::vec3(0.0, 0.0, -1.0));
        res.camera.set_axis_up(glm::vec3(0.0, 1.0, 0.0));
        res.camera.zoom = 45.0;
        app_events::dispatch_change_camera_zoom(&res)?;
        app_events::dispatch_top_message("The camera have been reset.".into())?;
    }

    res.camera.update_view()
}

pub fn change_frontend_input_values(res: &Resources) -> WasmResult<()> {
    app_events::dispatch_change_pixel_horizontal_gap(res.filters.cur_pixel_scale_y)?;
    app_events::dispatch_change_pixel_vertical_gap(res.filters.cur_pixel_scale_x)?;
    app_events::dispatch_change_pixel_width(res.filters.cur_pixel_width)?;
    app_events::dispatch_change_pixel_spread(res.filters.cur_pixel_gap)?;
    app_events::dispatch_change_pixel_brightness(res)?;
    app_events::dispatch_change_pixel_contrast(res)?;
    app_events::dispatch_change_light_color(res)?;
    app_events::dispatch_change_brightness_color(res)?;
    app_events::dispatch_change_camera_zoom(res)?;
    app_events::dispatch_change_blur_level(res)?;
    app_events::dispatch_change_lines_per_pixel(res)?;
    app_events::dispatch_color_representation(res)?;
    app_events::dispatch_pixel_geometry(res)?;
    app_events::dispatch_pixel_shadow_shape(res)?;
    app_events::dispatch_pixel_shadow_height(res)?;
    app_events::dispatch_screen_layering_type(res)?;
    app_events::dispatch_screen_curvature(res)?;
    app_events::dispatch_internal_resolution(res)?;
    app_events::dispatch_texture_interpolation(res)?;
    app_events::dispatch_change_pixel_speed(res.filters.change_speed / PIXEL_MANIPULATION_BASE_SPEED)?;
    app_events::dispatch_change_turning_speed(res.camera.turning_speed / TURNING_BASE_SPEED)?;
    app_events::dispatch_change_movement_speed(res.camera.movement_speed / res.initial_parameters.initial_movement_speed)?;
    Ok(())
}
