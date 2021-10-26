use super::super::{
  entity::Tool,
  repository::{ToolRepository, WarehouseRepository},
};

pub struct CreateTool<T, W> {
  tool_repo: T,
  warehouse_repo: W,
}

impl<T, W> CreateTool<T, W> {
  pub fn new(tool_repo: T, warehouse_repo: W) -> Self
  where
    T: ToolRepository,
    W: WarehouseRepository,
  {
    CreateTool {
      tool_repo,
      warehouse_repo,
    }
  }
  pub async fn execute(
    &mut self,
    name: String,
    warehouse_name: String,
  ) -> Result<Tool, &'static str>
  where
    T: ToolRepository,
    W: WarehouseRepository,
  {
    match self.warehouse_repo.get(&warehouse_name).await {
      None => Err("Invalid warehouse!"),
      Some(warehouse) => {
        let new_tool = Tool {
          id: String::from("1"),
          name,
          warehouse,
        };
        self.tool_repo.save(&new_tool).await?;
        Ok(new_tool)
      }
    }
  }
}
