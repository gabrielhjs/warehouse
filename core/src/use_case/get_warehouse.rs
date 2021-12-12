use async_trait::async_trait;

use super::super::{
  entity::Warehouse,
  repository::{AssetRepository, ToolRepository, WarehouseRepository},
};
use super::base::UseCase;

pub struct Input {
  pub name: String,
}

pub struct GetWarehouse<W, T, A> {
  warehouse_repo: W,
  tool_repo: T,
  asset_repo: A,
}

impl<W: WarehouseRepository, T: ToolRepository, A: AssetRepository>
  GetWarehouse<W, T, A>
{
  pub fn new(warehouse_repo: W, tool_repo: T, asset_repo: A) -> Self {
    GetWarehouse {
      warehouse_repo,
      tool_repo,
      asset_repo,
    }
  }
}

#[async_trait(?Send)]
impl<W: WarehouseRepository, T: ToolRepository, A: AssetRepository> UseCase
  for GetWarehouse<W, T, A>
{
  type Input = Input;
  type Output = Option<Warehouse>;

  fn name(&self) -> &'static str {
    "Get Warehouse"
  }
  async fn run(
    &self,
    params: &Self::Input,
  ) -> Result<Self::Output, &'static str> {
    Ok(match self.warehouse_repo.get(&params.name).await {
      None => None,
      Some(mut warehouse) => {
        warehouse.tools =
          Some(self.tool_repo.get_by_warehouse(&warehouse.name).await);
        warehouse.assets =
          Some(self.asset_repo.get_by_warehouse(&warehouse.name).await);

        Some(warehouse)
      }
    })
  }
}
