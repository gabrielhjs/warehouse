use serde::Serialize;

#[derive(Serialize)]
pub struct Error {
  pub error: &'static str,
}
