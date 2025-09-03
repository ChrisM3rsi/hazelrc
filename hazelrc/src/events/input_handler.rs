use spdlog::trace;

use crate::{KeyPressedEvent, events::event_dispatcher::EventHandler, log::CORE_LOGGER};

pub struct InputHandler {
    name: String,
}

impl InputHandler {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}

impl EventHandler<KeyPressedEvent> for InputHandler {
    fn handle(&mut self, event: &KeyPressedEvent) {
        trace!(logger: CORE_LOGGER, "{}: Key {}, is repeated {}",  self.name, event.key_code, event.is_repeated);
        
    }
}
