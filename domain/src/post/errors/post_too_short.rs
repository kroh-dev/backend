use errors::domain_error::{DomainError, DomainErrorContext};

#[derive(Debug)]
pub struct PostTooShort {
    context: DomainErrorContext,
}

impl PostTooShort {
    pub fn new(entity_id: String) -> Self {
        Self {
            context: DomainErrorContext { entity_id },
        }
    }
}

impl DomainError for PostTooShort {
    fn get_message(&self) -> &str {
        "Post is too short"
    }

    fn get_code(&self) -> &str {
        "POST_TOO_SHORT"
    }

    fn get_context(&self) -> errors::domain_error::DomainErrorContext {
        self.context.clone()
    }
}
