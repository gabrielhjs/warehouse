use async_trait::async_trait;

#[async_trait]
pub trait UseCase {
  fn new() -> Self;
  async fn execute() -> Result<(), &'static str>;
}
