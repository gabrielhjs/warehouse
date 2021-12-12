use super::super::use_case::{add_tool, base::UseCase};
use super::mock::MockToolRepository;

#[tokio::test]
async fn test_ok() {
  let tool_repo = MockToolRepository {};
  let use_case = add_tool::AddTools::new(tool_repo);
  let input = add_tool::Input {
    quantity: 1u32,
    tool_id: String::from("1"),
    warehouse_id: String::from("1"),
  };

  let tool = use_case.handle(&input).await.unwrap();

  assert_eq!(2u32, tool.quantity);
}
