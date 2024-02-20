use crate::errors::technical_error::TechnicalError;
use async_trait::async_trait;
use domain::post::post::Post;

#[async_trait]
pub trait PostRepository {
    async fn create(&self, post: Post) -> Result<(), Box<dyn TechnicalError>>;
    async fn get(&self, id: String) -> Result<Post, Box<dyn TechnicalError>>;
}
