use mongodb::{Client, Database};

pub async fn get_database(uri: &str, database_name: &str) -> mongodb::error::Result<Database> {
    let client = Client::with_uri_str(uri).await?;
    let database = client.database(database_name);
    Ok(database)
}
