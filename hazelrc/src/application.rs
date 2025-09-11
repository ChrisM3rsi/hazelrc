use spdlog::trace;

use crate::{
    events::{event_dispatcher::EventDispatcher, InputHandler}, log::CORE_LOGGER, Event, KeyPressedEvent
};

pub trait Application: 'static{
    fn run(&self);
}
