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

#[derive(Default)]
pub struct BooleanButton {
    pub input: bool,
    button: BooleanButtonBasic,
}

impl BooleanButton {
    pub fn track_input(&mut self) {
        self.button.track(self.input);
    }
    pub fn is_activated(&self) -> bool {
        self.button.is_activated()
    }
    pub fn is_just_pressed(&self) -> bool {
        self.button.is_just_pressed()
    }
    pub fn is_just_released(&self) -> bool {
        self.button.is_just_released()
    }
}

#[derive(Default)]
struct BooleanButtonBasic {
    activated: bool,
    just_pressed: bool,
    just_released: bool,
}

impl BooleanButtonBasic {
    pub fn track(&mut self, pushed: bool) {
        self.just_pressed = false;
        self.just_released = false;
        if !pushed && self.activated {
            self.just_released = true;
        } else if pushed && !self.activated {
            self.just_pressed = true;
        }
        self.activated = pushed;
    }

    pub fn is_activated(&self) -> bool {
        self.activated
    }
    pub fn is_just_pressed(&self) -> bool {
        self.just_pressed
    }
    pub fn is_just_released(&self) -> bool {
        self.just_released
    }
}
