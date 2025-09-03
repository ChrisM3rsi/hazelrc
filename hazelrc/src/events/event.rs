use std::any::Any;

use crate::events::{EventCategory, EventType};

pub trait Event: Any + Send + Sync + 'static {
    fn get_event_name(&self) -> EventType;
    fn get_event_category(&self) -> EventCategory;
    fn as_any(&self) -> &dyn Any;
}
