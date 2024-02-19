#[derive(Clone, Debug)]
pub struct DomainErrorContext {
    pub entity_id: String,
}

impl std::fmt::Display for DomainErrorContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.entity_id)
    }
}

pub trait DomainError {
    fn get_message(&self) -> &str;
    fn get_code(&self) -> &str;
    fn get_context(&self) -> DomainErrorContext;
}

// Implement the Display trait for DomainError
impl std::fmt::Display for dyn DomainError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}: {} ({})",
            self.get_code(),
            self.get_message(),
            self.get_context()
        )
    }
}
