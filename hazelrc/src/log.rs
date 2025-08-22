use std::sync::Arc;

use spdlog::{LevelFilter, Logger, sink::StdStreamSink};

pub struct Log {
    pub core_logger: Logger,
    pub app_logger: Logger,
}

impl Log {
    pub fn new(level: LevelFilter) -> Self {
        let sink: Arc<dyn spdlog::sink::Sink> = Arc::new(StdStreamSink::builder().stdout().build().unwrap());
        
        let core_logger = Logger::builder()
            .name("HAZELRC")
            .level_filter(level)
            .sink(Arc::clone(&sink))
            .build()
            .unwrap();

        let app_logger = Logger::builder()
            .name("APP")
            .level_filter(level)
            .sink(Arc::clone(&sink))
            .build()
            .unwrap();

        Self {
            core_logger,
            app_logger,
        }
    }
}

// static void Init()
