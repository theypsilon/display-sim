/* Copyright (c) 2019-2024 José manuel Barroso Galindo <theypsilon@gmail.com>
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
use crate::field_changer::FieldChanger;
use crate::general_types::IncDec;
use crate::simulation_context::SimulationContext;
use crate::simulation_core_state::MainState;
use crate::ui_controller::{EncodedValue, UiController};
use app_util::AppResult;

#[derive(Default, Copy, Clone)]
pub struct BacklightPercent {
    input: IncDec<bool>,
    event: Option<f32>,
    pub value: f32,
}

impl From<f32> for BacklightPercent {
    fn from(value: f32) -> Self {
        BacklightPercent {
            input: Default::default(),
            event: None,
            value,
        }
    }
}

impl UiController for BacklightPercent {
    fn event_tag(&self) -> &'static str {
        "front2back:backlight-percent"
    }
    fn keys_inc(&self) -> &[&'static str] {
        &["backlight-percent-inc", ","]
    }
    fn keys_dec(&self) -> &[&'static str] {
        &["backlight-percent-dec", "."]
    }
    fn update(&mut self, main: &MainState, ctx: &dyn SimulationContext) -> bool {
        FieldChanger::new(ctx, &mut self.value, self.input)
            .set_progression(1.25 * main.dt * main.filter_speed)
            .set_event_value(self.event)
            .set_min(0.0)
            .set_max(20.0)
            .set_trigger_handler(|x| dispatch(x, ctx.dispatcher()))
            .process_with_sums()
    }
    fn reset_inputs(&mut self) {
        self.event = None;
        self.input.increase = false;
        self.input.decrease = false;
    }
    fn read_event(&mut self, encoded: Box<dyn EncodedValue>) -> AppResult<()> {
        self.event = Some(encoded.to_f32()?);
        Ok(())
    }
    fn read_key_inc(&mut self, pressed: bool) {
        self.input.increase = pressed;
    }
    fn read_key_dec(&mut self, pressed: bool) {
        self.input.decrease = pressed;
    }
    fn dispatch_event(&self, dispatcher: &dyn AppEventDispatcher) {
        dispatch(self.value, dispatcher)
    }
    fn pre_process_input(&mut self) {}
    fn post_process_input(&mut self) {
        self.event = None;
    }
}

fn dispatch(value: f32, dispatcher: &dyn AppEventDispatcher) {
    dispatcher.dispatch_string_event(
        "back2front:backlight_percent",
        &if value.floor() == value {
            format!("{:.00}", value)
        } else {
            format!("{:.03}", value)
        },
    );
}
