use async_trait::async_trait;

use super::super::{
  entity::Warehouse, repository::WarehouseRepository,
  use_case::create_warehouse::CreateWarehouse,
};

struct Repository {
  pub data: Vec<Warehouse>,
}

#[async_trait]
impl WarehouseRepository for Repository {
  async fn create(&mut self, name: String) -> Result<Warehouse, &'static str> {
    let new_warehouse = Warehouse { name };

    self.data.push(new_warehouse.clone());

    Ok(new_warehouse)
  }
  async fn get(&self, name: String) -> Option<Warehouse> {
    Some(Warehouse { name })
  }
}

#[tokio::test]
async fn test_create_warehouse() {
  let repository = Repository { data: vec![] };
  let mut use_case = CreateWarehouse::new(repository);

  let warehouse = Warehouse {
    name: String::from("test_warehouse"),
  };
  let new_warehouse = use_case.execute(warehouse.name.clone()).await.unwrap();

  assert_eq!(new_warehouse.name, warehouse.name);
}
