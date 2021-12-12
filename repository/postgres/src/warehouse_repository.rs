use async_trait::async_trait;
use diesel::{
  r2d2::{ConnectionManager, Pool},
  ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl,
};
use uuid::Uuid;

use crate::models::WarehouseTable;

use super::models::NewWarehouse;
use super::schema::warehouse;
use warehouse_core::{entity::Warehouse, repository::WarehouseRepository};

pub struct PostgresWarehouseRepository {
  pool: Pool<ConnectionManager<PgConnection>>,
}

#[async_trait]
impl WarehouseRepository for PostgresWarehouseRepository {
  async fn save(&self, data: &Warehouse) -> Result<(), &'static str> {
    match self.pool.get() {
      Ok(conn) => {
        let new_warehouse_schema = NewWarehouse {
          id: Uuid::new_v4(),
          name: data.name.clone(),
        };

        diesel::insert_into(warehouse::table)
          .values(&new_warehouse_schema)
          .get_result::<WarehouseTable>(&conn)
          .expect("Failed to create new Warehouse!");

        Ok(())
      }
      Err(_) => Err("Failed to connect to database!"),
    }
  }
  async fn get(&self, name: &String) -> Option<Warehouse> {
    match self.pool.get() {
      Ok(conn) => {
        match warehouse::table
          .filter(warehouse::dsl::name.eq(name))
          .first::<WarehouseTable>(&conn)
        {
          Ok(data) => Some(Warehouse {
            id: data.id.to_string(),
            name: data.name,
            tools: None,
            assets: None,
          }),
          Err(_) => None,
        }
      }
      Err(_) => None,
    }
  }
}
