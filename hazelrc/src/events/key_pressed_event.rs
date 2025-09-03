use std::any::Any;

use crate::events::{Event, EventCategory, EventType};

#[derive(Debug)]
pub struct KeyPressedEvent {
    pub key_code: u32,
    pub is_repeated: bool
}

impl KeyPressedEvent {
    pub fn new(key_code: u32, is_repeated: Option<bool>) -> Self {
        Self {
            key_code,
            is_repeated: is_repeated.unwrap_or(false),
        }
    }
}

impl Event for KeyPressedEvent  {
    fn get_event_name(&self) -> EventType {
        EventType::KeyPressed
    }
    
    fn get_event_category(&self) -> EventCategory {
        EventCategory::INPUT | EventCategory::KEYBOARD
    }
    
    fn as_any(&self) -> &dyn Any {
        self
    }
    
}