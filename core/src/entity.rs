use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Warehouse {
  pub id: String,
  pub name: String,
  pub tools: Option<Vec<Tool>>,
  pub assets: Option<Vec<Asset>>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Asset {
  pub id: String,
  pub name: String,
  pub warehouse: Warehouse,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Tool {
  pub id: String,
  pub name: String,
  pub quantity: u32,
  pub warehouse: Warehouse,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Activity {
  pub id: String,
  pub user: String,
  pub datetime: String,
  pub available: bool,
  pub activity_type: String,
}
