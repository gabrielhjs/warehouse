use super::super::use_case::base;
use super::super::use_case::base::UseCase;

#[tokio::test]
async fn test_base() {
  let use_case = base::MyUseCase {};

  let input = base::MyInput {
    name: String::from("qualquer coisa"),
  };

  assert_eq!(
    use_case.handle(&input).await,
    Ok(String::from("qualquer coisa"))
  );
}
