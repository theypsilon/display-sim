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

use crate::app_events::{AppEventDispatcher, FakeEventDispatcher};
use derive_new::new;

#[derive(new)]
pub struct ConcreteSimulationContext<Dispatcher: AppEventDispatcher, Rnd: RandomGenerator> {
    pub dispatcher_instance: Dispatcher,
    pub rnd: Rnd,
}

impl<Dispatcher: AppEventDispatcher, Rnd: RandomGenerator> SimulationContext for ConcreteSimulationContext<Dispatcher, Rnd> {
    fn dispatcher(&self) -> &dyn AppEventDispatcher {
        &self.dispatcher_instance
    }
    fn random(&self) -> &dyn RandomGenerator {
        &self.rnd
    }
}

pub const fn make_fake_simulation_context() -> ConcreteSimulationContext<FakeEventDispatcher, FakeRngGenerator> {
    ConcreteSimulationContext {
        dispatcher_instance: FakeEventDispatcher {},
        rnd: FakeRngGenerator {},
    }
}

pub trait SimulationContext {
    fn dispatcher(&self) -> &dyn AppEventDispatcher;
    fn random(&self) -> &dyn RandomGenerator;
}

pub trait RandomGenerator {
    fn next(&self) -> f32;
}

pub struct FakeRngGenerator {}

impl RandomGenerator for FakeRngGenerator {
    fn next(&self) -> f32 {
        0.0
    }
}
