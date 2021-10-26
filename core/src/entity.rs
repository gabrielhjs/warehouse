#[derive(Clone, Debug, PartialEq)]
pub struct Warehouse {
  pub id: String,
  pub name: String,
  pub tools: Option<Vec<Tool>>,
  pub assets: Option<Vec<Asset>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Asset {
  pub id: String,
  pub name: String,
  pub warehouse: Warehouse,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Tool {
  pub id: String,
  pub name: String,
  pub warehouse: Warehouse,
}
