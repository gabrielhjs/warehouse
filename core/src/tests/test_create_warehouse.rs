use super::super::{
  entity::Warehouse,
  use_case::{base::UseCase, create_warehouse},
};
use super::mock::MockWarehouseRepository;

#[tokio::test]
async fn test_ok() {
  let warehouse_repo = MockWarehouseRepository {};
  let use_case = create_warehouse::CreateWarehouse::new(warehouse_repo);

  let warehouse = Warehouse {
    id: String::from("1"),
    name: String::from("warehouse"),
    assets: Some(vec![]),
    tools: Some(vec![]),
  };

  let input = create_warehouse::Input {
    name: String::from("warehouse"),
  };

  let new_warehouse = use_case.handle(&input).await.unwrap();

  assert_eq!(new_warehouse, warehouse);
}
