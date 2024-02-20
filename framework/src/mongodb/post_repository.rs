use async_trait::async_trait;
use domain::post::post::Post;
use mongodb::{bson::doc, Collection};
use usecases::{
    errors::{failed_get::FailedGetError, failed_insert::FailedInsertError},
    repositories::post_repository::PostRepository,
};

pub struct MongoPostRepository {
    collection: Collection<Post>,
}

impl MongoPostRepository {
    pub fn new(collection: Collection<Post>) -> Self {
        Self { collection }
    }
}

#[async_trait]
impl PostRepository for MongoPostRepository {
    async fn create(
        &self,
        post: Post,
    ) -> Result<(), Box<dyn usecases::errors::technical_error::TechnicalError>> {
        let result = self.collection.insert_one(post, None).await;

        if let Err(err) = result {
            // TODO: log the error
            println!("{}", err);
            return Err(Box::new(FailedInsertError::new("Failed to insert post")));
        }

        Ok(())
    }

    async fn get(
        &self,
        id: String,
    ) -> Result<Post, Box<dyn usecases::errors::technical_error::TechnicalError>> {
        let result = self.collection.find_one(doc! { "id": id }, None).await;

        if let Err(err) = result {
            // TODO: log the error
            println!("{}", err);
            return Err(Box::new(FailedGetError::new("Failed to get post")));
        }

        let post = result.ok().unwrap().unwrap();
        Ok(post)
    }
}
