use domain::post::post::Post;

use crate::errors::not_found::NotFoundError;

pub trait PostRepository {
    fn create(&self, post: &Post);
    fn get(&self, id: String) -> Result<Post, NotFoundError>;
}
