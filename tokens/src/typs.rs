use std::fmt::{Debug, Display};
use thiserror::Error;

#[derive(Clone)]
pub struct Pos {
  pub file: String,
  pub line: usize,
  pub char: usize,
  pub end_line: usize,
  pub end_char: usize,
}

impl Debug for Pos {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    Display::fmt(&self, f)
  }
}

impl Display for Pos {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}:{}:{}", self.file, self.line + 1, self.char + 1)
  }
}

impl Pos {
  pub fn new(file: &String, line: usize, char: usize) -> Self {
    Self {
      file: file.clone(),
      line,
      char,
      end_line: line,
      end_char: char + 1,
    }
  }

  pub fn extend(&self, pos: Pos) -> Pos {
    Pos {
      file: self.file.clone(),
      line: self.line,
      char: self.char,
      end_line: pos.end_line,
      end_char: pos.end_char,
    }
  }

  pub fn start(pos: Pos) -> Pos {
    Pos {
      file: pos.file.clone(),
      line: pos.end_line,
      char: pos.end_char,
      end_line: pos.end_line,
      end_char: pos.end_char + 1,
    }
  }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Operator {
  // Comparison ops
  Equal,
  NotEqual,
  Greater,
  GreaterEqual,
  Less,
  LessEqual,
  // Logical ops
  Or,
  Not,
  And,
  // Math Ops
  Add,
  Subtract,
  Multiply,
  Divide,
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
  Ident(String),
  End, // Semicolon
  String(String),
  LParen,
  RParen,
  LBrack,
  RBrack,
  Comma,
  Selector, // Period
  Number(String),
  Assign,
  Define,
  Operator(Operator),
  Comment(String),
}

impl Display for TokenKind {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    Debug::fmt(&self, f)
  }
}

#[derive(Clone)]
pub struct Token {
  pub kind: TokenKind,
  pub pos: Pos,
}

impl Debug for Token {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    Display::fmt(&self, f)
  }
}

impl Display for Token {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    Debug::fmt(&self.kind, f)?;
    write!(f, "[{}]", self.pos.to_string())
  }
}

impl Token {
  pub fn new(pos: Pos, val: TokenKind) -> Self {
    Self {
      pos,
      kind: val,
    }
  }
}


#[derive(Error, Debug)]
pub enum TokenizeError {
  #[error("{0}: unexpected character: {1}")]
  UnexpectedCharacter(Pos, char),
  #[error("{0}: invalid escape code: {1}")]
  InvalidEscapeCode(Pos, char),
}