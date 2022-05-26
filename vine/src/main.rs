use tokens;

const CODE: &str = "package main;";

fn main() {
    let res = tokens::tokenize("a.vine".to_string(), CODE.to_string()).expect("tokenize failure");
    println!("{res:?}");
}
