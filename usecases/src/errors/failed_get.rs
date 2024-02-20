use super::technical_error::TechnicalError;

pub struct FailedGetError {
    message: String,
    code: String,
}

impl FailedGetError {
    pub fn new(message: &str) -> Self {
        let mut full_message = "Resource not found: ".to_string();
        full_message.push_str(message);

        Self {
            message: full_message,
            code: "FAILED_TO_GET".to_string(),
        }
    }
}

impl TechnicalError for FailedGetError {
    fn get_message(&self) -> String {
        self.message.clone()
    }

    fn get_code(&self) -> String {
        self.code.clone()
    }
}
