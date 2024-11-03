use eframe::egui::{FontId, RichText};

pub struct Error {
    pub error_code: u8,
    pub error_message: RichText,
}

impl Clone for Error {
    fn clone(&self) -> Self {
        Self {
            error_code: self.error_code,
            error_message: self.error_message.clone(),
        }
    }
}

pub enum Errors {
    MissingFilePathArg,
}

impl Errors {
    pub fn get_error(&self) -> Error {
        match self {
            Errors::MissingFilePathArg => Error {
                error_code: 1,
                error_message: RichText::new("Missing filepath - Please specify the path to the chip 8 program to execute.")
                    .font(FontId::proportional(20f32)),
            }
        }
    }
}
