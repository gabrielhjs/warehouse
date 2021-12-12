use super::super::use_case::{base::UseCase, sub_tools};
use super::mock::MockToolRepository;

#[tokio::test]
async fn test_ok() {
  let tool_repo = MockToolRepository {};
  let use_case = sub_tools::SubTools::new(tool_repo);
  let input = sub_tools::Input {
    quantity: 1u32,
    tool_id: String::from("1"),
    warehouse_id: String::from("1"),
  };

  let tool = use_case.handle(&input).await.unwrap();

  assert_eq!(0u32, tool.quantity);
}

#[tokio::test]
async fn test_insufficient_stock() {
  let tool_repo = MockToolRepository {};
  let use_case = sub_tools::SubTools::new(tool_repo);
  let input = sub_tools::Input {
    quantity: 2u32,
    tool_id: String::from("1"),
    warehouse_id: String::from("1"),
  };

  let tool = use_case.handle(&input).await;

  assert_eq!(Err("Insufficient stock!"), tool);
}
