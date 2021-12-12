use super::mock::{
  MockAssetRepository, MockToolRepository, MockWarehouseRepository,
};

use super::super::{
  entity::{Asset, Tool, Warehouse},
  use_case::{base::UseCase, get_warehouse},
};

#[tokio::test]
async fn test_ok() {
  let warehouse_repo = MockWarehouseRepository {};
  let tool_repo = MockToolRepository {};
  let asset_repo = MockAssetRepository {};
  let use_case =
    get_warehouse::GetWarehouse::new(warehouse_repo, tool_repo, asset_repo);

  let mut expected_warehouse = Warehouse {
    id: String::from("1"),
    name: String::from("warehouse"),
    assets: None,
    tools: None,
  };

  let related_tool = Tool {
    id: String::from("1"),
    name: String::from("tool"),
    quantity: 1,
    warehouse: expected_warehouse.clone(),
  };

  let related_asset = Asset {
    id: String::from("1"),
    name: String::from("asset"),
    warehouse: expected_warehouse.clone(),
  };

  let input = get_warehouse::Input {
    name: String::from("warehouse"),
  };

  expected_warehouse.tools = Some(vec![related_tool]);
  expected_warehouse.assets = Some(vec![related_asset]);

  let warehouse = use_case.handle(&input).await.unwrap();

  assert_eq!(Some(expected_warehouse), warehouse);
}
