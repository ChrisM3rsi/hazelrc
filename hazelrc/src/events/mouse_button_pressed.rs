use std::any::Any;

use crate::{events::{EventCategory, EventType}, Event};

#[derive(Debug)]
pub struct MouseButtonPressed {
    pub height: u32,
    pub width: u32,
}

impl MouseButtonPressed {
    pub fn new(height: u32, width: u32) -> Self {
        Self { 
            height, 
            width 
        }
    }
}

impl Event for MouseButtonPressed {
    fn get_event_name(&self) -> EventType {
        EventType::MouseButtonPressed
    }

    fn get_event_category(&self) -> EventCategory {
        EventCategory::MOUSE | EventCategory::MOUSE_BUTTON
    }
    
    fn as_any(&self) -> &dyn Any {
        self
    }
}