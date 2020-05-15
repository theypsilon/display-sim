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
use crate::field_changer::FieldChanger;
use crate::general_types::IncDec;
use crate::simulation_context::SimulationContext;
use crate::simulation_core_state::MainState;
use crate::ui_controller::{EncodedValue, UiController};
use app_error::AppResult;

#[derive(Default, Copy, Clone)]
pub struct BrightnessColor {
    input: IncDec<bool>,
    event: Option<i32>,
    pub value: i32,
}

impl From<i32> for BrightnessColor {
    fn from(value: i32) -> Self {
        BrightnessColor {
            input: Default::default(),
            event: None,
            value,
        }
    }
}

impl UiController for BrightnessColor {
    fn event_tag(&self) -> &'static str {
        "front2back:brightness-color"
    }
    fn keys_inc(&self) -> &[&'static str] {
        &["j", "blur-level-inc"]
    }
    fn keys_dec(&self) -> &[&'static str] {
        &["shift+j", "blur-level-dec"]
    }
    fn update(&mut self, _: &MainState, ctx: &dyn SimulationContext) -> bool {
        FieldChanger::new(ctx, &mut self.value, self.input)
            .set_progression(1)
            .set_event_value(self.event)
            .set_min(0)
            .set_max(100)
            .set_trigger_handler(|x| dispatch(x, ctx.dispatcher()))
            .process_with_sums()
    }
    fn apply_event(&mut self) {
        if let Some(v) = self.event {
            self.value = v;
            // self.ctx.dispatcher().dispatch_top_message("Brightness Color changed.");
        }
    }
    fn reset_inputs(&mut self) {
        self.event = None;
        self.input.increase = false;
        self.input.decrease = false;
    }
    fn read_event(&mut self, encoded: &dyn EncodedValue) -> AppResult<()> {
        self.event = Some(encoded.to_i32()?);
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

fn dispatch(value: i32, dispatcher: &dyn AppEventDispatcher) {
    dispatcher.dispatch_string_event("back2front:change_brightness_color", &format!("#{:X}", value));
}
