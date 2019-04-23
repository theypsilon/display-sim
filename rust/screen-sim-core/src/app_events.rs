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
use crate::internal_resolution::InternalResolution;
use crate::pixels_shadow::ShadowShape;
use crate::simulation_core_state::{ColorChannels, PixelsGeometryKind, ScreenCurvatureKind, ScreenLayeringKind, TextureInterpolation};
use std::fmt::Display;

pub trait AppEventDispatcher: Default {
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
    fn dispatch_change_lines_per_pixel(&self, res: usize);
    fn dispatch_color_representation(&self, res: ColorChannels);
    fn dispatch_pixel_geometry(&self, res: PixelsGeometryKind);
    fn dispatch_pixel_shadow_shape(&self, res: ShadowShape);
    fn dispatch_pixel_shadow_height(&self, res: f32);
    fn dispatch_screen_layering_type(&self, res: ScreenLayeringKind);
    fn dispatch_screen_curvature(&self, res: ScreenCurvatureKind);
    fn dispatch_internal_resolution(&self, res: &InternalResolution);
    fn dispatch_texture_interpolation(&self, res: TextureInterpolation);
    fn dispatch_change_pixel_speed(&self, speed: f32);
    fn dispatch_change_turning_speed(&self, speed: f32);
    fn dispatch_change_movement_speed(&self, speed: f32);
    fn dispatch_exiting_session(&self);
    fn dispatch_toggle_info_panel(&self);
    fn dispatch_fps(&self, fps: f32);
    fn dispatch_request_pointer_lock(&self);
    fn dispatch_exit_pointer_lock(&self);
    fn dispatch_screenshot(&self, pixels: &[u8], multiplier: f64);
    fn dispatch_change_camera_movement_mode(&self, locked_mode: bool);
    fn dispatch_top_message(&self, message: &str);
    fn dispatch_minimum_value<T: Display>(&self, value: &T);
    fn dispatch_maximum_value<T: Display>(&self, value: &T);
}
