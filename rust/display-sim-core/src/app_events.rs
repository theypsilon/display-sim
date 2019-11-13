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

use crate::camera::CameraLockMode;
use crate::internal_resolution::InternalResolution;
use crate::pixels_shadow::ShadowShape;
use crate::simulation_core_state::{ColorChannels, PixelsGeometryKind, ScreenCurvatureKind, TextureInterpolation};
use std::fmt::Display;

pub trait AppEventDispatcher {
    fn enable_extra_messages(&self, extra_messages_enabled: bool);
    fn dispatch_camera_update(&self, position: &glm::Vec3, direction: &glm::Vec3, axis_up: &glm::Vec3);
    fn dispatch_change_pixel_horizontal_gap(&self, size: f32);
    fn dispatch_change_pixel_vertical_gap(&self, size: f32);
    fn dispatch_change_pixel_width(&self, size: f32);
    fn dispatch_change_pixel_spread(&self, size: f32);
    fn dispatch_change_pixel_brightness(&self, res: f32);
    fn dispatch_change_pixel_contrast(&self, res: f32);
    fn dispatch_change_light_color(&self, res: i32);
    fn dispatch_change_brightness_color(&self, res: i32);
    fn dispatch_change_camera_zoom(&self, zoom: f32);
    fn dispatch_change_blur_level(&self, res: usize);
    fn dispatch_change_vertical_lpp(&self, res: usize);
    fn dispatch_change_horizontal_lpp(&self, res: usize);
    fn dispatch_color_representation(&self, res: ColorChannels);
    fn dispatch_pixel_geometry(&self, res: PixelsGeometryKind);
    fn dispatch_pixel_shadow_shape(&self, res: ShadowShape);
    fn dispatch_pixel_shadow_height(&self, res: f32);
    fn dispatch_backlight_presence(&self, res: f32);
    fn dispatch_screen_curvature(&self, res: ScreenCurvatureKind);
    fn dispatch_internal_resolution(&self, res: &InternalResolution);
    fn dispatch_texture_interpolation(&self, res: TextureInterpolation);
    fn dispatch_change_pixel_speed(&self, speed: f32);
    fn dispatch_change_turning_speed(&self, speed: f32);
    fn dispatch_change_movement_speed(&self, speed: f32);
    fn dispatch_change_preset_selected(&self, name: &str);
    fn dispatch_exiting_session(&self);
    fn dispatch_toggle_info_panel(&self);
    fn dispatch_fps(&self, fps: f32);
    fn dispatch_new_frame(&self);
    fn dispatch_request_pointer_lock(&self);
    fn dispatch_exit_pointer_lock(&self);
    fn fire_screenshot(&self, width: i32, height: i32, pixels: &mut [u8], multiplier: f64);
    fn dispatch_screenshot(&self, pixels: &[u8], multiplier: f64);
    fn dispatch_change_camera_movement_mode(&self, locked_mode: CameraLockMode);
    fn dispatch_top_message(&self, message: &str);
    fn dispatch_minimum_value(&self, value: &dyn Display);
    fn dispatch_maximum_value(&self, value: &dyn Display);
}

#[derive(Default)]
pub struct FakeEventDispatcher {}

impl AppEventDispatcher for FakeEventDispatcher {
    fn enable_extra_messages(&self, _: bool) {}
    fn dispatch_camera_update(&self, _: &glm::Vec3, _: &glm::Vec3, _: &glm::Vec3) {}
    fn dispatch_change_pixel_horizontal_gap(&self, _: f32) {}
    fn dispatch_change_pixel_vertical_gap(&self, _: f32) {}
    fn dispatch_change_pixel_width(&self, _: f32) {}
    fn dispatch_change_pixel_spread(&self, _: f32) {}
    fn dispatch_change_pixel_brightness(&self, _: f32) {}
    fn dispatch_change_pixel_contrast(&self, _: f32) {}
    fn dispatch_change_light_color(&self, _: i32) {}
    fn dispatch_change_brightness_color(&self, _: i32) {}
    fn dispatch_change_camera_zoom(&self, _: f32) {}
    fn dispatch_change_blur_level(&self, _: usize) {}
    fn dispatch_change_vertical_lpp(&self, _: usize) {}
    fn dispatch_change_horizontal_lpp(&self, _: usize) {}
    fn dispatch_color_representation(&self, _: ColorChannels) {}
    fn dispatch_pixel_geometry(&self, _: PixelsGeometryKind) {}
    fn dispatch_pixel_shadow_shape(&self, _: ShadowShape) {}
    fn dispatch_pixel_shadow_height(&self, _: f32) {}
    fn dispatch_backlight_presence(&self, _: f32) {}
    fn dispatch_screen_curvature(&self, _: ScreenCurvatureKind) {}
    fn dispatch_internal_resolution(&self, _: &InternalResolution) {}
    fn dispatch_texture_interpolation(&self, _: TextureInterpolation) {}
    fn dispatch_change_pixel_speed(&self, _: f32) {}
    fn dispatch_change_turning_speed(&self, _: f32) {}
    fn dispatch_change_movement_speed(&self, _: f32) {}
    fn dispatch_change_preset_selected(&self, _: &str) {}
    fn dispatch_exiting_session(&self) {}
    fn dispatch_toggle_info_panel(&self) {}
    fn dispatch_fps(&self, fps: f32) {
        println!("frames in 20 seconds: {}", fps);
    }
    fn dispatch_new_frame(&self) {}
    fn fire_screenshot(&self, _: i32, _: i32, _: &mut [u8], _: f64) {}
    fn dispatch_request_pointer_lock(&self) {}
    fn dispatch_exit_pointer_lock(&self) {}
    fn dispatch_screenshot(&self, _: &[u8], _: f64) {}
    fn dispatch_change_camera_movement_mode(&self, _: CameraLockMode) {}
    fn dispatch_top_message(&self, _: &str) {}
    fn dispatch_minimum_value(&self, _: &dyn Display) {}
    fn dispatch_maximum_value(&self, _: &dyn Display) {}
}
