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

use crate::ui_controller::enum_ui::{EnumHolder, EnumUi};
use app_util::log_error;
use enum_len_derive::EnumLen;
use num_derive::{FromPrimitive, ToPrimitive};

use crate::ui_controller::EncodedValue;

#[derive(FromPrimitive, ToPrimitive, EnumLen, Copy, Clone)]
pub enum TextureInterpolationOptions {
    Nearest = 0,
    Linear = 1,
}

impl std::fmt::Display for TextureInterpolationOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            TextureInterpolationOptions::Nearest => write!(f, "Nearest"),
            TextureInterpolationOptions::Linear => write!(f, "Linear"),
        }
    }
}

impl EnumUi for TextureInterpolationOptions {
    fn event_tag(&self) -> &'static str {
        ""
    }
    fn keys_inc(&self) -> &[&'static str] {
        &["h", "texture-interpolation-inc"]
    }
    fn keys_dec(&self) -> &[&'static str] {
        &["shift+h", "texture-interpolation-dec"]
    }
    fn dispatch_tag(&self) -> &'static str {
        "back2front:texture_interpolation"
    }
}

impl From<Box<dyn EncodedValue>> for TextureInterpolationOptions {
    fn from(value: Box<dyn EncodedValue>) -> Self {
        match value.to_usize() {
            Ok(0) => TextureInterpolationOptions::Nearest,
            Ok(1) => TextureInterpolationOptions::Linear,
            Ok(x) => {
                log_error(&format!("Unexpected TextureInterpolationOptions value {}", x));
                TextureInterpolationOptions::Linear
            }
            Err(e) => {
                log_error(&e);
                TextureInterpolationOptions::Linear
            }
        }
    }
}

pub type TextureInterpolation = EnumHolder<'static, TextureInterpolationOptions>;
