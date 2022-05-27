use thiserror::Error;
use tokens;

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

#[derive(Error, Debug)]
pub enum ParseError {
  #[error("{0}: expected '{1}'")]
  ExpectedToken(tokens::Pos, tokens::TokenKind),
  #[error("{0}: expected token")]
  TokenOutOfRange(tokens::Pos),
}