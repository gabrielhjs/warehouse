use async_trait::async_trait;

use super::entity::*;

#[async_trait]
pub trait ActivityRepository {
  async fn save(activity: Activity);
}

#[async_trait]
pub trait WarehouseRepository {
  async fn save(&self, new_warehouse: &Warehouse) -> Result<(), &'static str>;
  async fn get(&self, name: &String) -> Option<Warehouse>;
}

#[async_trait]
pub trait ToolRepository {
  async fn save(&self, new_tool: &Tool) -> Result<(), &'static str>;
  async fn find(&self, tool_id: &String, warehouse_id: &String)
    -> Option<Tool>;
  async fn get_by_warehouse(&self, name: &String) -> Vec<Tool>;
}

#[async_trait]
pub trait AssetRepository {
  async fn save(&mut self, new_asset: &Asset) -> Result<(), &'static str>;
  async fn get(&self, name: &String) -> Option<Asset>;
  async fn get_by_warehouse(&self, name: &String) -> Vec<Asset>;
}
