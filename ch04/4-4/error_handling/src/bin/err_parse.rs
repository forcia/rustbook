fn get_int() -> Result<i32, std::num::ParseIntError> {
    let num_str = "42";

    let ret = num_str.trim().parse::<i32>();
    match ret {
        Ok(t) => Ok(t * 2),
        Err(e) => Err(e),
    }

    // let ret = num_str.trim().parse::<i32>()?;
    // Ok(ret * 2)

    // num_str.trim().parse::<i32>().map(|t| t * 2)
}

fn main() {
    match get_int() {
        Ok(x) => println!("{}", x),
        Err(e) => println!("{}", e),
    }
}
