use async_trait::async_trait;

use warehouse_core::{entity::*, repository::*};

pub struct MockWarehouseRepository;

#[async_trait]
impl WarehouseRepository for MockWarehouseRepository {
  async fn save(&self, _new_warehouse: &Warehouse) -> Result<(), &'static str> {
    Ok(())
  }
  async fn get(&self, name: &String) -> Option<Warehouse> {
    Some(Warehouse {
      id: String::from("1"),
      name: name.clone(),
      tools: None,
      assets: None,
    })
  }
}

pub struct MockToolRepository;

#[async_trait]
impl ToolRepository for MockToolRepository {
  async fn save(&self, _new_tool: &Tool) -> Result<(), &'static str> {
    Ok(())
  }
  async fn find(
    &self,
    tool_id: &String,
    warehouse_id: &String,
  ) -> Option<Tool> {
    Some(Tool {
      id: tool_id.clone(),
      name: String::from("tool"),
      quantity: 1,
      warehouse: Warehouse {
        id: warehouse_id.clone(),
        name: String::from("warehouse"),
        tools: None,
        assets: None,
      },
    })
  }
  async fn get_by_warehouse(&self, _name: &String) -> Vec<Tool> {
    vec![
      Tool {
        id: String::from("1"),
        name: String::from("tool"),
        quantity: 1,
        warehouse: Warehouse {
          id: String::from("1"),
          name: String::from("warehouse"),
          tools: None,
          assets: None,
        },
      },
    ]
  }
}

pub struct MockAssetRepository;

#[async_trait]
impl AssetRepository for MockAssetRepository {
  async fn save(&mut self, _new_asset: &Asset) -> Result<(), &'static str> {
    Ok(())
  }
  async fn get(&self, name: &String) -> Option<Asset> {
    Some(Asset {
      id: String::from("1"),
      name: name.clone(),
      warehouse: Warehouse {
        id: String::from("1"),
        name: String::from("warehouse"),
        tools: None,
        assets: None,
      },
    })
  }
  async fn get_by_warehouse(&self, _name: &String) -> Vec<Asset> {
    vec![
      Asset {
        id: String::from("1"),
        name: String::from("asset"),
        warehouse: Warehouse {
          id: String::from("1"),
          name: String::from("warehouse"),
          tools: None,
          assets: None,
        },
      },
    ]
  }
}
