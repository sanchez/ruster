use crate::LogMessage;

pub trait Logger {
    fn log(&self, message: LogMessage);
}

pub trait LoggerHelpers {
    fn debug(&self, message: String);
    fn verbose(&self, message: String);
    fn info(&self, message: String);
    fn success(&self, message: String);
    fn warning(&self, message: String);
    fn error(&self, message: String);
}

impl LoggerHelpers for dyn Logger {
    fn debug(&self, message: String) {
        self.log(LogMessage::Debug(message));
    }

    fn verbose(&self, message: String) {
        self.log(LogMessage::Verbose(message));
    }

    fn info(&self, message: String) {
        self.log(LogMessage::Info(message));
    }

    fn success(&self, message: String) {
        self.log(LogMessage::Success(message));
    }

    fn warning(&self, message: String) {
        self.log(LogMessage::Warning(message));
    }

    fn error(&self, message: String) {
        self.log(LogMessage::Error(message));
    }
}
