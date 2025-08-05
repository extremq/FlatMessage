use crate::*;
use flat_message::*;

#[test]
fn check_ipv4() {
    #[derive(Debug, PartialEq, Eq, FlatMessage)]
    #[flat_message_options(store_name: false)]
    struct Test {
        v1: Option<i32>,
        v2: Option<u32>,
    }
    validate_correct_serde(Test { v1: None, v2: None });
}
