enum MyError {
    Io(std::io::Error),
    Num(std::num::ParseIntError),
}

use std::fmt;
impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MyError::Io(e) => write!(f, "I/O Error: {}", e),
            MyError::Num(e) => write!(f, "Parse Error: {}", e),
        }
    }
}

impl From<std::io::Error> for MyError {
    fn from(cause: std::io::Error) -> Self {
        Self::Io(cause)
    }
}

fn get_int_from_file() -> Result<i32, MyError> {
    let path = "number.txt";
    //let num_str = std::fs::read_to_string(path).map_err(|e| MyError::Io(e))?;
    let num_str = std::fs::read_to_string(path).map_err(MyError::from)?;

    num_str
        .trim()
        .parse::<i32>()
        .map(|t| t * 2)
        .map_err(|e| MyError::Num(e))
}

fn main() {
    match get_int_from_file() {
        Ok(x) => println!("{}", x),
        Err(e) => println!("{}", e),
        // Err(e) => match e {
        //     MyError::Io(cause) => println!("I/O Error: {}", cause),
        //     MyError::Num(cause) => println!("Parse Error: {}", cause),
        // },
    }
}
