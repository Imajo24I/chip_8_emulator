use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct Error {
    message: &'static str,
    cause: Option<Box<dyn std::error::Error>>,
}

impl Error {
    pub fn new(message: &'static str) -> Self {
        Self {
            message,
            cause: None,
        }
    }

    pub fn new_with_cause(message: &'static str, cause: Box<dyn std::error::Error>) -> Self {
        Self {
            message,
            cause: Some(cause),
        }
    }

    pub fn message(&self) -> &str {
        self.message
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(cause) = &self.cause {
            write!(f, "{}\nCaused by:\n{}", self.message, cause)
        } else {
            write!(f, "{}", self.message)
        }
    }
}

impl std::error::Error for Error {
    fn cause(&self) -> Option<&dyn std::error::Error> {
        self.cause.as_deref()
    }
}
