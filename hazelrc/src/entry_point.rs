use spdlog::trace;

use crate::{application::Application, log::{Log, CORE_LOGGER}};

pub fn run(create_app: fn() -> Box<dyn Application>) {
    let app = create_app();
    trace!(logger: CORE_LOGGER, "Running application...");
    app.run();
}
