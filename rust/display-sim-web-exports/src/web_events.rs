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
use crate::dispatch_event::{dispatch_event, dispatch_event_with};
use core::app_events::AppEventDispatcher;
use core::internal_resolution::InternalResolution;
use core::pixels_shadow::ShadowShape;
use core::simulation_core_state::{ColorChannels, PixelsGeometryKind, ScreenCurvatureKind, ScreenLayeringKind, TextureInterpolation};
use js_sys::{Array, Float32Array};
use std::cell::RefCell;
use std::fmt::Display;
use web_error::{WebError, WebResult};

pub struct WebEventDispatcher {
    error: RefCell<Option<WebError>>,
    extra_messages_enabled: RefCell<bool>,
}

impl Default for WebEventDispatcher {
    fn default() -> Self {
        WebEventDispatcher {
            error: Default::default(),
            extra_messages_enabled: RefCell::new(true),
        }
    }
}

impl WebEventDispatcher {
    fn are_extra_messages_enabled(&self) -> bool {
        *self.extra_messages_enabled.borrow()
    }
}

impl AppEventDispatcher for WebEventDispatcher {
    fn enable_extra_messages(&self, extra_messages: bool) {
        *self.extra_messages_enabled.borrow_mut() = extra_messages;
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
        self.catch_error(dispatch_event_with("app-event.camera_update", &values_array.into()));
    }

    fn dispatch_change_pixel_horizontal_gap(&self, size: f32) {
        self.catch_error(dispatch_event_with("app-event.change_pixel_horizontal_gap", &format!("{:.03}", size).into()));
    }

    fn dispatch_change_pixel_vertical_gap(&self, size: f32) {
        self.catch_error(dispatch_event_with("app-event.change_pixel_vertical_gap", &format!("{:.03}", size).into()));
    }

    fn dispatch_change_pixel_width(&self, size: f32) {
        self.catch_error(dispatch_event_with("app-event.change_pixel_width", &format!("{:.03}", size).into()));
    }

    fn dispatch_change_pixel_spread(&self, size: f32) {
        self.catch_error(dispatch_event_with("app-event.change_pixel_spread", &format!("{:.03}", size).into()));
    }

    fn dispatch_change_pixel_brightness(&self, extra_bright: f32) {
        self.catch_error(dispatch_event_with(
            "app-event.change_pixel_brightness",
            &format!("{:.02}", extra_bright).into(),
        ));
    }

    fn dispatch_change_pixel_contrast(&self, extra_contrast: f32) {
        self.catch_error(dispatch_event_with(
            "app-event.change_pixel_contrast",
            &format!("{:.02}", extra_contrast).into(),
        ));
    }

    fn dispatch_change_light_color(&self, light_color: i32) {
        self.dispatch_change_color("app-event.change_light_color", light_color);
    }

    fn dispatch_change_brightness_color(&self, brightness_color: i32) {
        self.dispatch_change_color("app-event.change_brightness_color", brightness_color);
    }

    fn dispatch_change_camera_zoom(&self, zoom: f32) {
        self.catch_error(dispatch_event_with("app-event.change_camera_zoom", &format!("{:.02}", zoom).into()));
    }

    fn dispatch_change_blur_level(&self, blur_passes: usize) {
        if self.are_extra_messages_enabled() {
            self.dispatch_top_message(&format!("Blur level: {}", blur_passes));
        }
        self.catch_error(dispatch_event_with("app-event.change_blur_level", &(blur_passes as i32).into()));
    }

    fn dispatch_change_lines_per_pixel(&self, lpp: usize) {
        if self.are_extra_messages_enabled() {
            self.dispatch_top_message(&format!("Lines per pixel: {}", lpp));
        }
        self.catch_error(dispatch_event_with("app-event.change_lines_per_pixel", &(lpp as i32).into()));
    }

    fn dispatch_color_representation(&self, color_channels: ColorChannels) {
        if self.are_extra_messages_enabled() {
            self.dispatch_top_message(&format!("Pixel color representation: {}.", color_channels));
        }
        self.catch_error(dispatch_event_with("app-event.color_representation", &(color_channels.to_string()).into()));
    }

    fn dispatch_pixel_geometry(&self, pixels_geometry_kind: PixelsGeometryKind) {
        if self.are_extra_messages_enabled() {
            self.dispatch_top_message(&format!("Pixel geometry: {}.", pixels_geometry_kind));
        }
        self.catch_error(dispatch_event_with("app-event.pixel_geometry", &(pixels_geometry_kind.to_string()).into()));
    }

    fn dispatch_pixel_shadow_shape(&self, pixel_shadow_shape_kind: ShadowShape) {
        if self.are_extra_messages_enabled() {
            self.dispatch_top_message(&format!("Showing next pixel shadow: {}.", pixel_shadow_shape_kind));
        }
        self.catch_error(dispatch_event_with(
            "app-event.pixel_shadow_shape",
            &(pixel_shadow_shape_kind.to_string()).into(),
        ));
    }

    fn dispatch_pixel_shadow_height(&self, pixel_shadow_height: f32) {
        self.catch_error(dispatch_event_with(
            "app-event.pixel_shadow_height",
            &format!("{:.02}", pixel_shadow_height).into(),
        ));
    }

    fn dispatch_screen_layering_type(&self, layering_kind: ScreenLayeringKind) {
        if self.are_extra_messages_enabled() {
            self.dispatch_top_message(&format!("Layering kind: {}.", layering_kind));
        }
        self.catch_error(dispatch_event_with("app-event.screen_layering_type", &(layering_kind.to_string()).into()));
    }

    fn dispatch_screen_curvature(&self, screen_curvature_kind: ScreenCurvatureKind) {
        if self.are_extra_messages_enabled() {
            self.dispatch_top_message(&format!("Screen curvature: {}.", screen_curvature_kind));
        }
        self.catch_error(dispatch_event_with("app-event.screen_curvature", &(screen_curvature_kind.to_string()).into()));
    }

    fn dispatch_internal_resolution(&self, internal_resolution: &InternalResolution) {
        self.catch_error(dispatch_event_with("app-event.internal_resolution", &(internal_resolution.to_string()).into()));
    }

    fn dispatch_texture_interpolation(&self, texture_interpolation: TextureInterpolation) {
        self.catch_error(dispatch_event_with(
            "app-event.texture_interpolation",
            &(texture_interpolation.to_string()).into(),
        ));
    }

    fn dispatch_change_pixel_speed(&self, speed: f32) {
        let speed = self.format_speed(speed);
        if self.are_extra_messages_enabled() {
            self.dispatch_top_message(&format!("Pixel manipulation speed: {}", speed));
        }
        self.catch_error(dispatch_event_with("app-event.change_pixel_speed", &speed.into()));
    }

    fn dispatch_change_turning_speed(&self, speed: f32) {
        let speed = self.format_speed(speed);
        if self.are_extra_messages_enabled() {
            self.dispatch_top_message(&format!("Turning camera speed: {}", speed));
        }
        self.catch_error(dispatch_event_with("app-event.change_turning_speed", &speed.into()));
    }

    fn dispatch_change_movement_speed(&self, speed: f32) {
        let speed = self.format_speed(speed);
        if self.are_extra_messages_enabled() {
            self.dispatch_top_message(&format!("Translation camera speed: {}", speed));
        }
        self.catch_error(dispatch_event_with("app-event.change_movement_speed", &speed.into()));
    }

    fn dispatch_exiting_session(&self) {
        self.catch_error(dispatch_event("app-event.exiting_session"));
    }
    fn dispatch_toggle_info_panel(&self) {
        self.catch_error(dispatch_event("app-event.toggle_info_panel"));
    }
    fn dispatch_fps(&self, fps: f32) {
        self.catch_error(dispatch_event_with("app-event.fps", &fps.into()));
    }

    fn dispatch_request_pointer_lock(&self) {
        self.catch_error(dispatch_event("app-event.request_pointer_lock"));
    }

    fn dispatch_exit_pointer_lock(&self) {
        self.catch_error(dispatch_event("app-event.exit_pointer_lock"));
    }

    fn dispatch_screenshot(&self, pixels: &[u8], multiplier: f64) {
        let js_pixels = unsafe { js_sys::Uint8Array::view(pixels) };
        let array = Array::new();
        array.push(&js_pixels);
        array.push(&multiplier.into());
        self.catch_error(dispatch_event_with("app-event.screenshot", &array));
    }

    fn dispatch_change_camera_movement_mode(&self, locked_mode: bool) {
        self.catch_error(dispatch_event_with(
            "app-event.change_camera_movement_mode",
            &(if locked_mode { "Lock on Display" } else { "Free Flight" }).into(),
        ));
    }

    fn dispatch_top_message(&self, message: &str) {
        self.catch_error(dispatch_event_with("app-event.top_message", &message.into()));
    }

    fn dispatch_minimum_value<T: Display>(&self, value: &T) {
        self.dispatch_top_message(&format!("Minimum value is {}", value));
    }

    fn dispatch_maximum_value<T: Display>(&self, value: &T) {
        self.dispatch_top_message(&format!("Maximum value is {}", value));
    }
}

impl WebEventDispatcher {
    fn format_speed(&self, speed: f32) -> String {
        format!("x{}", (speed * 1000.0).round() / 1000.0)
    }
    fn dispatch_change_color(&self, id: &str, color: i32) {
        self.catch_error(dispatch_event_with(id, &format!("#{:X}", color).into()));
    }

    pub fn check_error(&self) -> WebResult<()> {
        if let Some(e) = self.error.borrow_mut().take() {
            return Err(e);
        }
        Ok(())
    }

    fn catch_error(&self, result: WebResult<()>) {
        if self.error.borrow().is_some() {
            return;
        }
        if let Err(e) = result {
            *self.error.borrow_mut() = Some(e);
        }
    }
}
