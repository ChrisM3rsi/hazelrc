use spdlog::trace;

use crate::{
    KeyPressedEvent,
    events::{InputHandler, event_dispatcher::EventDispatcher},
    log::CORE_LOGGER,
};

pub trait Application {
    fn run(&self) {
        trace!(logger: CORE_LOGGER, "Application is running...");
        let mut dispatcher = EventDispatcher::new();

        let input_handler = InputHandler::new("Alekos");
        
        dispatcher.subscribe(input_handler);

        let key_event = KeyPressedEvent {
            key_code: 65,
            is_repeated: false,
        };

        dispatcher.dispatch(&key_event);

        loop {}
    }
}
