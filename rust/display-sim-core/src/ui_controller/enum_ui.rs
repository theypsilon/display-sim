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
use crate::field_changer::FieldChanger;
use crate::general_types::{IncDec, OptionCursor};

use crate::boolean_button::BooleanButton;
use crate::input_types::TrackedButton;
use crate::simulation_context::SimulationContext;
use crate::simulation_core_state::MainState;
use crate::ui_controller::{EncodedValue, UiController};
use app_util::{AppError, AppResult};
use std::convert::{TryFrom, TryInto};
use std::fmt::Display;

pub trait EnumUi {
    fn event_tag(&self) -> &'static str;
    fn keys_inc(&self) -> &[&'static str];
    fn keys_dec(&self) -> &[&'static str];
    fn dispatch_tag(&self) -> &'static str;
}

#[derive(Clone, Default)]
pub struct EnumHolder<'a, T: Clone + OptionCursor + Display + EnumUi + TryFrom<&'a dyn EncodedValue>>
where
    AppError: From<<T as TryFrom<&'a dyn EncodedValue>>::Error>,
{
    input: IncDec<BooleanButton>,
    event: Option<T>,
    pub value: T,
    _u: std::marker::PhantomData<&'a T>,
}

impl<'a, T: Clone + OptionCursor + Display + EnumUi + TryFrom<&'a dyn EncodedValue>> From<T> for EnumHolder<'a, T>
where
    AppError: From<<T as TryFrom<&'a dyn EncodedValue>>::Error>,
{
    fn from(value: T) -> Self {
        EnumHolder {
            input: Default::default(),
            event: None,
            value,
            _u: Default::default(),
        }
    }
}

impl<'a, T: Clone + OptionCursor + Display + EnumUi + TryFrom<&'a dyn EncodedValue>> UiController for EnumHolder<'a, T>
where
    AppError: From<<T as TryFrom<&'a dyn EncodedValue>>::Error>,
{
    fn event_tag(&self) -> &'static str {
        self.value.event_tag()
    }
    fn keys_inc(&self) -> &[&'static str] {
        self.value.keys_inc()
    }
    fn keys_dec(&self) -> &[&'static str] {
        self.value.keys_dec()
    }
    fn update(&mut self, _: &MainState, ctx: &dyn SimulationContext) -> bool {
        FieldChanger::new(ctx, &mut self.value, self.input.to_just_pressed())
            .set_trigger_handler(|x: &T| dispatch(x, ctx.dispatcher()))
            .process_options()
    }
    fn reset_inputs(&mut self) {
        self.input = Default::default();
        self.event = None;
    }
    fn read_event(&mut self, value: &dyn EncodedValue) -> AppResult<()> {
        self.event = Some(value.try_into()?);
        Ok(())
    }
    fn read_key_inc(&mut self, pressed: bool) {
        self.input.increase.input = pressed;
    }
    fn read_key_dec(&mut self, pressed: bool) {
        self.input.decrease.input = pressed;
    }
    fn dispatch_event(&self, dispatcher: &dyn AppEventDispatcher) {
        dispatch(&self.value, dispatcher)
    }
    fn pre_process_input(&mut self) {
        self.input.track()
    }
    fn post_process_input(&mut self) {
        self.event = None;
    }
}

fn dispatch<T: Clone + OptionCursor + Display + EnumUi>(value: &T, dispatcher: &dyn AppEventDispatcher) {
    dispatcher.dispatch_string_event(value.dispatch_tag(), &(value.to_string()));
}
