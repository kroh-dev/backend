pub trait Usecase<Payload, Data, Error> {
    fn execute(&self, payload: Payload) -> Result<Data, Error>;
}
