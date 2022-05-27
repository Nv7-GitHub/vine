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

  past_lens: Vec<usize>,
}

impl StrIter {
  pub fn new(src: String, file: String) -> Self {
    Self {
      val: src.chars().collect(),
      ind: 0,
      line: 0,
      char: 0,
      file,
      past_lens: Vec::new(),
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
      self.past_lens.push(self.char);
      self.char = 0;
    } else {
      self.char += 1;
    }
    return v;
  }

  pub fn back(&mut self) {
    self.ind -= 1;
    if self.char == 0 {
      println!("{} {} {}", self.line, self.char, self.ind);
      self.line -= 1;
      self.char = self.past_lens.pop().unwrap();
    } else {
      self.char -= 1;
    }
  } 

  pub fn pos(&self) -> Pos {
    Pos::new(&self.file, self.line, self.char)
  }
}

pub fn tokenize(file: String, src: String) -> Result<Vec<Token>, TokenizeError> {
  let mut out: Vec<Token> = Vec::new();
  let mut it = StrIter::new(src, file);
  while it.hasnext() {
    let c = it.next();
    match c {
      ' ' => (),
      '\t' => (),
      '\n' => (),
      ';' => out.push(Token::new(it.pos(), TokenKind::End)),
      '(' => out.push(Token::new(it.pos(), TokenKind::LParen)),
      ')' => out.push(Token::new(it.pos(), TokenKind::RParen)),
      '{' => out.push(Token::new(it.pos(), TokenKind::LBrack)),
      '}' => out.push(Token::new(it.pos(), TokenKind::RBrack)),
      ',' => out.push(Token::new(it.pos(), TokenKind::Comma)),
      '.' => out.push(Token::new(it.pos(), TokenKind::Selector)),
      '+' => out.push(Token::new(it.pos(), TokenKind::Operator(Operator::Add))),
      '-' => out.push(Token::new(it.pos(), TokenKind::Operator(Operator::Subtract))),
      '*' => out.push(Token::new(it.pos(), TokenKind::Operator(Operator::Multiply))),
      '/' => out.push(Token::new(it.pos(), TokenKind::Operator(Operator::Divide))),
      '"' => {
        let start = it.pos();
        let mut val = String::new();
        'stringadd: while it.hasnext() {
          let c = it.next();
          match c {
            '"' => break 'stringadd,
            '\\' => {
              if it.hasnext() {
                let code = it.next();
                match code {
                  'n' => val.push('\n'),
                  't' => val.push('\t'),
                  '\\' => val.push('\\'),
                  '"' => val.push('"'),
                  _ => {it.back(); return Err(TokenizeError::InvalidEscapeCode(it.pos(), c))}
                }
              }
            },
            _ => val.push(c),
          }
        }
        out.push(Token::new(start.extend(it.pos()), TokenKind::String(val)));
      },
      '0' | '1' | '2' | '3' | '5' | '6' | '7' | '8' | '9' => {
        // Add ident
        let start = it.pos();
        let mut val = c.to_string();
        let mut has_period = false;
        while it.hasnext() {
          let c = it.next();

          if c == '.' && !has_period {
            has_period = true;
            val.push(c);
            continue;
          }

          if !valid_num(c) {
            it.back();
            break;
          } else {
            val.push(c);
          }
        }
        out.push(Token::new(start.extend(it.pos()), TokenKind::Number(val)));
      }
      '=' => {
        if it.hasnext() {
          let start = it.pos();
          let next = it.next();
          match next {
            '=' => {
              out.push(Token::new(start.extend(it.pos()), TokenKind::Operator(Operator::Equal)));
            },
            _ => {
              it.back();
              out.push(Token::new(it.pos(), TokenKind::Assign));
            }
          }
        }
      },
      '!' => {
        if it.hasnext() {
          let start = it.pos();
          let next = it.next();
          match next {
            '=' => {
              out.push(Token::new(start.extend(it.pos()), TokenKind::Operator(Operator::NotEqual)));
            },
            _ => {
              it.back();
              out.push(Token::new(it.pos(), TokenKind::Operator(Operator::Not)));
            }
          }
        }
      },
      '|' => {
        if it.hasnext() {
          let start = it.pos();
          let next = it.next();
          match next {
            '|' => {
              out.push(Token::new(start.extend(it.pos()), TokenKind::Operator(Operator::Or)));
            },
            _ => {
              it.back();
            }
          }
        }
      },
      '&' => {
        if it.hasnext() {
          let start = it.pos();
          let next = it.next();
          match next {
            '&' => {
              out.push(Token::new(start.extend(it.pos()), TokenKind::Operator(Operator::And)));
            },
            _ => {
              it.back();
            }
          }
        }
      },
      '>' => {
        if it.hasnext() {
          let start = it.pos();
          let next = it.next();
          match next {
            '=' => {
              out.push(Token::new(start.extend(it.pos()), TokenKind::Operator(Operator::GreaterEqual)));
            },
            _ => {
              it.back();
              out.push(Token::new(it.pos(), TokenKind::Operator(Operator::Greater)));
            }
          }
        }
      },
      '<' => {
        if it.hasnext() {
          let start = it.pos();
          let next = it.next();
          match next {
            '=' => {
              out.push(Token::new(start.extend(it.pos()), TokenKind::Operator(Operator::LessEqual)));
            },
            _ => {
              it.back();
              out.push(Token::new(it.pos(), TokenKind::Operator(Operator::Less)));
            }
          }
        }
      },
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
      },
    }
  }
  Ok(out)
}

fn valid_ident_start(c: char) -> bool {
  match c {
    'A' | 'B' | 'C' | 'D' | 'E' | 'F' | 'G' | 'H' | 'I' | 'J' | 'K' | 'L' | 'M' | 'N' | 'O' | 'P' | 'Q' | 'R' | 'S' | 'T' | 'U' | 'V' | 'W' | 'X' | 'Y' | 'Z' | 'a' | 'b' | 'c' | 'd' | 'e' | 'f' | 'g' | 'h' | 'i' | 'j' | 'k' | 'l' | 'm' | 'n' | 'o' | 'p' | 'q' | 'r' | 's' | 't' | 'u' | 'v' | 'w' | 'x' | 'y' | 'z' => true,
    _ => false
  }
}

fn valid_ident(c: char) -> bool {
  match c {
    'A' | 'B' | 'C' | 'D' | 'E' | 'F' | 'G' | 'H' | 'I' | 'J' | 'K' | 'L' | 'M' | 'N' | 'O' | 'P' | 'Q' | 'R' | 'S' | 'T' | 'U' | 'V' | 'W' | 'X' | 'Y' | 'Z' | 'a' | 'b' | 'c' | 'd' | 'e' | 'f' | 'g' | 'h' | 'i' | 'j' | 'k' | 'l' | 'm' | 'n' | 'o' | 'p' | 'q' | 'r' | 's' | 't' | 'u' | 'v' | 'w' | 'x' | 'y' | 'z' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '0' => true,
    _ => false
  }
}

fn valid_num(c: char) -> bool {
  match c {
    '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '0' => true,
    _ => false
  }
}