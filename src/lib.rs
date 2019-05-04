// TODO: rename
pub trait StdinParser {
    fn parse_stdin() -> Self;
}

fn parse_stdin_line<T: std::str::FromStr>() -> Result<T, T::Err> {
    let mut buffer = String::new();
    // TODO: remove this unwrap
    std::io::stdin().read_line(&mut buffer).unwrap();
    let _ = buffer.pop(); // remove \n
    buffer.parse()
}

impl<T: std::str::FromStr> StdinParser for T
where
    T::Err: std::fmt::Debug,
{
    fn parse_stdin() -> Self {
        // TODO: remove this unwrap
        loop {
            let parsed = parse_stdin_line();

            if let Ok(value) = parsed {
                return value;
            }
        }
    }
}
