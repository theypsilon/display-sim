/* Copyright (c) 2019-2022 José manuel Barroso Galindo <theypsilon@gmail.com>
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

use crate::console;
use crate::dispatch_event::{dispatch_event, dispatch_event_with};
use app_util::{AppError, AppResult};
use core::app_events::AppEventDispatcher;
use core::camera::CameraLockMode;
use core::simulation_core_state::ScalingMethod;
use js_sys::Float32Array;
use std::cell::RefCell;
use std::fmt::Display;
use wasm_bindgen::JsValue;
use web_sys::WebGl2RenderingContext;

pub struct WebEventDispatcher {
    error: RefCell<Option<AppError>>,
    extra_messages_enabled: RefCell<bool>,
    gl: WebGl2RenderingContext,
    event_bus: JsValue,
}

impl WebEventDispatcher {
    pub fn new(gl: WebGl2RenderingContext, event_bus: JsValue) -> Self {
        WebEventDispatcher {
            error: Default::default(),
            extra_messages_enabled: RefCell::new(true),
            gl,
            event_bus,
        }
    }
}

impl AppEventDispatcher for WebEventDispatcher {
    fn enable_extra_messages(&self, extra_messages: bool) {
        *self.extra_messages_enabled.borrow_mut() = extra_messages;
    }

    fn are_extra_messages_enabled(&self) -> bool {
        *self.extra_messages_enabled.borrow()
    }

    fn dispatch_log(&self, msg: String) {
        console!(log.msg);
    }

    fn dispatch_string_event(&self, event_id: &'static str, message: &str) {
        self.catch_error(dispatch_event_with(&self.event_bus, event_id, &message.into()));
    }

    fn dispatch_camera_update(&self, position: &glm::Vec3, direction: &glm::Vec3, axis_up: &glm::Vec3) {
        let values_array = Float32Array::new(&wasm_bindgen::JsValue::from(9));
        values_array.fill(position.x, 0, 1);
        values_array.fill(position.y, 1, 2);
        values_array.fill(position.z, 2, 3);
        values_array.fill(direction.x, 3, 4);
        values_array.fill(direction.y, 4, 5);
        values_array.fill(direction.z, 5, 6);
        values_array.fill(axis_up.x, 6, 7);
        values_array.fill(axis_up.y, 7, 8);
        values_array.fill(axis_up.z, 8, 9);
        self.catch_error(dispatch_event_with(&self.event_bus, "back2front:camera_update", &values_array.into()));
    }

    fn dispatch_change_pixel_width(&self, size: f32) {
        self.catch_error(dispatch_event_with(
            &self.event_bus,
            "back2front:change_pixel_width",
            &format!("{:.03}", size).into(),
        ));
    }

    fn dispatch_change_camera_zoom(&self, zoom: f32) {
        self.catch_error(dispatch_event_with(
            &self.event_bus,
            "back2front:change_camera_zoom",
            &format!("{:.02}", zoom).into(),
        ));
    }

    fn dispatch_scaling_method(&self, method: ScalingMethod) {
        if self.are_extra_messages_enabled() {
            self.dispatch_top_message(&format!("Scaling method: {}.", method));
        }
        self.catch_error(dispatch_event_with(&self.event_bus, "back2front:scaling_method", &(method.to_string()).into()));
    }

    fn dispatch_scaling_resolution_width(&self, width: u32) {
        self.catch_error(dispatch_event_with(&self.event_bus, "back2front:scaling_resolution_width", &(width).into()));
    }

    fn dispatch_scaling_resolution_height(&self, height: u32) {
        self.catch_error(dispatch_event_with(&self.event_bus, "back2front:scaling_resolution_height", &(height).into()));
    }

    fn dispatch_scaling_aspect_ratio_x(&self, x: f32) {
        self.catch_error(dispatch_event_with(
            &self.event_bus,
            "back2front:scaling_aspect_ratio_x",
            &(if x.floor() == x { format!("{:.00}", x) } else { format!("{:.03}", x) }).into(),
        ));
    }

    fn dispatch_scaling_aspect_ratio_y(&self, y: f32) {
        self.catch_error(dispatch_event_with(
            &self.event_bus,
            "back2front:scaling_aspect_ratio_y",
            &(if y.floor() == y { format!("{:.00}", y) } else { format!("{:.03}", y) }).into(),
        ));
    }

    fn dispatch_custom_scaling_stretch_nearest(&self, stretch: bool) {
        self.catch_error(dispatch_event_with(
            &self.event_bus,
            "back2front:custom_scaling_stretch_nearest",
            &(stretch).into(),
        ));
    }

    fn dispatch_change_pixel_speed(&self, speed: f32) {
        let speed = self.format_speed(speed);
        if self.are_extra_messages_enabled() {
            self.dispatch_top_message(&format!("Pixel manipulation speed: {}", speed));
        }
        self.catch_error(dispatch_event_with(&self.event_bus, "back2front:change_pixel_speed", &speed.into()));
    }

    fn dispatch_change_turning_speed(&self, speed: f32) {
        let speed = self.format_speed(speed);
        if self.are_extra_messages_enabled() {
            self.dispatch_top_message(&format!("Turning camera speed: {}", speed));
        }
        self.catch_error(dispatch_event_with(&self.event_bus, "back2front:change_turning_speed", &speed.into()));
    }

    fn dispatch_change_movement_speed(&self, speed: f32) {
        let speed = self.format_speed(speed);
        if self.are_extra_messages_enabled() {
            self.dispatch_top_message(&format!("Translation camera speed: {}", speed));
        }
        self.catch_error(dispatch_event_with(&self.event_bus, "back2front:change_movement_speed", &speed.into()));
    }
    fn dispatch_exiting_session(&self) {
        self.catch_error(dispatch_event(&self.event_bus, "back2front:exiting_session"));
    }
    fn dispatch_toggle_info_panel(&self) {
        self.catch_error(dispatch_event(&self.event_bus, "back2front:toggle_info_panel"));
    }
    fn dispatch_fps(&self, fps: f32) {
        self.catch_error(dispatch_event_with(&self.event_bus, "back2front:fps", &fps.into()));
    }

    fn dispatch_request_fullscreen(&self) {
        self.catch_error(dispatch_event(&self.event_bus, "back2front:request_fullscreen"));
    }

    fn dispatch_request_pointer_lock(&self) {
        self.catch_error(dispatch_event(&self.event_bus, "back2front:request_pointer_lock"));
    }

    fn dispatch_exit_pointer_lock(&self) {
        self.catch_error(dispatch_event(&self.event_bus, "back2front:exit_pointer_lock"));
    }

    // @TODO no other way to handle this by now, because of glow lacking API, find better way later
    fn dispatch_screenshot(&self, width: i32, height: i32, pixels: &mut [u8]) -> AppResult<()> {
        let gl = &self.gl;
        gl.read_pixels_with_opt_u8_array(0, 0, width, height, glow::RGBA, glow::UNSIGNED_BYTE, Some(&mut *pixels))?;
        let js_pixels = unsafe { js_sys::Uint8Array::view(pixels) };
        let object = js_sys::Object::new();
        js_sys::Reflect::set(&object, &"width".into(), &width.into()).expect("Reflection failed on width");
        js_sys::Reflect::set(&object, &"height".into(), &height.into()).expect("Reflection failed on height");
        js_sys::Reflect::set(&object, &"buffer".into(), &js_pixels.into()).expect("Reflection failed on js_pixels");
        self.catch_error(dispatch_event_with(&self.event_bus, "back2front:screenshot", &object));
        Ok(())
    }

    fn dispatch_change_camera_movement_mode(&self, locked_mode: CameraLockMode) {
        self.catch_error(dispatch_event_with(
            &self.event_bus,
            "back2front:change_camera_movement_mode",
            &locked_mode.to_string().into(),
        ));
    }

    fn dispatch_top_message(&self, message: &str) {
        self.catch_error(dispatch_event_with(&self.event_bus, "back2front:top_message", &message.into()));
    }

    fn dispatch_minimum_value(&self, value: &dyn Display) {
        self.dispatch_top_message(&format!("Minimum value is {}", value));
    }

    fn dispatch_maximum_value(&self, value: &dyn Display) {
        self.dispatch_top_message(&format!("Maximum value is {}", value));
    }
}

impl WebEventDispatcher {
    fn format_speed(&self, speed: f32) -> String {
        format!("x{}", (speed * 1000.0).round() / 1000.0)
    }

    pub fn check_error(&self) -> AppResult<()> {
        if let Some(e) = self.error.borrow_mut().take() {
            return Err(e);
        }
        Ok(())
    }

    fn catch_error(&self, result: AppResult<()>) {
        if self.error.borrow().is_some() {
            return;
        }
        if let Err(e) = result {
            *self.error.borrow_mut() = Some(e);
        }
    }
}
