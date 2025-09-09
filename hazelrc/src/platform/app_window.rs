use spdlog::trace;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Fullscreen, Window},
};

use crate::{Event, log::CORE_LOGGER};

type EventCallbackFn = Box<dyn FnMut(&dyn Event)>;

pub struct WindowsWindow {
    // win: Window, //?
    pub title: String,
    pub width: u32,
    pub height: u32,
    window: Window,
}

impl ApplicationHandler for WindowsWindow {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.window = event_loop
            .create_window(Window::default_attributes())
            .unwrap();
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                trace!(logger: CORE_LOGGER, "closing window...");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                // Redraw the application.
                //
                // It's preferable for applications that do not render continuously to render in
                // this event rather than in AboutToWait, since rendering in here allows
                // the program to gracefully handle redraws requested by the OS.

                // Draw.

                // Queue a RedrawRequested event.
                //
                // You only need to call this if you've determined that you need to redraw in
                // applications which do not always need to. Applications that redraw continuously
                // can render here instead.
                self.window.request_redraw();
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
        let event_loop = EventLoop::new().unwrap();
        event_loop.set_control_flow(ControlFlow::Poll);

        let window = event_loop
            .create_window(Window::default_attributes())
            .unwrap();

        WindowsWindow {
            title,
            width,
            height,
            window,
        }
    }
}

impl AppWindow for WindowsWindow {
    fn create(title: String, width: u32, height: u32) -> Self {
        Self::new(title, width, height)
    }

    fn on_update(&self) {
        // &self.window.
    }
}

pub trait AppWindow {
    fn create(title: String, width: u32, height: u32) -> Self;
    fn on_update(&self);

    // fn set_event_callback()
}
