use tokens::*;

mod typs;
pub use typs::*;

struct Tokens {
  tok: Vec<Token>,
  ind: usize,
}

impl Tokens {
  fn expect(&mut self, kind: TokenKind) -> Result<(), ParseError> {
    if self.ind >= self.tok.len() {
      let last = self.tok.last().unwrap();
      return Err(ParseError::ExpectedToken(Pos::start(last.pos.clone()), kind));
    }
    let val = &self.tok[self.ind];
    if val.kind != kind {
      return Err(ParseError::ExpectedToken(val.pos.clone(), kind));
    }
    self.ind += 1;
    Ok(())
  }

  fn next(&mut self) -> Result<Token, ParseError> {
    if self.ind >= self.tok.len() {
      let last = self.tok.last().unwrap();
      return Err(ParseError::TokenOutOfRange(Pos::start(last.pos.clone())));
    }
    self.ind += 1;
    Ok(self.tok[self.ind - 1].clone())
  }
}

pub fn parse(name: String, tokens: Vec<Token>) -> Result<File, ParseError> {
  let mut out = File::new(name);
  let mut tok = Tokens {tok: tokens, ind: 0};

  // Get package
  tok.expect(TokenKind::Ident("package".to_string()))?;
  let next = tok.next()?;
  if let TokenKind::Ident(name) = next.kind {
    out.package = name;
  } else {
    return Err(ParseError::ExpectedToken(next.pos, TokenKind::Ident("".to_string())));
  }

  Ok(out)
}