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
use crate::boolean_button::BooleanButton;
use crate::field_changer::FieldChanger;
use crate::general_types::IncDec;
use crate::input_types::TrackedButton;
use crate::simulation_context::SimulationContext;
use crate::simulation_core_state::MainState;
use crate::ui_controller::{EncodedValue, UiController};
use app_util::AppResult;

#[derive(Default, Clone)]
pub struct VerticalLpp {
    input: IncDec<BooleanButton>,
    event: Option<usize>,
    pub value: usize,
}

impl From<usize> for VerticalLpp {
    fn from(value: usize) -> Self {
        VerticalLpp {
            input: Default::default(),
            event: None,
            value,
        }
    }
}

impl UiController for VerticalLpp {
    fn event_tag(&self) -> &'static str {
        "front2back:vertical-lpp"
    }
    fn keys_inc(&self) -> &[&'static str] {
        &["k", "vertical-lpp-inc"]
    }
    fn keys_dec(&self) -> &[&'static str] {
        &["shift+k", "vertical-lpp-dec"]
    }
    fn update(&mut self, _: &MainState, ctx: &dyn SimulationContext) -> bool {
        FieldChanger::new(ctx, &mut self.value, self.input.to_just_pressed())
            .set_progression(1)
            .set_min(1)
            .set_max(20)
            .set_event_value(self.event)
            .set_trigger_handler(|x| dispatch(x, ctx.dispatcher()))
            .process_with_sums()
    }
    fn reset_inputs(&mut self) {
        self.event = None;
        self.input.increase.input = false;
        self.input.decrease.input = false;
    }
    fn read_event(&mut self, encoded: Box<dyn EncodedValue>) -> AppResult<()> {
        self.event = Some(encoded.to_usize()?);
        Ok(())
    }
    fn read_key_inc(&mut self, pressed: bool) {
        self.input.increase.input = pressed;
    }
    fn read_key_dec(&mut self, pressed: bool) {
        self.input.decrease.input = pressed;
    }
    fn dispatch_event(&self, dispatcher: &dyn AppEventDispatcher) {
        dispatch(self.value, dispatcher)
    }
    fn pre_process_input(&mut self) {
        self.input.track()
    }
    fn post_process_input(&mut self) {
        self.event = None;
    }
}

fn dispatch(value: usize, dispatcher: &dyn AppEventDispatcher) {
    if dispatcher.are_extra_messages_enabled() {
        dispatcher.dispatch_top_message(&format!("V. lines per pixel: {}", value));
    }
    dispatcher.dispatch_string_event("back2front:change_vertical_lpp", &(value as i32).to_string());
}
