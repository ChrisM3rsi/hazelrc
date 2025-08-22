use std::sync::Arc;

use spdlog::{LevelFilter, Logger, sink::StdStreamSink};
use once_cell::sync::Lazy;

pub struct Log {
    pub core_logger: Logger,
    pub app_logger: Logger,

    
}
pub static CORE_LOGGER: Lazy<Logger> = Lazy::new(|| {
    let sink: Arc<dyn spdlog::sink::Sink> =
        Arc::new(StdStreamSink::builder().stdout().build().unwrap());

    Logger::builder()
        .name("HAZELRC")
        .level_filter(LevelFilter::All)
        .sink(sink)
        .build()
        .unwrap()
});

pub static APP_LOGGER: Lazy<Logger> = Lazy::new(|| {
    let sink: Arc<dyn spdlog::sink::Sink> =
        Arc::new(StdStreamSink::builder().stdout().build().unwrap());

    Logger::builder()
        .name("APP")
        .level_filter(LevelFilter::All)
        .sink(sink)
        .build()
        .unwrap()
});
