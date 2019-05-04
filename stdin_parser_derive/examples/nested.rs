use stdin_parser::StdinParser;
use stdin_parser_derive::StdinParser;

#[derive(StdinParser, Debug)]
struct Inner {
    /// We are inside the Inner struct
    a: i32,
}

#[derive(StdinParser, Debug)]
struct Outer {
    /// We are inside the Outer struct
    a: Inner,
}

pub fn main() {
    let test = Outer::parse_stdin();
    println!("{:#?}", test);
}
