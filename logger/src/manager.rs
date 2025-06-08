use crate::{logger::Logger, sink::LoggerSink};

pub struct LogManager {
    sinks: Vec<Box<dyn LoggerSink>>,
}

impl LogManager {
    pub fn new() -> Self {
        LogManager { sinks: Vec::new() }
    }

    pub fn add_sink(&mut self, sink: Box<dyn LoggerSink>) {
        self.sinks.push(sink);
    }
}

impl Logger for LogManager {
    fn log(&self, message: crate::LogMessage) {
        let payload = crate::LogMessagePayload {
            message,
            timestamp: timer::SystemTime::now(),
            module: "default".to_string(), // Placeholder for module name
        };

        for sink in &self.sinks {
            sink.write(payload.clone());
        }
    }
}
