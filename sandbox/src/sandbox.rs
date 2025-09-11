use std::{cell::RefCell, rc::Rc};

use hazelrc::{
    Event, KeyPressedEvent,
    application::Application,
    events::{InputHandler, event_dispatcher::EventDispatcher},
    log::CORE_LOGGER,
    platform::app_window::{AppWindow, WindowsWindow},
};
use spdlog::trace;
use winit::event_loop::{ControlFlow, EventLoop};

pub struct Sandbox {
    pub win: WindowsWindow,
}

struct SandboxWrapper {
    inner: Rc<RefCell<Sandbox>>,
}

impl Sandbox {
    fn new() -> Sandbox {
        Sandbox {
            win: WindowsWindow::create("HazelRC".into(), 500, 500),
        }
    }

     fn on_event(e: &dyn Event) {
        trace!(logger: CORE_LOGGER, "{:?}", e)
    }
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

impl Application for SandboxWrapper {
    fn run(&self) {
        self.inner.borrow_mut().run();
    }

}

pub fn create_application() -> Box<dyn Application> {
    let mut sandbox = Sandbox::new();

    sandbox.win.set_event_callback(Box::new( |evnt| {
        Sandbox::on_event(evnt);
    }));

    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Wait);
    event_loop.run_app(&mut sandbox.win).unwrap();
    Box::new(sandbox)
}
