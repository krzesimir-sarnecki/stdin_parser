// TODO: rename
pub trait StdinParser: Sized {
    fn parse_stdin() -> std::io::Result<Self>;
}

fn parse_stdin_line<T: std::str::FromStr>() -> std::io::Result<Result<T, T::Err>> {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer)?;
    let _ = buffer.pop(); // remove \n
    Ok(buffer.parse())
}

impl<T: std::str::FromStr> StdinParser for T
where
    T::Err: std::fmt::Display,
{
    fn parse_stdin() -> std::io::Result<Self> {
        let mut parsed = parse_stdin_line::<Self>()?;

        loop {
            match parsed {
                Ok(value) => return Ok(value),
                Err(err) => println!("{}, try again", err.to_string()),
            }

            parsed = parse_stdin_line::<Self>()?;
        }
    }
}
