use super::super::{
  entity::Warehouse, use_case::create_warehouse::CreateWarehouse,
};
use super::mock::MockWarehouseRepository;

#[tokio::test]
async fn test_create_warehouse() {
  let warehouse_repo = MockWarehouseRepository {};
  let mut use_case = CreateWarehouse::new(warehouse_repo);

  let warehouse = Warehouse {
    id: String::from("1"),
    name: String::from("test_warehouse"),
    assets: Some(vec![]),
    tools: Some(vec![]),
  };

  let new_warehouse = use_case.execute(warehouse.name.clone()).await.unwrap();

  assert_eq!(new_warehouse, warehouse);
}
