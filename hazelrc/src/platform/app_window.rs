use std::time::Duration;

use spdlog::trace;
use winit::{
    application::ApplicationHandler,
    dpi::LogicalSize,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    platform::pump_events::EventLoopExtPumpEvents,
    window::{Fullscreen, Window, WindowAttributes, WindowId},
};

use crate::{Event, KeyPressedEvent, log::CORE_LOGGER};

type EventCallbackFn = Box<dyn FnMut(&dyn Event)>;

pub struct WindowsWindow {
    pub title: String,
    pub width: u32,
    pub height: u32,
    window: Option<Window>,
    event_loop: Option<EventLoop<()>>,
    event_callback: Option<EventCallbackFn>,
}

impl ApplicationHandler for WindowsWindow {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.window = Some(event_loop
            .create_window(Window::default_attributes()
            .with_title(self.title.clone())
            .with_inner_size(LogicalSize::new(self.width, self.height))
        )
                
            .unwrap());
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                if let Some(callback) = &mut self.event_callback {
                    callback(&KeyPressedEvent::new(43, Some(false)));
                }

               if let Some(window) = &self.window {
                    window.request_redraw();
                }
            }
            _ => (),
        }
    }
}

impl WindowsWindow {
    pub fn new(title: String, width: u32, height: u32) -> Self {
        Self::init(title, width, height)
    }

    fn init(title: String, width: u32, height: u32) -> Self {
        // let event_loop = EventLoop::new().unwrap();
        // event_loop.set_control_flow(ControlFlow::Poll);

        // let window = event_loop
        //     .create_window(Window::default_attributes().with_title(title.clone()))
        //     .unwrap();

        WindowsWindow {
            title,
            width,
            height,
            window: None,
            event_loop: None,
            event_callback: None,
        }
    }
}

impl AppWindow for WindowsWindow {
    fn create(title: String, width: u32, height: u32) -> Self {
        Self::new(title, width, height)
    }

    fn on_update(&self) {
        // let timeout = Some(Duration::ZERO);
        // if let Some(mut event_loop) = self.event_loop.take() {
        //     let _ = event_loop.pump_app_events(timeout, self);

        //      self.event_loop = Some(event_loop);
        // }
    }

    fn set_event_callback(&mut self, callback_fn: EventCallbackFn) {
        self.event_callback = Some(callback_fn);
    }

    fn run(&self) {
    }
}

pub trait AppWindow {
    fn create(title: String, width: u32, height: u32) -> Self;
    fn on_update(&self);

    fn set_event_callback(&mut self, callback_fn: EventCallbackFn);
    fn run(&self);
}
