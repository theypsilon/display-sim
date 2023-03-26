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

use crate::app_events::AppEventDispatcher;
use crate::simulation_context::SimulationContext;
use crate::simulation_core_state::MainState;
use crate::ui_controller::{EncodedValue, UiController};
use app_util::AppResult;

macro_rules! rgb_impl {
    ($ty:ident, $event_tag:expr, $dispatch_tag:expr) => {
        #[derive(Default, Copy, Clone)]
        pub struct $ty {
            event: Option<f32>,
            pub value: f32,
        }

        impl From<f32> for $ty {
            fn from(value: f32) -> Self {
                $ty { event: None, value }
            }
        }

        impl From<$ty> for f32 {
            fn from(component: $ty) -> Self {
                component.value
            }
        }

        impl UiController for $ty {
            fn event_tag(&self) -> &'static str {
                $event_tag
            }
            fn keys_inc(&self) -> &[&'static str] {
                &[]
            }
            fn keys_dec(&self) -> &[&'static str] {
                &[]
            }
            fn update(&mut self, _: &MainState, _: &dyn SimulationContext) -> bool {
                false
            }
            fn reset_inputs(&mut self) {
                self.event = None;
            }
            fn read_event(&mut self, encoded: Box<dyn EncodedValue>) -> AppResult<()> {
                self.event = Some(encoded.to_f32()?);
                Ok(())
            }
            fn read_key_inc(&mut self, _: bool) {}
            fn read_key_dec(&mut self, _: bool) {}
            fn dispatch_event(&self, dispatcher: &dyn AppEventDispatcher) {
                dispatcher.dispatch_string_event(
                    $dispatch_tag,
                    &if self.value.floor() == self.value {
                        format!("{:.00}", self.value)
                    } else {
                        format!("{:.03}", self.value)
                    },
                );
            }
            fn pre_process_input(&mut self) {}
            fn post_process_input(&mut self) {
                self.event = None;
            }
        }
    };
}

rgb_impl! {RgbRedR, "front2back:rgb-red-r", "back2front:rgb_red_r"}
rgb_impl! {RgbRedG, "front2back:rgb-red-g", "back2front:rgb_red_g"}
rgb_impl! {RgbRedB, "front2back:rgb-red-b", "back2front:rgb_red_b"}

rgb_impl! {RgbGreenR, "front2back:rgb-green-r", "back2front:rgb_green_r"}
rgb_impl! {RgbGreenG, "front2back:rgb-green-g", "back2front:rgb_green_g"}
rgb_impl! {RgbGreenB, "front2back:rgb-green-b", "back2front:rgb_green_b"}

rgb_impl! {RgbBlueR, "front2back:rgb-blue-r", "back2front:rgb_blue_r"}
rgb_impl! {RgbBlueG, "front2back:rgb-blue-g", "back2front:rgb_blue_g"}
rgb_impl! {RgbBlueB, "front2back:rgb-blue-b", "back2front:rgb_blue_b"}
