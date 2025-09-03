use std::any::Any;

use crate::{
    Event,
    events::{EventCategory, EventType},
};

#[derive(Debug)]
pub struct KeyReleasedEvent {
    pub key_code: u32,
}

impl KeyReleasedEvent {
    pub fn new(key_code: u32) -> Self {
        Self {
            key_code
        }
    }
}

impl Event for KeyReleasedEvent {
    fn get_event_name(&self) -> EventType {
        EventType::KeyReleased
    }

    fn get_event_category(&self) -> EventCategory {
        EventCategory::INPUT | EventCategory::KEYBOARD
    }
    
    fn as_any(&self) -> &dyn Any {
        self
    }
}
