use async_trait::async_trait;
#[async_trait]
pub trait Usecase<Payload, Data, Error> {
    async fn execute(&self, payload: Payload) -> Result<Data, Error>;
}
