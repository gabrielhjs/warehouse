use super::entity::Warehouse;
use async_trait::async_trait;

#[async_trait]
pub trait WarehouseRepository {
  async fn create(&mut self, name: String) -> Result<Warehouse, &'static str>;
  async fn get(&self, name: String) -> Option<Warehouse>;
}
