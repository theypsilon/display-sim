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

use crate::camera::CameraLockMode;
use crate::simulation_core_state::ScalingMethod;
use app_util::AppResult;
use std::fmt::Display;

pub trait AppEventDispatcher {
    fn enable_extra_messages(&self, extra_messages_enabled: bool);
    fn are_extra_messages_enabled(&self) -> bool;
    fn dispatch_log(&self, msg: String);
    fn dispatch_string_event(&self, event_id: &'static str, message: &str);
    fn dispatch_camera_update(&self, position: &glm::Vec3, direction: &glm::Vec3, axis_up: &glm::Vec3);
    fn dispatch_change_pixel_width(&self, size: f32);
    fn dispatch_change_camera_zoom(&self, zoom: f32);
    fn dispatch_change_pixel_speed(&self, speed: f32);
    fn dispatch_change_turning_speed(&self, speed: f32);
    fn dispatch_change_movement_speed(&self, speed: f32);
    fn dispatch_scaling_method(&self, method: ScalingMethod);
    fn dispatch_scaling_resolution_width(&self, width: u32);
    fn dispatch_scaling_resolution_height(&self, height: u32);
    fn dispatch_scaling_aspect_ratio_x(&self, x: f32);
    fn dispatch_scaling_aspect_ratio_y(&self, y: f32);
    fn dispatch_custom_scaling_stretch_nearest(&self, stretch: bool);
    fn dispatch_exiting_session(&self);
    fn dispatch_toggle_info_panel(&self);
    fn dispatch_fps(&self, fps: f32);
    fn dispatch_request_fullscreen(&self);
    fn dispatch_request_pointer_lock(&self);
    fn dispatch_exit_pointer_lock(&self);
    fn dispatch_screenshot(&self, width: i32, height: i32, pixels: &mut [u8]) -> AppResult<()>;
    fn dispatch_change_camera_movement_mode(&self, locked_mode: CameraLockMode);
    fn dispatch_top_message(&self, message: &str);
    fn dispatch_minimum_value(&self, value: &dyn Display);
    fn dispatch_maximum_value(&self, value: &dyn Display);
}

#[derive(Default)]
pub struct FakeEventDispatcher {}

impl AppEventDispatcher for FakeEventDispatcher {
    fn enable_extra_messages(&self, _: bool) {}
    fn are_extra_messages_enabled(&self) -> bool {
        true
    }
    fn dispatch_log(&self, _: String) {}
    fn dispatch_string_event(&self, _: &'static str, _: &str) {}
    fn dispatch_camera_update(&self, _: &glm::Vec3, _: &glm::Vec3, _: &glm::Vec3) {}
    fn dispatch_change_pixel_width(&self, _: f32) {}
    fn dispatch_change_camera_zoom(&self, _: f32) {}
    fn dispatch_change_pixel_speed(&self, _: f32) {}
    fn dispatch_change_turning_speed(&self, _: f32) {}
    fn dispatch_change_movement_speed(&self, _: f32) {}
    fn dispatch_scaling_method(&self, _: ScalingMethod) {}
    fn dispatch_scaling_resolution_width(&self, _: u32) {}
    fn dispatch_scaling_resolution_height(&self, _: u32) {}
    fn dispatch_scaling_aspect_ratio_x(&self, _: f32) {}
    fn dispatch_scaling_aspect_ratio_y(&self, _: f32) {}
    fn dispatch_custom_scaling_stretch_nearest(&self, _: bool) {}
    fn dispatch_exiting_session(&self) {}
    fn dispatch_toggle_info_panel(&self) {}
    fn dispatch_fps(&self, fps: f32) {
        println!("frames in 20 seconds: {}", fps);
    }
    fn dispatch_screenshot(&self, _: i32, _: i32, _: &mut [u8]) -> AppResult<()> {
        Ok(())
    }
    fn dispatch_request_fullscreen(&self) {}
    fn dispatch_request_pointer_lock(&self) {}
    fn dispatch_exit_pointer_lock(&self) {}
    fn dispatch_change_camera_movement_mode(&self, _: CameraLockMode) {}
    fn dispatch_top_message(&self, _: &str) {}
    fn dispatch_minimum_value(&self, _: &dyn Display) {}
    fn dispatch_maximum_value(&self, _: &dyn Display) {}
}
