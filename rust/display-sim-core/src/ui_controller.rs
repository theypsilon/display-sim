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
use crate::simulation_context::SimulationContext;
use crate::simulation_core_state::MainState;
use app_error::AppResult;

pub trait UiController {
    fn event_tag(&self) -> &'static str;
    fn keys_inc(&self) -> &[&'static str];
    fn keys_dec(&self) -> &[&'static str];
    fn update(&mut self, main: &MainState, ctx: &dyn SimulationContext) -> bool;
    fn apply_event(&mut self);
    fn reset_inputs(&mut self);
    fn read_event(&mut self, encoded: &dyn EncodedValue) -> AppResult<()>;
    fn read_key_inc(&mut self, pressed: bool);
    fn read_key_dec(&mut self, pressed: bool);
    fn dispatch_event(&self, dispatcher: &dyn AppEventDispatcher);
    fn pre_process_input(&mut self);
    fn post_process_input(&mut self);
}

pub trait EncodedValue {
    fn to_f64(&self) -> AppResult<f64>;
    fn to_f32(&self) -> AppResult<f32>;
    fn to_u32(&self) -> AppResult<u32>;
    fn to_i32(&self) -> AppResult<i32>;
    fn to_usize(&self) -> AppResult<usize>;
    fn to_string(&self) -> AppResult<String>;
}

pub mod backlight_percent;
pub mod blur_passes;
pub mod brightness_color;
pub mod color_channels;
pub mod color_gamma;
pub mod color_noise;
pub mod cur_pixel_horizontal_gap;
pub mod cur_pixel_spread;
pub mod cur_pixel_vertical_gap;
mod enum_ui;
pub mod extra_bright;
pub mod extra_contrast;
pub mod filter_preset;
pub mod horizontal_lpp;
pub mod internal_resolution;
pub mod light_color;
pub mod pixel_geometry_kind;
pub mod pixel_shadow_height;
pub mod pixel_shadow_shape_kind;
pub mod rgb_calibration;
pub mod screen_curvature_kind;
pub mod texture_interpolation;
pub mod vertical_lpp;
