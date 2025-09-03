use std::{
    any::TypeId,
    collections::HashMap,
};

use spdlog::warn;

use crate::{Event, log::CORE_LOGGER};

type BoxedHandler = Box<dyn FnMut(&dyn Event) + Send + 'static>;

pub trait EventHandler<T: Event>: Send + 'static {
    fn handle(&mut self, event: &T);
}

pub struct EventDispatcher {
    handlers: HashMap<TypeId, Vec<BoxedHandler>>,
}

impl EventDispatcher {
    pub fn new() -> Self {
        Self {
            handlers: HashMap::new(),
        }
    }

    pub fn subscribe<T, H>(&mut self, mut handler: H)
    where
        T: Event,
        H: EventHandler<T> + Send + 'static,
    {
        let type_id = TypeId::of::<T>();
        let boxed_handler: BoxedHandler = Box::new(move |event| {
            if let Some(typed_event) = event.as_any().downcast_ref::<T>() {
                handler.handle(typed_event);
            }
        });

        self.handlers
            .entry(type_id)
            .or_insert_with(Vec::new)
            .push(boxed_handler);
    }

    pub fn dispatch<T>(&mut self, event: &T)
    where
        T: Event,
    {
        let type_id = TypeId::of::<T>();
        if let Some(handlers) = self.handlers.get_mut(&type_id) {
            for handler in handlers.iter_mut() {
                handler(event);
            }
        } else {
            warn!(logger: CORE_LOGGER, "handler not found for event {:?}", event.get_event_name())
        }
    }

    pub fn clear_handlers<T>(&mut self)
    where
        T: Event,
    {
        let type_id = TypeId::of::<T>();
        self.handlers.remove(&type_id);
    }
}
