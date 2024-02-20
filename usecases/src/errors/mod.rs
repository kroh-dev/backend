use errors::domain_error::DomainError;

use self::technical_error::TechnicalError;

pub mod failed_get;
pub mod failed_insert;
pub mod not_found;
pub mod technical_error;

pub enum UsecaseErrors {
    TechnicalError(Box<dyn TechnicalError>),
    DomainError(Box<dyn DomainError>),
}
