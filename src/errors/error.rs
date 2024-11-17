use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct Error {
    pub message: String,
    pub cause: Cause,
}

impl Error {
    pub fn new(message: String, cause: Cause) -> Self {
        Self {
            message,
            cause,
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\nAdditional cause information:\n{}", self.message, self.cause)
    }
}

impl std::error::Error for Error {
    fn cause(&self) -> Option<&dyn std::error::Error> {
        self.cause.error.as_deref()
    }
}

#[derive(Debug)]
pub struct Cause {
    pub additional_info: Option<String>,
    pub error: Option<Box<dyn std::error::Error>>,
}

impl Cause {
    pub fn new(additional_info: Option<String>, error: Option<Box<dyn std::error::Error>>) -> Self {
        Self {
            additional_info,
            error,
        }
    }
}

impl Display for Cause {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(additonal_info) = &self.additional_info {
            write!(f, "{}", additonal_info)?;
        }

        if let Some(error) = &self.error {
            write!(f, "{}", error)?;
        }

        Ok(())
    }
}