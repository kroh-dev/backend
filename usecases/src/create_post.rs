use std::rc::Rc;

use domain::post::post::Post;
use errors::domain_error::DomainError;

use crate::{repositories::post_repository::PostRepository, usecase::Usecase};

struct CreatePostPayload {
    id: String,
    text: String,
    user_id: String,
}

struct CreatePost {
    post_repository: Rc<dyn PostRepository>,
}

type Error = Box<dyn DomainError>;

impl Usecase<CreatePostPayload, Post, Error> for CreatePost {
    fn execute(&self, payload: CreatePostPayload) -> Result<Post, Error> {
        let post_result = Post::new(payload.id, payload.user_id, payload.text);

        if post_result.is_err() {
            return Err(post_result.err().unwrap());
        }

        let post = post_result.ok().unwrap();

        self.post_repository.create(&post);

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

    impl PostRepository for InMemoryPostRepository {
        fn create(&self, post: &Post) {
            // In a real repository we'll always be accessing a remote process
            // and thus we'll never be updating state
            // To avoid making the create method mutable in the trait
            // we use unsafe here.
            unsafe {
                let self_mut_ptr: *mut Self = self as *const Self as *mut Self;
                let posts_ptr: *mut Vec<Post> = &mut (*self_mut_ptr).posts;
                (*posts_ptr).push(post.clone());
            }
        }

        fn get(&self, id: String) -> Result<Post, NotFoundError> {
            match self.posts.iter().find(|post| post.id == id) {
                Some(post) => Ok(post.clone()),
                None => Err(NotFoundError::new("Post does not exist")),
            }
        }
    }

    use crate::errors::not_found::NotFoundError;

    use super::*;

    #[test]
    fn alice_should_be_able_to_create_a_post() {
        let post_repository: Rc<dyn PostRepository> = Rc::new(InMemoryPostRepository::default());
        let create_post = CreatePost {
            post_repository: Rc::clone(&post_repository),
        };

        let id = "__ID__".to_string();
        let text = "hello world".to_string();
        let user_id = "__USER_ID__".to_string();

        let create_post_result = create_post.execute(CreatePostPayload {
            id: id.clone(),
            text: text.clone(),
            user_id: user_id.clone(),
        });

        assert!(create_post_result.is_ok());

        let post_result = post_repository.get("__ID__".into());

        assert!(post_result.is_ok());

        let post = post_result.ok().unwrap();

        assert_eq!(post.id, id);
        assert_eq!(post.text, text);
        assert_eq!(post.user_id, user_id);
    }
}
