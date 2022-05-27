#[derive(Debug)]
pub struct File {
  pub name: String,
  pub package: String,
}

impl File {
  pub fn new(name: String) -> Self {
    Self {name, package: "".to_string()}
  }
}