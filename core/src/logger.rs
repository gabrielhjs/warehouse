pub trait Logger {
  fn debug(msg: String) {
    println!("DEBUG: {}", msg)
  }
  fn info(msg: String) {
    println!("INFO: {}", msg)
  }
  fn warn(msg: String) {
    println!("WARN: {}", msg)
  }
  fn error(msg: String) {
    println!("ERROR: {}", msg)
  }
}
