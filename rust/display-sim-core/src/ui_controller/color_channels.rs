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

use std::convert::TryFrom;

use crate::ui_controller::enum_ui::{EnumHolder, EnumUi};
use app_util::AppError;
use enum_len_derive::EnumLen;
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::FromPrimitive;

use crate::ui_controller::EncodedValue;

#[derive(FromPrimitive, ToPrimitive, EnumLen, Copy, Clone)]
pub enum ColorChannelsOptions {
    Combined,
    Overlapping,
    SplitHorizontal,
    SplitVertical,
}

impl std::fmt::Display for ColorChannelsOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            ColorChannelsOptions::Combined => write!(f, "Combined"),
            ColorChannelsOptions::Overlapping => write!(f, "Horizontal overlapping"),
            ColorChannelsOptions::SplitHorizontal => write!(f, "Horizontal split"),
            ColorChannelsOptions::SplitVertical => write!(f, "Vertical split"),
        }
    }
}

impl EnumUi for ColorChannelsOptions {
    fn event_tag(&self) -> &'static str {
        ""
    }
    fn keys_inc(&self) -> &[&'static str] {
        &["c", "color-representation-inc"]
    }
    fn keys_dec(&self) -> &[&'static str] {
        &["shift+c", "color-representation-dec"]
    }
    fn dispatch_tag(&self) -> &'static str {
        "back2front:color_representation"
    }
}

impl TryFrom<Box<dyn EncodedValue>> for ColorChannelsOptions {
    type Error = AppError;

    fn try_from(value: Box<dyn EncodedValue>) -> Result<Self, Self::Error> {
        if let Some(option) = ColorChannelsOptions::from_usize(value.to_usize()?) {
            Ok(option)
        } else {
            Err(AppError::from("Incorrect value for ColorChannelsOptions"))
        }
    }
}

pub type ColorChannels = EnumHolder<'static, ColorChannelsOptions>;
