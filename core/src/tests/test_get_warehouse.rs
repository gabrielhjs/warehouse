use super::mock::{
  MockAssetRepository, MockToolRepository, MockWarehouseRepository,
};

use super::super::{
  entity::{Asset, Tool, Warehouse},
  use_case::get_warehouse::GetWarehouse,
};

#[tokio::test]
async fn test_valid_warehouse() {
  let warehouse_repo = MockWarehouseRepository {};
  let tool_repo = MockToolRepository {};
  let asset_repo = MockAssetRepository {};
  let use_case = GetWarehouse::new(warehouse_repo, tool_repo, asset_repo);

  let mut expected_warehouse = Warehouse {
    id: String::from("1"),
    name: String::from("test_warehouse"),
    assets: None,
    tools: None,
  };

  let related_tool = Tool {
    id: String::from("1"),
    name: String::from("test_tool"),
    warehouse: expected_warehouse.clone(),
  };

  let related_asset = Asset {
    id: String::from("1"),
    name: String::from("test_asset"),
    warehouse: expected_warehouse.clone(),
  };

  expected_warehouse.tools = Some(vec![related_tool]);
  expected_warehouse.assets = Some(vec![related_asset]);

  let warehouse = use_case
    .execute(expected_warehouse.name.clone())
    .await
    .unwrap();

  assert_eq!(expected_warehouse, warehouse);
}
