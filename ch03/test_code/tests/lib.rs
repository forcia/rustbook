use test_code::add;

#[test]
fn integration_test() {
    assert_eq!(3, add(1, 2));
}
