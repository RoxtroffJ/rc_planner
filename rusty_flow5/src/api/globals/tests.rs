use crate::api::globals::{poplog, push_to_log};

#[test]
fn main() {
    push_to_log("This is".to_string());
    push_to_log("a".to_string());

    assert_eq!("This is", poplog());

    push_to_log("test".to_string());

    assert_eq!("a", poplog());
    assert_eq!("test", poplog());
}