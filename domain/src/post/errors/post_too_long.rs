use errors::domain_error::{DomainError, DomainErrorContext};

#[derive(Debug)]
pub struct PostTooLong {
    context: DomainErrorContext,
}

impl PostTooLong {
    pub fn new(entity_id: String) -> Self {
        Self {
            context: DomainErrorContext { entity_id },
        }
    }
}

impl DomainError for PostTooLong {
    fn get_message(&self) -> &str {
        "Post exceeded the maximum allowed characters"
    }

    fn get_code(&self) -> &str {
        "POST_TOO_LONG"
    }

    fn get_context(&self) -> errors::domain_error::DomainErrorContext {
        self.context.clone()
    }
}
