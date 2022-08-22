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
use crate::general_types::{IncDec, OptionCursor, Size2D};

use crate::boolean_button::BooleanButton;
use crate::simulation_context::SimulationContext;
use crate::simulation_core_state::MainState;
use crate::ui_controller::{EncodedValue, UiController};
use app_util::AppResult;
use std::fmt::{Display, Error, Formatter};

#[derive(Clone)]
pub struct InternalResolution {
    max_texture_size: i32,
    viewport: Size2D<i32>,
    minimum_reached: bool,
    maximium_reached: bool,
    pub changed: bool,
    input: IncDec<BooleanButton>,
}

impl Default for InternalResolution {
    fn default() -> Self {
        InternalResolution {
            max_texture_size: std::i32::MAX,
            viewport: Size2D { width: 3840, height: 2160 },
            input: Default::default(),
            minimum_reached: false,
            maximium_reached: false,
            changed: false,
        }
    }
}

impl Display for InternalResolution {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let height = self.height();
        if height <= 1080 {
            write!(f, "{}p", height)?;
        } else {
            write!(f, "{}K", height / 540)?;
        }
        Ok(())
    }
}

impl InternalResolution {
    pub(crate) fn set_max_texture_size(&mut self, value: i32) {
        self.max_texture_size = value;
    }
    pub(crate) fn set_resolution(&mut self, resolution: i32) {
        self.viewport.height = resolution;
        self.viewport.width = match resolution {
            0 => unreachable!(),
            102 => 160,
            144 => 160,
            152 => 160,
            224 => 256,
            240 => 320,
            243 => 320,
            480 => 640,
            486 => 640,
            _ => resolution * 16 / 9,
        };
        if self.width() > self.max_texture_size || self.height() > self.max_texture_size {
            self.previous_option();
            self.maximium_reached = true;
        }
    }
    pub fn width(&self) -> i32 {
        self.viewport.width as i32
    }
    pub fn height(&self) -> i32 {
        self.viewport.height as i32
    }
}

impl OptionCursor for InternalResolution {
    fn next_option(&mut self) {
        self.minimum_reached = false;
        let new_height = match self.height() {
            std::i32::MIN..=0 => 1080,
            720 => 1080,
            486 => 720,
            480 => 486,
            243 => 480,
            240 => 243,
            224 => 240,
            160 => 224,
            152 => 160,
            144 => 152,
            102 => 144,
            51..=101 => 102,
            height => height * 2,
        };
        self.set_resolution(new_height);
    }
    fn previous_option(&mut self) {
        self.maximium_reached = false;
        let new_height = match self.height() {
            std::i32::MIN..=-1 => 1080,
            1080 => 720,
            720 => 486,
            486 => 480,
            480 => 243,
            243 => 240,
            240 => 224,
            224 => 160,
            160 => 152,
            152 => 144,
            144 => 102,
            height @ 0..=4 => {
                self.minimum_reached = true;
                height
            }
            height => height / 2,
        };
        self.set_resolution(new_height);
    }
    fn has_reached_maximum_limit(&self) -> bool {
        self.maximium_reached
    }
    fn has_reached_minimum_limit(&self) -> bool {
        self.minimum_reached
    }
}

impl UiController for InternalResolution {
    fn event_tag(&self) -> &'static str {
        ""
    }
    fn keys_inc(&self) -> &[&'static str] {
        &["y", "internal-resolution-inc"]
    }
    fn keys_dec(&self) -> &[&'static str] {
        &["shift+y", "internal-resolution-dec"]
    }
    fn update(&mut self, _: &MainState, ctx: &dyn SimulationContext) -> bool {
        let inputs = self.input.to_just_pressed();
        self.changed = FieldChanger::new(ctx, self as &mut InternalResolution, inputs)
            .set_trigger_handler(|x: &InternalResolution| dispatch(x, ctx.dispatcher()))
            .process_options();
        self.changed
    }
    fn apply_event(&mut self) {}
    fn reset_inputs(&mut self) {
        self.input = Default::default();
    }
    fn read_event(&mut self, _: &dyn EncodedValue) -> AppResult<()> {
        Ok(())
    }
    fn read_key_inc(&mut self, pressed: bool) {
        self.input.increase.input = pressed;
    }
    fn read_key_dec(&mut self, pressed: bool) {
        self.input.decrease.input = pressed;
    }
    fn dispatch_event(&self, dispatcher: &dyn AppEventDispatcher) {
        dispatch(self, dispatcher)
    }
    fn pre_process_input(&mut self) {
        self.input.get_buttons().iter_mut().for_each(|button| button.track_input());
    }
    fn post_process_input(&mut self) {}
}

fn dispatch(value: &InternalResolution, dispatcher: &dyn AppEventDispatcher) {
    dispatcher.dispatch_string_event("back2front:internal_resolution", &value.to_string());
}
