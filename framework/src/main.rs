mod mongodb;

use ::mongodb::{
    bson::{doc, Document},
    Collection,
};

use crate::mongodb::get_database;

#[tokio::main]
async fn main() -> Result<(), ()> {
    let uri = "mongodb://root:example@localhost:27017";
    let database = get_database(uri, "kroh")
        .await
        .expect("Expected mongodb connection to succeed");

    let my_coll: Collection<Document> = database.collection("movies");
    // Find a movie based on the title value
    let my_movie_result = my_coll
        .find_one(doc! { "title": "The Perils of Pauline" }, None)
        .await;
    // Print the document
    println!("Found a movie:\n{:#?}", my_movie_result.ok().unwrap());
    Ok(())
}
