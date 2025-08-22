use spdlog::trace;

use crate::{application::Application, log::Log};

pub fn run(create_app: fn() -> Box<dyn Application>) {
    let log = Log::new(spdlog::LevelFilter::All);
    let app = create_app();
    trace!(logger: log.core_logger, "Running application...");
    app.run();
}
