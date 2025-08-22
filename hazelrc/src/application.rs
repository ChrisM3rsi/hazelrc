use spdlog::trace;

use crate::log::CORE_LOGGER;

pub trait Application {
    fn run(&self) {
        trace!(logger: CORE_LOGGER, "Application is running...");
        loop {}
    }
}

