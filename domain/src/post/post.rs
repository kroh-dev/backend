use super::errors::post_too_long::PostTooLong;

const MAXIMUM_CHARACTERS: usize = 0xFFF;

#[derive(Clone)]
pub struct Post {
    pub id: String,
    pub user_id: String,
    pub text: String,
}

impl Post {
    pub fn new(id: String, user_id: String, text: String) -> Result<Self, PostTooLong> {
        if text.len() > MAXIMUM_CHARACTERS {
            return Err(PostTooLong::new(id));
        }

        Ok(Self { id, text, user_id })
    }
}

#[cfg(test)]
mod tests {
    use errors::domain_error::DomainError;

    use super::*;

    struct PostFactory {
        pub id: String,
        pub user_id: String,
        pub text: String,
    }

    impl PostFactory {
        fn default() -> Self {
            PostFactory {
                id: "__ID__".into(),
                user_id: "__USER_ID__".into(),
                text: "hello world".into(),
            }
        }

        fn with_max_chars(&mut self) -> &Self {
            let text_list: Vec<&str> = vec!["a"; MAXIMUM_CHARACTERS as usize];
            self.text = text_list.join("");
            self
        }

        fn with_more_than_max_chars(&mut self) -> &Self {
            let text_list: Vec<&str> = vec!["a"; MAXIMUM_CHARACTERS as usize + 1];
            self.text = text_list.join("");
            self
        }

        fn build(&self) -> Result<Post, PostTooLong> {
            Post::new(self.id.clone(), self.user_id.clone(), self.text.clone())
        }
    }

    #[test]
    fn alice_should_be_able_to_create_a_post() {
        let factory = PostFactory::default();

        let post_result = factory.build();

        assert!(post_result.is_ok());

        let post = post_result.expect("lets gooo");

        assert_eq!(post.id, "__ID__".to_string());
        assert_eq!(post.user_id, "__USER_ID__".to_string());
        assert_eq!(post.text, "hello world".to_string());
    }

    #[test]
    fn alice_should_be_able_to_create_a_post_with_maximum_characters() {
        let mut factory = PostFactory::default();
        factory.with_max_chars();

        let post_result = factory.build();

        assert!(post_result.is_ok());
        let post = post_result.unwrap();

        assert_eq!(post.text.len(), MAXIMUM_CHARACTERS);
    }

    #[test]
    fn alice_should_not_be_able_to_create_a_post_which_exceeds_maximum_allowed_characters() {
        let mut factory = PostFactory::default();

        factory.with_more_than_max_chars();

        let post_result = factory.build();

        assert!(post_result.is_err());

        let err = post_result.err().unwrap();

        assert_eq!(
            err.get_message(),
            "Post exceeded the maximum allowed characters"
        );
        assert_eq!(err.get_code(), "POST_TOO_LONG");
        assert_eq!(err.get_context().entity_id, "__ID__");
    }
}
