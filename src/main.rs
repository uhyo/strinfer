mod parser;

fn main() {
    let code = r#"
let digits = "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9";
"#;
    let ast = parser::parse(code);
    println!("{:?}", ast);
}
