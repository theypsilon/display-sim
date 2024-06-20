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

#![allow(clippy::identity_op)]

pub mod background_render;
pub mod blur_render;
pub mod internal_resolution_render;
pub mod pixels_render;
pub mod render_types;
pub mod rgb_render;
mod shaders;
pub mod simulation_draw;
pub mod simulation_render_state;

pub mod error {
    pub use app_util::*;
}
