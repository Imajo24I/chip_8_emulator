pub trait Error {
    fn name(&self) -> &'static str;
    fn message(&self) -> &'static str;
}

pub struct MissingFilepathError;

impl Error for MissingFilepathError {
    fn name(&self) -> &'static str {
        "MissingFilepathError"
    }

    fn message(&self) -> &'static str {
        "Missing filepath - Please specify the path to the chip 8 program to execute."
    }
}
