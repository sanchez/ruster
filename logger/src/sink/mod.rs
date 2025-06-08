use crate::LogMessagePayload;

pub trait LoggerSink {
    fn write(&self, message: LogMessagePayload);
}

mod console;
pub use console::*;
