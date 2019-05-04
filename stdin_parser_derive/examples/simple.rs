use stdin_parser::StdinParser;
use stdin_parser_derive::StdinParser;

#[derive(StdinParser, Debug)]
struct SimpleStruct {
    /// a is i32
    /// sdfsdfs
    /// /sdfsdfsdfsd
    // aasdasddas
    a: i32,
    /// b is f64
    b: i64,
    /// c is f32
    c: f32,
    /// d is String
    d: String,
    e: bool,
}

pub fn main() {
    let test = SimpleStruct::parse_stdin();
    println!("{:#?}", test);
}
