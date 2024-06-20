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

#![allow(clippy::useless_attribute)]
#![allow(clippy::identity_op)]
#![allow(clippy::float_cmp)]

extern crate derive_new;

pub mod app_events;
mod boolean_actions;
mod boolean_button;
pub mod camera;
mod field_changer;
pub mod general_types;
pub mod input_types;
mod math;
pub mod simulation_context;
pub mod simulation_core_state;
pub mod simulation_core_ticker;
pub mod ui_controller;
