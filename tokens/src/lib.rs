mod typs;
mod errors;
pub use typs::*;
pub use errors::*;

struct StrIter {
  val: Vec<char>,
  ind: usize,
  line: usize,
  char: usize,
  file: String,
}

impl StrIter {
  pub fn new(src: String, file: String) -> Self {
    Self {
      val: src.chars().collect(),
      ind: 0,
      line: 0,
      char: 0,
      file: file,
    }
  }

  pub fn hasnext(&self) -> bool {
    return self.ind < self.val.len();
  }

  pub fn next(&mut self) -> char {
    let v = self.val[self.ind];
    self.ind += 1;
    if v == '\n' {
      self.line += 1;
      self.char = 0;
    }
    return v;
  }

  pub fn back(&mut self) {
    self.ind -= 1;
  } 

  pub fn pos(&self) -> Pos {
    Pos::new(&self.file, self.line, self.char)
  }
}

pub fn tokenize(file: String, src: String) -> Result<Vec<Token>, impl std::error::Error> {
  let mut out: Vec<Token> = Vec::new();
  let mut it = StrIter::new(src, file);
  while it.hasnext() {
    let c = it.next();
    match c {
      ' ' => (),
      '\t' => (),
      '\n' => (),
      ';' => out.push(Token::new(it.pos(), TokenKind::End)),
      _ => {
        if valid_ident_start(c) {
          // Add ident
          let start = it.pos();
          let mut val = c.to_string();
          while it.hasnext() {
            let c = it.next();
            if !valid_ident(c) {
              it.back();
              break;
            } else {
              val.push(c);
            }
          }
          out.push(Token::new(start.extend(it.pos()), TokenKind::Ident(val)));
        } else {
          return Err(TokenizeError::UnexpectedCharacter(it.pos(), c))
        }
      }
    }
  }
  Ok(out)
}

fn valid_ident_start(c: char) -> bool {
  match c {
    'A' => true,
    'B' => true,
    'C' => true,
    'D' => true,
    'E' => true,
    'F' => true,
    'G' => true,
    'H' => true,
    'I' => true,
    'J' => true,
    'K' => true,
    'L' => true,
    'M' => true,
    'N' => true,
    'O' => true,
    'P' => true,
    'Q' => true,
    'R' => true,
    'S' => true,
    'T' => true,
    'U' => true,
    'V' => true,
    'W' => true,
    'X' => true,
    'Y' => true,
    'Z' => true,
    'a' => true,
    'b' => true,
    'c' => true,
    'd' => true,
    'e' => true,
    'f' => true,
    'g' => true,
    'h' => true,
    'i' => true,
    'j' => true,
    'k' => true,
    'l' => true,
    'm' => true,
    'n' => true,
    'o' => true,
    'p' => true,
    'q' => true,
    'r' => true,
    's' => true,
    't' => true,
    'u' => true,
    'v' => true,
    'w' => true,
    'x' => true,
    'y' => true,
    'z' => true,
    _ => false
  }
}

fn valid_ident(c: char) -> bool {
  match c {
    'A' => true,
    'B' => true,
    'C' => true,
    'D' => true,
    'E' => true,
    'F' => true,
    'G' => true,
    'H' => true,
    'I' => true,
    'J' => true,
    'K' => true,
    'L' => true,
    'M' => true,
    'N' => true,
    'O' => true,
    'P' => true,
    'Q' => true,
    'R' => true,
    'S' => true,
    'T' => true,
    'U' => true,
    'V' => true,
    'W' => true,
    'X' => true,
    'Y' => true,
    'Z' => true,
    '0' => true,
    '1' => true,
    '2' => true,
    '3' => true,
    '4' => true,
    '5' => true,
    '6' => true,
    '7' => true,
    '8' => true,
    '9' => true,
    'a' => true,
    'b' => true,
    'c' => true,
    'd' => true,
    'e' => true,
    'f' => true,
    'g' => true,
    'h' => true,
    'i' => true,
    'j' => true,
    'k' => true,
    'l' => true,
    'm' => true,
    'n' => true,
    'o' => true,
    'p' => true,
    'q' => true,
    'r' => true,
    's' => true,
    't' => true,
    'u' => true,
    'v' => true,
    'w' => true,
    'x' => true,
    'y' => true,
    'z' => true,
    _ => false
  }
}