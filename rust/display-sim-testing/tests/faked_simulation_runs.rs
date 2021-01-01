/* Copyright (c) 2019-2021 José manuel Barroso Galindo <theypsilon@gmail.com>
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

use display_sim_testing::fake::FakeVideoInput;

#[test]
fn test_simulation_initializes_fine() {
    assert_eq!(FakeVideoInput::default().iterate_times(0).is_ok(), true);
}

#[test]
fn test_simulation_run_2_iterations_fine() {
    assert_eq!(FakeVideoInput::default().iterate_times(2).is_ok(), true);
}
