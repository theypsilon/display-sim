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
use app_error::AppResult;

pub trait UiController {
    fn event_tag(&self) -> &'static str;
    fn keys_inc(&self) -> &[&'static str];
    fn keys_dec(&self) -> &[&'static str];
    fn update(&mut self, speed: f32, ctx: &dyn SimulationContext) -> bool;
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
}

pub mod color_noise;
