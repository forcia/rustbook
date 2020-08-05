use thiserror::Error;

#[derive(Error, Debug)]
enum MyError {
    #[error("failed to read string from {0}")]
    ReadError(String),
    #[error(transparent)]
    ParseError(#[from] std::num::ParseIntError),
}

fn get_int_from_file() -> Result<i32, MyError> {
    let path = "number.txt";

    let num_str = std::fs::read_to_string(path).map_err(|_| MyError::ReadError(path.into()))?;

    num_str
        .trim()
        .parse::<i32>()
        .map(|t| t * 2)
        .map_err(MyError::from)
}

fn main() {
    match get_int_from_file() {
        Ok(x) => println!("{}", x),
        Err(e) => println!("{:#?}", e),
    }
}
