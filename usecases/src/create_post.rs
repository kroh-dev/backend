use async_trait::async_trait;
use domain::post::post::Post;
use std::sync::Arc;

use crate::{
    errors::UsecaseErrors, repositories::post_repository::PostRepository, usecase::Usecase,
};

pub struct CreatePostPayload {
    id: String,
    text: String,
    user_id: String,
}

impl CreatePostPayload {
    pub fn new(id: String, user_id: String, text: String) -> Self {
        Self { id, user_id, text }
    }
}

pub struct CreatePost {
    post_repository: Arc<dyn PostRepository + Send + Sync>,
}

impl CreatePost {
    pub fn new(post_repository: Arc<dyn PostRepository + Send + Sync>) -> Self {
        Self { post_repository }
    }
}

#[async_trait]
impl Usecase<CreatePostPayload, Post, UsecaseErrors> for CreatePost {
    async fn execute(&self, payload: CreatePostPayload) -> Result<Post, UsecaseErrors> {
        let post_result = Post::new(payload.id, payload.user_id, payload.text);

        if let Err(err) = post_result {
            return Err(UsecaseErrors::DomainError(err));
        }

        let post = post_result.ok().unwrap();

        let post_create_result = self.post_repository.create(post.clone()).await;

        if let Err(err) = post_create_result {
            return Err(UsecaseErrors::TechnicalError(err));
        }

        Ok(post)
    }
}

#[cfg(test)]
mod tests {

    struct InMemoryPostRepository {
        posts: Vec<Post>,
    }

    impl Default for InMemoryPostRepository {
        fn default() -> Self {
            Self {
                posts: Default::default(),
            }
        }
    }

    #[async_trait]
    impl PostRepository for InMemoryPostRepository {
        async fn create(&self, post: Post) -> Result<(), Box<dyn TechnicalError>> {
            // In a real repository we'll always be accessing a remote process
            // and thus we'll never be updating state
            // To avoid making the create method mutable in the trait
            // we use unsafe here.
            unsafe {
                let self_mut_ptr: *mut Self = self as *const Self as *mut Self;
                let posts_ptr: *mut Vec<Post> = &mut (*self_mut_ptr).posts;
                (*posts_ptr).push(post.clone());
            }
            Ok(())
        }

        async fn get(&self, id: String) -> Result<Post, Box<dyn TechnicalError>> {
            match self.posts.iter().find(|post| post.id == id) {
                Some(post) => Ok(post.clone()),
                None => Err(Box::new(NotFoundError::new("Post does not exist"))),
            }
        }
    }

    use crate::errors::{not_found::NotFoundError, technical_error::TechnicalError};

    use super::*;

    #[tokio::test]
    async fn alice_should_be_able_to_create_a_post() {
        let post_repository = Arc::new(InMemoryPostRepository::default());
        let id = "__ID__".to_string();
        let text = "hello world".to_string();
        let user_id = "__USER_ID__".to_string();

        let create_post = CreatePost::new(post_repository.clone());

        let create_post_result = create_post
            .execute(CreatePostPayload {
                id: id.clone(),
                text: text.clone(),
                user_id: user_id.clone(),
            })
            .await;

        assert!(create_post_result.is_ok());

        let post_result = post_repository.get("__ID__".into()).await;

        assert!(post_result.is_ok());

        let post = post_result.ok().unwrap();

        assert_eq!(post.id, id);
        assert_eq!(post.text, text);
        assert_eq!(post.user_id, user_id);
    }
}
