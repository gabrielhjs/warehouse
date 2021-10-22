use super::super::{entity::Warehouse, repository::WarehouseRepository};

struct GetWarehouse<R> {
  repository: R,
}

impl<R> GetWarehouse<R> {
  fn new(repository: R) -> Self
  where
    R: WarehouseRepository,
  {
    GetWarehouse { repository }
  }
  pub async fn execute(&self, name: String) -> Option<Warehouse>
  where
    R: WarehouseRepository,
  {
    self.repository.get(name).await
  }
}
