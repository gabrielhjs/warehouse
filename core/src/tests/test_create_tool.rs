use super::super::{
  entity::{Tool, Warehouse},
  use_case::{base::UseCase, create_tool},
};
use super::mock::{MockToolRepository, MockWarehouseRepository};

#[tokio::test]
async fn test_ok() {
  let warehouse_repo = MockWarehouseRepository {};
  let tool_repo = MockToolRepository {};

  let warehouse = Warehouse {
    id: String::from("1"),
    name: String::from("warehouse"),
    tools: None,
    assets: None,
  };

  let use_case = create_tool::CreateTool::new(tool_repo, warehouse_repo);

  let expected_tool = Tool {
    id: String::from("1"),
    name: String::from("tool"),
    quantity: 0,
    warehouse: warehouse.clone(),
  };

  let input = create_tool::Input {
    name: expected_tool.name.clone(),
    warehouse_name: String::from("warehouse"),
  };

  let new_tool = use_case.run(&input).await.unwrap();

  assert_eq!(new_tool, expected_tool);
}
