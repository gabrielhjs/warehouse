use diesel::{Insertable, Queryable};

use super::schema::warehouse;

#[derive(Queryable)]
pub struct WarehouseTable {
  pub id: uuid::Uuid,
  pub name: String,
}

#[derive(Insertable)]
#[table_name = "warehouse"]
pub struct NewWarehouse {
  pub id: uuid::Uuid,
  pub name: String,
}
