use super::super::{entity::Warehouse, repository::WarehouseRepository};

pub struct CreateWarehouse<R> {
  repository: R,
}

impl<R> CreateWarehouse<R> {
  pub fn new(repository: R) -> Self
  where
    R: WarehouseRepository,
  {
    CreateWarehouse { repository }
  }
  pub async fn execute(
    &mut self,
    name: String,
  ) -> Result<Warehouse, &'static str>
  where
    R: WarehouseRepository,
  {
    self.repository.create(name).await
  }
}
