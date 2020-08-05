/// This function adds 2 numbers.
/// 
/// # Example
/// 
/// ```
/// use test_code::add;
/// 
/// add(1, 2);
/// ```
#[allow(dead_code)]
pub fn add(x:i32, y:i32) -> i32 {
    return x + y;
}

/// This is a test function for add function.
#[test]
fn test_add() {
    assert_eq!(0, add(0, 0));
    assert_eq!(1, add(0, 1));
    assert_eq!(1, add(1, 0));
    assert_eq!(2, add(1, 1));
}

/// This test is executed by `cargo test -- --ignored`
#[test]
#[ignore]
fn test_add_ignored() {
    assert_eq!(-2, add(-1, -1));
}
