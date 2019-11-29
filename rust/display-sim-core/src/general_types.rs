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

use crate::boolean_button::BooleanButton;
use arraygen::Arraygen;
use enum_len_trait::EnumLen;
use num_traits::{FromPrimitive, ToPrimitive};

#[derive(Clone, Default, Arraygen)]
#[gen_array(pub fn get_buttons: &mut T)]
pub struct IncDec<T> {
    #[in_array(get_buttons)]
    pub increase: T,
    #[in_array(get_buttons)]
    pub decrease: T,
}

impl IncDec<BooleanButton> {
    pub fn any_just_pressed(&self) -> bool {
        self.increase.is_just_pressed() || self.decrease.is_just_pressed()
    }
    pub fn any_just_released(&self) -> bool {
        self.increase.is_just_released() || self.decrease.is_just_released()
    }
    pub fn to_just_pressed(&self) -> IncDec<bool> {
        IncDec {
            increase: self.increase.is_just_pressed(),
            decrease: self.decrease.is_just_pressed(),
        }
    }
}

impl IncDec<bool> {
    pub fn new(increase: bool, decrease: bool) -> IncDec<bool> {
        IncDec { increase, decrease }
    }
    pub fn any_active(self) -> bool {
        self.increase || self.decrease
    }
}

impl Copy for IncDec<bool> {}

pub trait DefaultReset {
    fn reset(&mut self)
    where
        Self: Default,
    {
        std::mem::swap(self, &mut Self::default());
    }
}

impl<T> DefaultReset for IncDec<T> where T: std::marker::Sized + std::default::Default {}

#[derive(Copy, Clone, Default, Debug)]
pub struct Size2D<T: Copy + Clone + Default> {
    pub width: T,
    pub height: T,
}

impl Size2D<u32> {
    pub fn to_f32(self) -> Size2D<f32> {
        Size2D {
            width: self.width as f32,
            height: self.height as f32,
        }
    }
}

impl<T: Copy + Clone + Default> Size2D<T> {
    pub fn to_tuple(self) -> (T, T) {
        (self.width, self.height)
    }
}

pub trait OptionCursor {
    fn next_option(&mut self);
    fn previous_option(&mut self);
    fn has_reached_maximum_limit(&self) -> bool;
    fn has_reached_minimum_limit(&self) -> bool;
}

impl<T> OptionCursor for T
where
    T: FromPrimitive + ToPrimitive + EnumLen,
{
    fn next_option(&mut self)
    where
        Self: FromPrimitive + ToPrimitive,
    {
        change_enum_variant(self, |u| u + 1)
    }

    fn previous_option(&mut self)
    where
        Self: FromPrimitive + ToPrimitive,
    {
        change_enum_variant(self, |u| if u == 0 { Self::len() - 1 } else { u - 1 })
    }

    fn has_reached_maximum_limit(&self) -> bool {
        false
    }
    fn has_reached_minimum_limit(&self) -> bool {
        false
    }
}

fn change_enum_variant<T: FromPrimitive + ToPrimitive + EnumLen>(instance: &mut T, action: impl FnOnce(usize) -> usize) {
    let mut changed = match instance.to_usize().and_then(|as_usize| FromPrimitive::from_usize(action(as_usize))) {
        Some(n) => n,
        None => FromPrimitive::from_usize(0).expect("Can't construct enum from 0."),
    };
    std::mem::swap(instance, &mut changed);
}

pub fn f32_to_u8(v: &[f32]) -> &[u8] {
    unsafe { std::slice::from_raw_parts(v.as_ptr() as *const u8, v.len() * 4) }
}

pub fn i32_to_u8(v: &[i32]) -> &[u8] {
    unsafe { std::slice::from_raw_parts(v.as_ptr() as *const u8, v.len() * 4) }
}

pub fn transform_u32_to_array_of_u8(x: u32) -> [u8; 4] {
    let b1: u8 = ((x >> 24) & 0xff) as u8;
    let b2: u8 = ((x >> 16) & 0xff) as u8;
    let b3: u8 = ((x >> 8) & 0xff) as u8;
    let b4: u8 = (x & 0xff) as u8;
    [b1, b2, b3, b4]
}

pub fn get_3_f32color_from_int(color: i32) -> [f32; 3] {
    [
        (color >> 16) as f32 / 255.0,
        ((color >> 8) & 0xFF) as f32 / 255.0,
        (color & 0xFF) as f32 / 255.0,
    ]
}

fn get_u8_from_f32color(color: f32) -> u32 {
    (match color {
        c if c < 0.0 => 0.0,
        c if c > 1.0 => 1.0,
        c => c,
    } * 255.0) as u32
}

pub fn get_int_from_3_f32color(color: &[f32; 3]) -> i32 {
    let r = get_u8_from_f32color(color[0]);
    let g = get_u8_from_f32color(color[1]);
    let b = get_u8_from_f32color(color[2]);
    ((r << 16) + (g << 8) + b) as i32
}

#[cfg(test)]
mod tests {
    mod get_3_f32color_from_int {
        mod gives_good {
            use super::super::super::*;

            macro_rules! get_3_f32color_from_int_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (input, expected) = $value;
                assert_eq!(expected, get_3_f32color_from_int(input));
            }
        )*
        }
    }

            get_3_f32color_from_int_tests! {
                white: (0x00FF_FFFF, [1.0, 1.0, 1.0]),
                black: (0x0000_0000, [0.0, 0.0, 0.0]),
                red: (0x00FF_0000, [1.0, 0.0, 0.0]),
                green: (0x0000_FF00, [0.0, 1.0, 0.0]),
                blue: (0x0000_00FF, [0.0, 0.0, 1.0]),
                yellow: (0x00eb_f114, [0.92156863, 0.94509804, 0.078431375]),
            }
        }
    }

    use super::*;

    #[test]
    fn test_get_int_from_3_f32color() {
        let expected = 0x00eb_f114;
        let actual = get_int_from_3_f32color(&get_3_f32color_from_int(expected));
        assert_eq!(actual, expected);
    }
}
