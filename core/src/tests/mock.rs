use async_trait::async_trait;

use super::super::{entity::*, repository::*};

pub struct MockWarehouseRepository;

#[async_trait]
impl WarehouseRepository for MockWarehouseRepository {
  async fn save(
    &mut self,
    _new_warehouse: &Warehouse,
  ) -> Result<(), &'static str> {
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
  async fn save(&mut self, _new_tool: &Tool) -> Result<(), &'static str> {
    Ok(())
  }
  async fn get(&self, name: &String) -> Option<Tool> {
    Some(Tool {
      id: String::from("1"),
      name: name.clone(),
      warehouse: Warehouse {
        id: String::from("1"),
        name: String::from("test_warehouse"),
        tools: None,
        assets: None,
      },
    })
  }
  async fn get_by_warehouse(&self, _name: &String) -> Vec<Tool> {
    vec![
      Tool {
        id: String::from("1"),
        name: String::from("test_tool"),
        warehouse: Warehouse {
          id: String::from("1"),
          name: String::from("test_warehouse"),
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
        name: String::from("test_warehouse"),
        tools: None,
        assets: None,
      },
    })
  }
  async fn get_by_warehouse(&self, _name: &String) -> Vec<Asset> {
    vec![
      Asset {
        id: String::from("1"),
        name: String::from("test_asset"),
        warehouse: Warehouse {
          id: String::from("1"),
          name: String::from("test_warehouse"),
          tools: None,
          assets: None,
        },
      },
    ]
  }
}
