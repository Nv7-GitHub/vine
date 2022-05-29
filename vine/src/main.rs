use tokens;
use parse;

const CODE: &str = r#"import "a" as b;
import "b";

fn add(a int, b int) int {
    return a;
    //return a.add(a, b);
}

fn math(a int, b int) int {
    //if a != 0 {
    //    return 100;
    //}
    return a + b;
}

fn main() {
    //if (add(1, 2) == 3) {
    //    print("hi");
    //} else {
    //    print("no");
    //}
}
"#;

fn main() {
    let res = tokens::tokenize("a.vine".to_string(), CODE.to_string()).expect("tokenize failure");
    println!("Tokens:");
    for tok in res.iter() {
        println!("{}", tok);
    }
    println!("\nParsed:");
    let f = parse::parse("a.vine".to_string(), res).expect("parse failure");
    println!("{f:#?}");
}
