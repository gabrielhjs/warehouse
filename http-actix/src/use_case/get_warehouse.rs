use actix_web::{HttpRequest, HttpResponse};
use repository_postgres::warehouse_repository::PostgresWarehouseRepository;
use warehouse_core::use_case::{base::UseCase, get_warehouse};

use super::super::{entity::Error, repository};

pub async fn http_get_warehouse(request: HttpRequest) -> HttpResponse {
  let name = request.match_info().get("name").unwrap_or("");
  match get_warehouse::GetWarehouse::new(
      repository::PostgresWarehouseRepository {
          pool: 
      },
    repository::MockToolRepository {},
    repository::MockAssetRepository {},
  )
  .handle(&get_warehouse::Input { name: name.into() })
  .await
  {
    Ok(warehouse) => HttpResponse::Created().json(warehouse),
    Err(error) => HttpResponse::InternalServerError().json(Error { error }),
  }
}
