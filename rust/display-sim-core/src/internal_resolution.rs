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

use crate::general_types::{OptionCursor, Size2D};
use std::fmt::{Display, Error, Formatter};

#[derive(Clone)]
pub struct InternalResolution {
    max_texture_size: i32,
    viewport: Size2D<i32>,
    minimum_reached: bool,
    maximium_reached: bool,
}

impl Default for InternalResolution {
    fn default() -> Self {
        InternalResolution {
            max_texture_size: std::i32::MAX,
            viewport: Size2D { width: 3840, height: 2160 },
            minimum_reached: false,
            maximium_reached: false,
        }
    }
}

impl InternalResolution {
    pub fn set_max_texture_size(&mut self, value: i32) {
        self.max_texture_size = value;
    }
    pub fn set_resolution(&mut self, resolution: i32) {
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
            std::i32::MIN..=0 => 1080,
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
