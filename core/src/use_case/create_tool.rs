use async_trait::async_trait;

use super::super::{
  entity::Tool,
  repository::{ToolRepository, WarehouseRepository},
};
use super::base::UseCase;

pub struct Input {
  pub name: String,
  pub warehouse_name: String,
}

pub struct CreateTool<T, W> {
  tool_repo: T,
  warehouse_repo: W,
}

impl<T: ToolRepository, W: WarehouseRepository> CreateTool<T, W> {
  pub fn new(tool_repo: T, warehouse_repo: W) -> Self {
    CreateTool {
      tool_repo,
      warehouse_repo,
    }
  }
}

#[async_trait(?Send)]
impl<T: ToolRepository, W: WarehouseRepository> UseCase for CreateTool<T, W> {
  type Input = Input;
  type Output = Tool;

  fn name(&self) -> &'static str {
    "Create tool"
  }

  async fn run(&self, params: &Input) -> Result<Self::Output, &'static str> {
    match self.warehouse_repo.get(&params.warehouse_name).await {
      None => Err("Invalid warehouse!"),
      Some(warehouse) => {
        let new_tool = Tool {
          id: String::from("1"),
          name: params.name.clone(),
          warehouse,
          quantity: 0,
        };
        self.tool_repo.save(&new_tool).await?;
        Ok(new_tool)
      }
    }
  }
}
