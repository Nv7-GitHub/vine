use tokens::*;

mod typs;
mod defs;
pub use typs::*;

pub struct Tokens {
  tok: Vec<Token>,
  ind: usize,
}

impl Tokens {
  fn expect(&mut self, kind: TokenKind) -> Result<Pos, ParseError> {
    if self.ind >= self.tok.len() {
      let last = self.tok.last().unwrap();
      return Err(ParseError::ExpectedToken(Pos::start(last.pos.clone()), kind));
    }
    let val = &self.tok[self.ind];
    if val.kind != kind {
      return Err(ParseError::ExpectedToken(val.pos.clone(), kind));
    }
    self.ind += 1;
    Ok(val.pos.clone())
  }

  fn next(&mut self) -> Result<Token, ParseError> {
    if self.ind >= self.tok.len() {
      let last = self.tok.last().unwrap();
      return Err(ParseError::TokenOutOfRange(Pos::start(last.pos.clone())));
    }
    let val = self.tok[self.ind].clone();
    self.ind += 1;
    Ok(val)
  }

  fn curr(&self) -> Token {
    self.tok[self.ind - 1].clone()
  }

  fn back(&mut self) {
    self.ind -= 1;
  }

  fn hasnext(&self) -> bool {
    self.ind < self.tok.len()
  }
}

pub fn parse(name: String, tokens: Vec<Token>) -> Result<File, ParseError> {
  let mut out = File::new(name);
  let mut tok = Tokens {tok: tokens, ind: 0};
  defs::parse_defs(&mut out, &mut tok)?;
  Ok(out)
}