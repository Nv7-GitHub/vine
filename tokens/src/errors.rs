use thiserror::Error;
use super::Pos;

#[derive(Error, Debug)]
pub enum TokenizeError {
  #[error("{0}: unexpected character: {1}")]
  UnexpectedCharacter(Pos, char),
  #[error("{0}: invalid escape code: {1}")]
  InvalidEscapeCode(Pos, char),
}