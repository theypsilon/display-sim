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

use crate::ui_controller::enum_ui::{EnumHolder, EnumUi};
use enum_len_derive::EnumLen;
use num_derive::{FromPrimitive, ToPrimitive};

use super::EncodedValue;

#[derive(FromPrimitive, ToPrimitive, EnumLen, Copy, Clone)]
pub enum ScreenCurvatureKindOptions {
    Flat,
    Curved1,
    Curved2,
    Curved3,
    Pulse,
}

impl std::fmt::Display for ScreenCurvatureKindOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            ScreenCurvatureKindOptions::Flat => write!(f, "Flat"),
            ScreenCurvatureKindOptions::Curved1 => write!(f, "Curved 1"),
            ScreenCurvatureKindOptions::Curved2 => write!(f, "Curved 2"),
            ScreenCurvatureKindOptions::Curved3 => write!(f, "Curved 3"),
            ScreenCurvatureKindOptions::Pulse => write!(f, "Weavy"),
        }
    }
}

impl EnumUi for ScreenCurvatureKindOptions {
    fn event_tag(&self) -> &'static str {
        ""
    }
    fn keys_inc(&self) -> &[&'static str] {
        &["b", "screen-curvature-inc"]
    }
    fn keys_dec(&self) -> &[&'static str] {
        &["shift+b", "screen-curvature-dec"]
    }
    fn dispatch_tag(&self) -> &'static str {
        "back2front:screen_curvature"
    }
}

impl From<&'static dyn EncodedValue> for ScreenCurvatureKindOptions {
    fn from(value: &'static dyn EncodedValue) -> Self {
        value.to_usize()
    }
}

pub type ScreenCurvatureKind = EnumHolder<'static, ScreenCurvatureKindOptions>;
