use std::fmt::{Debug, Display};

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
    write!(f, "{}:{}:{}", self.file, self.line, self.char)
  }
}

impl Pos {
  pub fn new(file: &String, line: usize, char: usize) -> Self {
    Self {
      file: file.clone(),
      line: line,
      char: char,
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
}

#[derive(Debug)]
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

#[derive(Debug)]
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
  Operator(Operator),
}

pub struct Token {
  kind: TokenKind,
  pos: Pos,
}

impl Debug for Token {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    Display::fmt(&self, f)
  }
}

impl Display for Token {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.kind.fmt(f)?;
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