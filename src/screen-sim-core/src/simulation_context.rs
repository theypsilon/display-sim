use crate::app_events::AppEventDispatcher;

#[derive(Default)]
pub struct SimulationContext<EventDispatcher: AppEventDispatcher + Default> {
    pub dispatcher: EventDispatcher,
}
