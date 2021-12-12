use async_trait::async_trait;

use super::super::{entity::Tool, repository::ToolRepository};
use super::base::UseCase;

pub struct Input {
  pub quantity: u32,
  pub tool_id: String,
  pub warehouse_id: String,
}

pub struct SubTools<T> {
  tool_repo: T,
}

impl<T: ToolRepository> SubTools<T> {
  pub fn new(tool_repo: T) -> Self {
    SubTools { tool_repo }
  }
}

#[async_trait(?Send)]
impl<T: ToolRepository> UseCase for SubTools<T> {
  type Input = Input;
  type Output = Tool;

  fn name(&self) -> &'static str {
    "Sub Tools"
  }
  async fn run(
    &self,
    params: &Self::Input,
  ) -> Result<Self::Output, &'static str> {
    match self
      .tool_repo
      .find(&params.tool_id, &params.warehouse_id)
      .await
    {
      None => Err("Invalid tool!"),
      Some(mut tool) => match tool.quantity >= params.quantity {
        false => Err("Insufficient stock!"),
        true => {
          tool.quantity -= params.quantity;
          self.tool_repo.save(&tool).await?;
          Ok(tool)
        }
      },
    }
  }
}
