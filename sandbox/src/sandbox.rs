use hazelrc::{
    KeyPressedEvent,
    application::Application,
    events::{InputHandler, event_dispatcher::EventDispatcher},
    log::CORE_LOGGER,
    platform::app_window::{AppWindow, WindowsWindow},
};
use spdlog::trace;

pub struct Sandbox {
    pub win: WindowsWindow,
}

impl Application for Sandbox {
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

        loop {
           WindowsWindow::on_update(&self.win);
        }
    }
}

pub fn create_application() -> Box<dyn Application> {
    let sandbox = Sandbox {
        win: WindowsWindow::create("HazelRC".into(), 500, 500),
    };

    Box::new(sandbox)
}
