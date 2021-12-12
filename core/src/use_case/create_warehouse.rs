use async_trait::async_trait;

use super::super::{entity::Warehouse, repository::WarehouseRepository};
use super::base::UseCase;

pub struct Input {
  pub name: String,
}

pub struct CreateWarehouse<W> {
  warehouse_repo: W,
}

impl<W: WarehouseRepository> CreateWarehouse<W> {
  pub fn new(warehouse_repo: W) -> Self {
    CreateWarehouse { warehouse_repo }
  }
}

#[async_trait(?Send)]
impl<W: WarehouseRepository> UseCase for CreateWarehouse<W> {
  type Input = Input;
  type Output = Warehouse;

  fn name(&self) -> &'static str {
    "Create Warehouse"
  }

  async fn run(
    &self,
    params: &Self::Input,
  ) -> Result<Self::Output, &'static str> {
    let new_warehouse = Warehouse {
      id: String::from("1"),
      name: params.name.clone(),
      assets: Some(vec![]),
      tools: Some(vec![]),
    };

    self.warehouse_repo.save(&new_warehouse).await?;

    Ok(new_warehouse)
  }
}
