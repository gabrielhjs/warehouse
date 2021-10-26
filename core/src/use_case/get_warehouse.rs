use super::super::{
  entity::Warehouse,
  repository::{AssetRepository, ToolRepository, WarehouseRepository},
};

pub struct GetWarehouse<W, T, A> {
  warehouse_repo: W,
  tool_repo: T,
  asset_repo: A,
}

impl<W, T, A> GetWarehouse<W, T, A> {
  pub fn new(warehouse_repo: W, tool_repo: T, asset_repo: A) -> Self
  where
    W: WarehouseRepository,
    T: ToolRepository,
    A: AssetRepository,
  {
    GetWarehouse {
      warehouse_repo,
      tool_repo,
      asset_repo,
    }
  }
  pub async fn execute(&self, name: String) -> Option<Warehouse>
  where
    W: WarehouseRepository,
    T: ToolRepository,
    A: AssetRepository,
  {
    match self.warehouse_repo.get(&name).await {
      None => None,
      Some(mut warehouse) => {
        warehouse.tools =
          Some(self.tool_repo.get_by_warehouse(&warehouse.name).await);
        warehouse.assets =
          Some(self.asset_repo.get_by_warehouse(&warehouse.name).await);

        Some(warehouse)
      }
    }
  }
}
