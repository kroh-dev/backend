use mongodb::{Client, Database};

use self::post_repository::MongoPostRepository;

pub mod post_repository;

pub async fn get_database(uri: &str, database_name: &str) -> mongodb::error::Result<Database> {
    let client = Client::with_uri_str(uri).await?;
    let database = client.database(database_name);
    Ok(database)
}

pub fn get_repositories(database: &Database) -> MongoPostRepository {
    let post_collection = database.collection("post");
    let post_repository = MongoPostRepository::new(post_collection);
    post_repository
}
