use crate::app_events::AppEventDispatcher;
use crate::general_types::{IncDec, OptionCursor};
use crate::simulation_context::SimulationContext;
use std::cmp::{PartialEq, PartialOrd};
use std::fmt::Display;
use std::ops::{AddAssign, DivAssign, MulAssign, SubAssign};

pub struct FilterParams<'a, T, U, TriggerHandler: FnOnce(U), Dispatcher: AppEventDispatcher> {
    ctx: &'a SimulationContext<Dispatcher>,
    var: &'a mut T,
    incdec: IncDec<bool>,
    trigger_handler: Option<TriggerHandler>,
    event_value: Option<T>,
    velocity: Option<T>,
    min: Option<T>,
    max: Option<T>,
    _u: std::marker::PhantomData<FnOnce(U)>,
}

impl<'a, T, U, TriggerHandler: FnOnce(U), Dispatcher: AppEventDispatcher> FilterParams<'a, T, U, TriggerHandler, Dispatcher> {
    pub fn new(ctx: &'a SimulationContext<Dispatcher>, var: &'a mut T, incdec: IncDec<bool>) -> Self {
        FilterParams {
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
    pub fn set_event_value(mut self, event_value: Option<T>) -> Self {
        self.event_value = event_value;
        self
    }
    pub fn set_trigger_handler(mut self, trigger_handler: TriggerHandler) -> Self {
        self.trigger_handler = Some(trigger_handler);
        self
    }
}

impl<'a, T: PartialOrd + PartialEq, TriggerHandler: FnOnce(T), Dispatcher: AppEventDispatcher> FilterParams<'a, T, T, TriggerHandler, Dispatcher> {
    pub fn set_progression(mut self, velocity: T) -> Self {
        self.velocity = Some(velocity);
        self
    }
    pub fn set_min(mut self, min: T) -> Self {
        self.min = Some(min);
        self
    }
    pub fn set_max(mut self, max: T) -> Self {
        self.max = Some(max);
        self
    }
}

impl<'a, T, TriggerHandler, Dispatcher> FilterParams<'a, T, &'a T, TriggerHandler, Dispatcher>
where
    T: OptionCursor + Display,
    TriggerHandler: FnOnce(&'a T),
    Dispatcher: AppEventDispatcher,
{
    #[allow(clippy::useless_let_if_seq)]
    pub fn process_options(self) {
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
                self.ctx.dispatcher.dispatch_minimum_value(self.var);
            } else if self.var.has_reached_maximum_limit() {
                self.ctx.dispatcher.dispatch_maximum_value(self.var);
            } else if let Some(handler) = self.trigger_handler {
                handler(self.var);
            }
        }
    }
}

impl<'a, T, TriggerHandler, Dispatcher> FilterParams<'a, T, T, TriggerHandler, Dispatcher>
where
    T: Display + AddAssign + SubAssign + PartialOrd + PartialEq + Copy + Default,
    TriggerHandler: FnOnce(T),
    Dispatcher: AppEventDispatcher,
{
    pub fn process_with_sums(self) {
        operate_filter(self, AddAssign::add_assign, SubAssign::sub_assign)
    }
}

impl<'a, T, TriggerHandler, Dispatcher> FilterParams<'a, T, T, TriggerHandler, Dispatcher>
where
    T: Display + MulAssign + DivAssign + PartialOrd + PartialEq + Copy + Default,
    TriggerHandler: FnOnce(T),
    Dispatcher: AppEventDispatcher,
{
    pub fn process_with_multiplications(self) {
        operate_filter(self, MulAssign::mul_assign, DivAssign::div_assign)
    }
}

fn operate_filter<T, TriggerHandler, Dispatcher>(
    params: FilterParams<T, T, TriggerHandler, Dispatcher>,
    inc_op: impl FnOnce(&mut T, T),
    dec_op: impl FnOnce(&mut T, T),
) where
    T: Display + PartialOrd + PartialEq + Copy + Default,
    TriggerHandler: FnOnce(T),
    Dispatcher: AppEventDispatcher,
{
    let last_value = *params.var;
    let velocity = if let Some(velocity) = params.velocity { velocity } else { Default::default() };
    if params.incdec.increase {
        inc_op(params.var, velocity);
    }
    if params.incdec.decrease {
        dec_op(params.var, velocity);
    }
    if let Some(val) = params.event_value {
        *params.var = val;
    }
    if last_value != *params.var {
        if let Some(min) = params.min {
            if *params.var < min {
                *params.var = min;
                params.ctx.dispatcher.dispatch_minimum_value(&min);
            }
        }
        if let Some(max) = params.max {
            if *params.var > max {
                *params.var = max;
                params.ctx.dispatcher.dispatch_maximum_value(&max);
            }
        }
        if let Some(handler) = params.trigger_handler {
            handler(*params.var);
        }
    }
}
