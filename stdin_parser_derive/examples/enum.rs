use stdin_parser::StdinParser;
use stdin_parser_derive::StdinParser;

#[derive(StdinParser, Debug)]
enum SimpleEnum {
    /// variant A line1
    /// variant A line 2
    A(i32),
    /// variant B
    B,
    /// variant C
    C,
}

pub fn main() {
    let test = SimpleEnum::parse_stdin();
    println!("{:#?}", test);
}
