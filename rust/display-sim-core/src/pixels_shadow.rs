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
use crate::general_types::OptionCursor;
use std::fmt::{Display, Error, Formatter};

pub const TEXTURE_SIZE: usize = 510;

const SHADOWS_LEN: usize = 24;

#[derive(Default, Clone, Copy)]
pub struct ShadowShape {
    pub value: usize,
}

impl Display for ShadowShape {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}", self.value)
    }
}

impl OptionCursor for ShadowShape {
    fn next_option(&mut self) {
        self.value += 1;
        if self.value >= SHADOWS_LEN {
            self.value = 0;
        }
    }
    fn previous_option(&mut self) {
        if self.value == 0 {
            self.value = SHADOWS_LEN;
        }
        self.value -= 1;
    }
    fn has_reached_maximum_limit(&self) -> bool {
        false
    }

    fn has_reached_minimum_limit(&self) -> bool {
        false
    }
}

pub fn get_shadows() -> [Box<Fn(usize, usize) -> f64>; SHADOWS_LEN] {
    [
        Box::new(|_i, _j| 255.0),
        Box::new(|i, j| calc_with_log(i, 0) * calc_with_log(j, 0) * 1.0 * 255.0),
        Box::new(|i, j| calc_with_log(i, 1) * calc_with_log(j, 1) * 1.5 * 255.0),
        Box::new(|i, j| calc_with_log(i, 2) * calc_with_log(j, 2) * 3.0 * 255.0),
        Box::new(|i, j| (calc_with_log(i, 0) * 0.9 + calc_with_log(j, 0) * 0.1) * 1.0 * 255.0),
        Box::new(|i, j| (calc_with_log(i, 1) * 0.9 + calc_with_log(j, 1) * 0.1) * 1.5 * 255.0),
        Box::new(|i, j| (calc_with_log(i, 2) * 0.9 + calc_with_log(j, 2) * 0.1) * 3.0 * 255.0),
        Box::new(|i, j| (calc_with_log(i, 3) * 0.9 + calc_with_log(j, 3) * 0.1) * 6.0 * 255.0),
        Box::new(|i, j| (calc_with_log(i, 0) * 0.8 + calc_with_log(j, 0) * 0.2) * 1.0 * 255.0),
        Box::new(|i, j| (calc_with_log(i, 1) * 0.8 + calc_with_log(j, 1) * 0.2) * 1.5 * 255.0),
        Box::new(|i, j| (calc_with_log(i, 2) * 0.8 + calc_with_log(j, 2) * 0.2) * 3.0 * 255.0),
        Box::new(|i, j| (calc_with_log(i, 3) * 0.8 + calc_with_log(j, 3) * 0.2) * 6.0 * 255.0),
        Box::new(|i, j| (calc_with_log(i, 0) * 0.5 + calc_with_log(j, 0) * 0.5) * 1.0 * 255.0),
        Box::new(|i, j| (calc_with_log(i, 1) * 0.5 + calc_with_log(j, 1) * 0.5) * 1.5 * 255.0),
        Box::new(|i, j| (calc_with_log(i, 2) * 0.5 + calc_with_log(j, 2) * 0.5) * 3.0 * 255.0),
        Box::new(|i, j| (calc_with_log(i, 3) * 0.5 + calc_with_log(j, 3) * 0.5) * 6.0 * 255.0),
        Box::new(|i, _j| calc_with_log(i, 0) * 1.0 * 255.0),
        Box::new(|i, _j| calc_with_log(i, 1) * 1.5 * 255.0),
        Box::new(|i, _j| calc_with_log(i, 2) * 3.0 * 255.0),
        Box::new(|i, _j| calc_with_log(i, 3) * 6.0 * 255.0),
        Box::new(|i, _j| calc_with_log(i, 4) * 9.0 * 255.0),
        Box::new(|i, j| calc_diamond(i, 0) * calc_diamond(j, 0) * 1.0 * 255.0),
        Box::new(|i, _j| calc_diamond(i, 0) * 1.0 * 255.0),
        Box::new(|i, _j| calc_diamond(i, 1) * 1.5 * 255.0),
    ]
}

fn calc_with_log(number: usize, count: usize) -> f64 {
    let result = log(TEXTURE_SIZE - number);
    pow(result, count)
}
fn log(number: usize) -> f64 {
    f64::log(number as f64, (TEXTURE_SIZE / 2) as f64)
}
fn calc_diamond(number: usize, count: usize) -> f64 {
    let result = 1.0 - ((number - TEXTURE_SIZE / 2) as f64 / (TEXTURE_SIZE as f64 / 2.0));
    pow(result, count)
}
fn pow(mut number: f64, count: usize) -> f64 {
    for _i in 0..count {
        number *= number;
    }
    number
}
