use thiserror::Error;
use super::Pos;

#[derive(Error, Debug)]
pub enum TokenizeError {
  #[error("{0}: unexpected character: {1}")]
  UnexpectedCharacter(Pos, char)
}