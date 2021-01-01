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

use crate::general_types::{IncDec, OptionCursor};
use crate::simulation_context::SimulationContext;
use std::cmp::{PartialEq, PartialOrd};
use std::fmt::Display;
use std::ops::{AddAssign, DivAssign, MulAssign, SubAssign};

pub(crate) struct FieldChanger<'a, T, U, TriggerHandler: FnOnce(U)> {
    ctx: &'a dyn SimulationContext,
    var: &'a mut T,
    incdec: IncDec<bool>,
    trigger_handler: Option<TriggerHandler>,
    event_value: Option<T>,
    velocity: Option<T>,
    min: Option<T>,
    max: Option<T>,
    _u: std::marker::PhantomData<dyn FnOnce(U)>,
}

impl<'a, T, U, TriggerHandler: FnOnce(U)> FieldChanger<'a, T, U, TriggerHandler> {
    pub(crate) fn new(ctx: &'a dyn SimulationContext, var: &'a mut T, incdec: IncDec<bool>) -> Self {
        FieldChanger {
            ctx,
            var,
            incdec,
            trigger_handler: None,
            event_value: None,
            velocity: None,
            min: None,
            max: None,
            _u: Default::default(),
        }
    }
    pub(crate) fn set_event_value(mut self, event_value: Option<T>) -> Self {
        self.event_value = event_value;
        self
    }
    pub(crate) fn set_trigger_handler(mut self, trigger_handler: TriggerHandler) -> Self {
        self.trigger_handler = Some(trigger_handler);
        self
    }
}

impl<'a, T: PartialOrd + PartialEq + AddAssign + SubAssign, TriggerHandler: FnOnce(T)> FieldChanger<'a, T, T, TriggerHandler> {
    pub(crate) fn set_progression(mut self, velocity: T) -> Self {
        self.velocity = Some(velocity);
        self
    }
    pub(crate) fn set_min(mut self, min: T) -> Self {
        self.min = Some(min);
        self
    }
    pub(crate) fn set_max(mut self, max: T) -> Self {
        self.max = Some(max);
        self
    }
}

impl<'a, T, TriggerHandler> FieldChanger<'a, T, &'a T, TriggerHandler>
where
    T: OptionCursor + Display,
    TriggerHandler: FnOnce(&'a T),
{
    #[allow(clippy::useless_let_if_seq)]
    pub(crate) fn process_options(self) -> bool {
        let mut changed = false;
        if self.incdec.increase {
            self.var.next_option();
            changed = true;
        }
        if self.incdec.decrease {
            self.var.previous_option();
            changed = true;
        }
        if let Some(val) = self.event_value {
            *self.var = val;
            changed = true;
        }
        if changed {
            if self.var.has_reached_minimum_limit() {
                self.ctx.dispatcher().dispatch_minimum_value(self.var);
            } else if self.var.has_reached_maximum_limit() {
                self.ctx.dispatcher().dispatch_maximum_value(self.var);
            } else if let Some(handler) = self.trigger_handler {
                handler(self.var);
                return true;
            }
        }
        false
    }
}

impl<'a, T, TriggerHandler> FieldChanger<'a, T, T, TriggerHandler>
where
    T: Display + AddAssign + SubAssign + PartialOrd + PartialEq + Copy + Default,
    TriggerHandler: FnOnce(T),
{
    pub(crate) fn process_with_sums(self) -> bool {
        operate_filter(self, AddAssign::add_assign, SubAssign::sub_assign)
    }
}

impl<'a, T, TriggerHandler> FieldChanger<'a, T, T, TriggerHandler>
where
    T: Display + MulAssign + DivAssign + PartialOrd + PartialEq + Copy + Default,
    TriggerHandler: FnOnce(T),
{
    pub(crate) fn process_with_multiplications(self) -> bool {
        operate_filter(self, MulAssign::mul_assign, DivAssign::div_assign)
    }
}

fn operate_filter<T, TriggerHandler>(params: FieldChanger<T, T, TriggerHandler>, inc_op: impl FnOnce(&mut T, T), dec_op: impl FnOnce(&mut T, T)) -> bool
where
    T: Display + PartialOrd + PartialEq + Copy + Default,
    TriggerHandler: FnOnce(T),
{
    let last_value = *params.var;
    let is_min = if let Some(min) = params.min { *params.var <= min } else { false };
    let is_max = if let Some(max) = params.max { *params.var >= max } else { false };
    let velocity = if let Some(velocity) = params.velocity { velocity } else { Default::default() };
    if !is_max && params.incdec.increase {
        inc_op(params.var, velocity);
    }
    if !is_min && params.incdec.decrease {
        dec_op(params.var, velocity);
    }
    if let Some(val) = params.event_value {
        *params.var = val;
    }
    if let Some(min) = params.min {
        if *params.var < min || (is_min && params.incdec.decrease) {
            *params.var = min;
            params.ctx.dispatcher().dispatch_minimum_value(&min);
        }
    }
    if let Some(max) = params.max {
        if *params.var > max || (is_max && params.incdec.increase) {
            *params.var = max;
            params.ctx.dispatcher().dispatch_maximum_value(&max);
        }
    }
    if last_value != *params.var {
        if let Some(handler) = params.trigger_handler {
            handler(*params.var);
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]

    use super::*;
    use crate::app_events::FakeEventDispatcher;
    use crate::simulation_context::{make_fake_simulation_context, ConcreteSimulationContext, FakeRngGenerator};

    static INCDEC_DOWN: IncDec<bool> = IncDec {
        increase: false,
        decrease: true,
    };
    static INCDEC_UP: IncDec<bool> = IncDec {
        increase: true,
        decrease: false,
    };
    static INCDEC_NONE: IncDec<bool> = IncDec {
        increase: false,
        decrease: false,
    };
    static INCDEC_BOTH: IncDec<bool> = IncDec {
        increase: true,
        decrease: true,
    };

    mod process_options {
        use super::*;
        use enum_len_derive::EnumLen;
        use num_derive::{FromPrimitive, ToPrimitive};

        #[derive(FromPrimitive, ToPrimitive, EnumLen, Clone, Copy, PartialEq, Debug)]
        enum OptionKind {
            A,
            B,
            C,
        }

        #[test]
        fn process_options__has_some_event_value__changes_parameter() {
            let mut actual = OptionKind::A;
            sut_ref(&mut actual, INCDEC_UP).set_event_value(Some(OptionKind::C)).process_options();
            assert_eq!(actual, OptionKind::C);
        }

        #[test]
        fn process_options__with_false_incdec__does_not_change_parameter() {
            let mut actual = OptionKind::A;
            sut_ref(&mut actual, INCDEC_NONE).process_options();
            assert_eq!(actual, OptionKind::A);
        }

        #[test]
        fn process_options__with_true_inc__changes_parameter_up_as_expected() {
            let mut actual = OptionKind::A;
            sut_ref(&mut actual, INCDEC_UP).process_options();
            assert_eq!(actual, OptionKind::B);
        }

        #[test]
        fn process_options__with_true_dec__changes_parameter_down_as_expected() {
            let mut actual = OptionKind::B;
            sut_ref(&mut actual, INCDEC_DOWN).process_options();
            assert_eq!(actual, OptionKind::A);
        }

        #[test]
        fn process_options__with_true_incdec__does_not_change_parameter() {
            let mut actual = OptionKind::A;
            sut_ref(&mut actual, INCDEC_BOTH).process_options();
            assert_eq!(actual, OptionKind::A);
        }

        #[test]
        fn trigger_handler__on_change__triggers() {
            let mut actual = OptionKind::A;
            let mut triggered = false;
            FieldChanger::new(&CTX, &mut actual, INCDEC_DOWN)
                .set_trigger_handler(|_: &OptionKind| triggered = true)
                .process_options();
            assert_eq!(triggered, true);
        }

        impl std::fmt::Display for OptionKind {
            fn fmt(&self, _: &mut std::fmt::Formatter) -> std::fmt::Result {
                Ok(())
            }
        }
    }

    mod process_with_sums {
        use super::*;

        #[test]
        fn set_event_value__has_some__changes_parameter() {
            let mut actual = 0;
            sut(&mut actual, INCDEC_UP).set_event_value(Some(3)).process_with_sums();
            assert_eq!(actual, 3);
        }

        #[test]
        fn set_progression__with_false_incdec__does_not_change_parameter() {
            let mut actual = 0;
            sut(&mut actual, INCDEC_NONE).set_progression(1).process_with_sums();
            assert_eq!(actual, 0);
        }

        #[test]
        fn set_progression__with_true_inc__changes_parameter_up_as_expected() {
            let mut actual = 0;
            sut(&mut actual, INCDEC_UP).set_progression(1).process_with_sums();
            assert_eq!(actual, 1);
        }

        #[test]
        fn set_progression__with_true_dec__changes_parameter_down_as_expected() {
            let mut actual = 0;
            sut(&mut actual, INCDEC_DOWN).set_progression(1).process_with_sums();
            assert_eq!(actual, -1);
        }

        #[test]
        fn set_progression__with_true_incdec__does_not_change_parameter() {
            let mut actual = 0;
            sut(&mut actual, INCDEC_BOTH).set_progression(1).process_with_sums();
            assert_eq!(actual, 0);
        }

        #[test]
        fn set_min__when_going_down__blocks_change() {
            let mut actual = 0;
            sut(&mut actual, INCDEC_DOWN).set_progression(1).set_min(0).process_with_sums();
            assert_eq!(actual, 0);
        }

        #[test]
        fn set_min__when_going_up__doesnt_block_change() {
            let mut actual = 0;
            sut(&mut actual, INCDEC_UP).set_progression(1).set_min(0).process_with_sums();
            assert_eq!(actual, 1);
        }

        #[test]
        fn set_max__when_going_up__blocks_change() {
            let mut actual = 0;
            sut(&mut actual, INCDEC_UP).set_progression(1).set_max(0).process_with_sums();
            assert_eq!(actual, 0);
        }

        #[test]
        fn set_max__when_going_down__blocks_change() {
            let mut actual = 0;
            sut(&mut actual, INCDEC_DOWN).set_progression(1).set_max(0).process_with_sums();
            assert_eq!(actual, -1);
        }

        #[test]
        fn trigger_handler__on_change__triggers() {
            let mut actual = 0;
            let mut triggered = false;
            FieldChanger::new(&CTX, &mut actual, INCDEC_DOWN)
                .set_trigger_handler(|_| triggered = true)
                .set_progression(1)
                .process_with_sums();
            assert_eq!(triggered, true);
        }

        #[test]
        fn trigger_handler__on_blocked_change__doesnt_trigger() {
            let mut actual = 0;
            let mut triggered = false;
            FieldChanger::new(&CTX, &mut actual, INCDEC_DOWN)
                .set_trigger_handler(|_| triggered = true)
                .set_progression(1)
                .set_min(0)
                .process_with_sums();
            assert_eq!(triggered, false);
        }
    }

    mod process_with_multiplications {
        use super::*;

        #[test]
        fn on_increase__multiplies_by_progression() {
            let mut actual: i32 = 5;
            sut(&mut actual, INCDEC_UP).set_progression(3).process_with_multiplications();
            assert_eq!(actual, 15);
        }

        #[test]
        fn on_decrease__divides_by_progression() {
            let mut actual: i32 = 15;
            sut(&mut actual, INCDEC_DOWN).set_progression(3).process_with_multiplications();
            assert_eq!(actual, 5);
        }
    }

    static CTX: ConcreteSimulationContext<FakeEventDispatcher, FakeRngGenerator> = make_fake_simulation_context();

    fn sut<'a, T>(parameter: &'a mut T, incdec: IncDec<bool>) -> FieldChanger<'a, T, T, impl FnOnce(T)> {
        FieldChanger::new(&CTX, parameter, incdec).set_trigger_handler(|_| {})
    }

    fn sut_ref<'a, T>(parameter: &'a mut T, incdec: IncDec<bool>) -> FieldChanger<'a, T, &'a T, impl FnOnce(&'a T)> {
        FieldChanger::new(&CTX, parameter, incdec).set_trigger_handler(|_| {})
    }
}
