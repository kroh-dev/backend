mod mongodb;

use std::sync::Arc;

use usecases::{
    create_post::{CreatePost, CreatePostPayload},
    usecase::Usecase,
};

use crate::mongodb::{get_database, get_repositories};

#[tokio::main]
async fn main() -> Result<(), ()> {
    let uri = "mongodb://root:example@localhost:27017";

    let database = get_database(uri, "kroh")
        .await
        .expect("Expected mongodb connection to succeed");

    let post_repository = get_repositories(&database);

    let create_post_usecase = CreatePost::new(Arc::new(post_repository));

    let create_post_payload = CreatePostPayload::new(
        "__ID__".to_string(),
        "__USER_ID__".to_string(),
        "hello world".to_string(),
    );

    let create_post_result = create_post_usecase.execute(create_post_payload).await;

    if let Err(err) = create_post_result {
        match err {
            usecases::errors::UsecaseErrors::TechnicalError(err) => panic!("{}", err.get_message()),
            usecases::errors::UsecaseErrors::DomainError(err) => panic!("{}", err.get_message()),
        }
    }

    println!("{:?}", create_post_result.ok().unwrap());

    Ok(())
}
