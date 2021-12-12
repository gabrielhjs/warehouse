use async_trait::async_trait;

#[async_trait(?Send)]
pub trait UseCase {
  type Input;
  type Output;

  fn name(&self) -> &'static str;

  async fn handle(
    &self,
    params: &Self::Input,
  ) -> Result<Self::Output, &'static str> {
    println!("|{}| requested", self.name());

    let result = self.run(params).await;

    match result {
      Ok(response) => {
        println!("|{}| exited with success", self.name());
        Ok(response)
      }
      Err(error) => {
        println!("|{}| exited with an error: {}", self.name(), error);
        Err(error)
      }
    }
  }

  async fn run(
    &self,
    params: &Self::Input,
  ) -> Result<Self::Output, &'static str>;
}

pub struct MyUseCase;

pub struct MyInput {
  pub name: String,
}

pub type MyOutput = String;

#[async_trait(?Send)]
impl UseCase for MyUseCase {
  type Input = MyInput;
  type Output = MyOutput;

  fn name(&self) -> &'static str {
    "My Use Case"
  }

  async fn run(
    &self,
    params: &Self::Input,
  ) -> Result<Self::Output, &'static str> {
    Ok(params.name.clone())
  }
}
