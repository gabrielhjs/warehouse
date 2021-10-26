use super::super::{
  entity::{Tool, Warehouse},
  use_case::create_tool::CreateTool,
};
use super::mock::{MockToolRepository, MockWarehouseRepository};

#[tokio::test]
async fn test_create_warehouse() {
  let warehouse_repo = MockWarehouseRepository {};
  let tool_repo = MockToolRepository {};

  let warehouse = Warehouse {
    id: String::from("1"),
    name: String::from("test_warehouse"),
    tools: None,
    assets: None,
  };

  let mut use_case = CreateTool::new(tool_repo, warehouse_repo);

  let expected_tool = Tool {
    id: String::from("1"),
    name: String::from("test_tool"),
    warehouse: warehouse.clone(),
  };

  let new_tool = use_case
    .execute(expected_tool.name.clone(), warehouse.name.clone())
    .await
    .unwrap();

  assert_eq!(new_tool, expected_tool);
}
