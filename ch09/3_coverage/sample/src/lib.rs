pub fn abs(x: i32) -> u32 {
    if x > 0 {
        return x as u32;
    } else {
        return -x as u32;
    }
}

#[cfg_attr(tarpaulin, skip)]
pub fn hello() {
    dbg!("hello");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_abs() {
        assert_eq!(abs(2), 2);
    }

    #[test]
    fn test_abs_neg() {
        assert_eq!(abs(-2), 2);
    }
}
