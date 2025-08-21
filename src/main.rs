mod log;

use spdlog::{trace, warn, error};

use crate::log::Log;


fn main() {
    let log = Log::new(spdlog::LevelFilter::All);
    trace!(logger: log.core_logger, "tracing");
    error!(logger: log.app_logger, "Error");
}
