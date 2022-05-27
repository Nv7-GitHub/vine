use tokens;

mod typs;
pub use typs::*;

pub fn parse(name: String, tok: Vec<tokens::Token>) -> File {
  let out = File::new(name);
  
  return out;
}