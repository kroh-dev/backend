use domain::post::post::Post;

use crate::errors::technical_error::TechnicalError;

pub trait PostRepository {
    fn create(&self, post: &Post) -> Result<(), Box<dyn TechnicalError>>;
    fn get(&self, id: String) -> Result<Post, Box<dyn TechnicalError>>;
}
