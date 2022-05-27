use tokens;
use parse;

const CODE: &str = r#"package main;

import "a";

fn add(a int, b int) int {
    return a.add(a, b);
}

fn math(a int, b int) int {
    if a != 0 {
        return 100;
    }
    return a + b;
}

fn main() {
    if (add(1, 2) == 3) {
        print("hi");
    } else {
        print("no");
    }
}
"#;

fn main() {
    let res = tokens::tokenize("a.vine".to_string(), CODE.to_string()).expect("tokenize failure");
    let f = parse::parse("a.vine".to_string(), res);
    println!("{f:?}");
}
