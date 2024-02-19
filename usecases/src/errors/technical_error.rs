pub trait TechnicalError {
    fn get_message(&self) -> String;
    fn get_code(&self) -> String;
}

impl std::fmt::Display for dyn TechnicalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.get_code(), self.get_message())
    }
}
