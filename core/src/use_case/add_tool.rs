use async_trait::async_trait;

use super::super::{entity::Tool, repository::ToolRepository};
use super::base::UseCase;

pub struct Input {
  pub quantity: u32,
  pub warehouse_id: String,
  pub tool_id: String,
}

pub type Output = Result<Tool, &'static str>;

pub struct AddTools<T> {
  tool_repo: T,
}

impl<T: ToolRepository> AddTools<T> {
  pub fn new(tool_repo: T) -> Self {
    AddTools { tool_repo }
  }
}

#[async_trait(?Send)]
impl<T: ToolRepository> UseCase for AddTools<T> {
  type Input = Input;
  type Output = Tool;

  fn name(&self) -> &'static str {
    "Add Tools"
  }
  async fn run(
    &self,
    params: &Self::Input,
  ) -> Result<Self::Output, &'static str> {
    {
      match self
        .tool_repo
        .find(&params.tool_id, &params.warehouse_id)
        .await
      {
        None => Err("Invalid tool!"),
        Some(mut tool) => {
          tool.quantity += params.quantity.clone();
          self.tool_repo.save(&tool).await?;
          Ok(tool)
        }
      }
    }
  }
}
