use warehouse_core::use_case::base::{MyInput, MyUseCase, UseCase};

#[tokio::main]
async fn main() {
  let use_case = MyUseCase {};

  let input = MyInput {
    name: String::from("alo"),
  };

  use_case.handle(&input).await;
}
