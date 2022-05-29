use std::collections::HashMap;

use thiserror::Error;
use tokens;

#[derive(Debug)]
pub struct File {
  pub name: String,
  pub imports: Vec<Import>,
  pub functions: HashMap<String, Function>,
}

#[derive(Debug)]
pub struct Import {
  pub path: Vec<String>,
  pub name: String,
}

#[derive(Debug)]
pub struct Function {
  pub name: String,
  pub params: Vec<Param>,
  pub ret: Option<Expr>,
  pub body: Vec<Stmt>,
}

#[derive(Debug)]
pub struct Param {
  pub name: String,
  pub typ: Expr,
}

#[derive(Debug)]
pub enum ExprKind {
  Ident(String),
}

#[derive(Debug)]
pub struct Expr {
  pub kind: ExprKind,
  pub pos: tokens::Pos,
}

#[derive(Debug)]
pub enum StmtKind {
  TODO, // TODO: Put actual kinds in here
}

#[derive(Debug)]
pub struct Stmt {
  pub kind: StmtKind,
  pub pos: tokens::Pos,
}

impl File {
  pub fn new(name: String) -> Self {
    Self {name, imports: Vec::new(), functions: HashMap::new()}
  }
}

#[derive(Error, Debug)]
pub enum ParseError {
  #[error("{0}: expected '{1}'")]
  ExpectedToken(tokens::Pos, tokens::TokenKind),
  #[error("{0}: expected token")]
  TokenOutOfRange(tokens::Pos),
  #[error("{0}: unexpected token: '{1}'")]
  UnexpectedToken(tokens::Pos, tokens::TokenKind),
}