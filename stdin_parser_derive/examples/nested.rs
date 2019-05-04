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

#[derive(StdinParser, Debug)]
enum OuterEnum {
    /// We are inside the Outer enum (variant A)
    A(Outer),
    /// We are inside the Outer enum (variant B)
    B(Inner),
    /// We are inside the Outer enum (variant C)
    C,
}

pub fn main() {
    let test = Outer::parse_stdin();
    println!("{:#?}", test);
    let test = OuterEnum::parse_stdin();
    println!("{:#?}", test);
}
