use super::super::{entity::Warehouse, repository::WarehouseRepository};

pub struct CreateWarehouse<W> {
  warehouse_repo: W,
}

impl<W> CreateWarehouse<W> {
  pub fn new(warehouse_repo: W) -> Self
  where
    W: WarehouseRepository,
  {
    CreateWarehouse { warehouse_repo }
  }
  pub async fn execute(
    &mut self,
    name: String,
  ) -> Result<Warehouse, &'static str>
  where
    W: WarehouseRepository,
  {
    let new_warehouse = Warehouse {
      id: String::from("1"),
      name,
      assets: Some(vec![]),
      tools: Some(vec![]),
    };

    self.warehouse_repo.save(&new_warehouse).await?;

    Ok(new_warehouse)
  }
}
