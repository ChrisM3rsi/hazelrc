use crate::events::{EventCategory, EventType};

pub trait Event {
    fn get_event_name(&self) -> EventType;
    fn get_event_category(&self) -> EventCategory;
}