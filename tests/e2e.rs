use std::env;

#[test]
fn e2e() {
    let binary_path = env::args().nth(2).unwrap();
    assert_eq!(binary_path, "target/debug/hrh");
}
