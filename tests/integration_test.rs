#[macro_use]
extern crate assert_json_diff;
#[macro_use]
extern crate serde_json;

use assert_json_diff::{assert_json_eq_no_panic, assert_json_include_no_panic};

#[test]
fn can_pass() {
    assert_json_include!(
        actual: json!({ "a": { "b": true }, "c": [true, null, 1] }),
        expected: json!({ "a": { "b": true }, "c": [true, null, 1] })
    );

    assert_json_include!(
        actual: json!({ "a": { "b": true } }),
        expected: json!({ "a": {} })
    );

    assert_json_include!(
        actual: json!({ "a": { "b": true } }),
        expected: json!({ "a": {} }),
    );

    assert_json_include!(
        expected: json!({ "a": {} }),
        actual: json!({ "a": { "b": true } }),
    );
}

#[test]
#[should_panic]
fn can_fail() {
    assert_json_include!(
        actual: json!({ "a": { "b": true }, "c": [true, null, 1] }),
        expected: json!({ "a": { "b": false }, "c": [false, null, {}] })
    );
}

#[test]
fn can_pass_with_exact_match() {
    assert_json_eq!(json!({ "a": { "b": true } }), json!({ "a": { "b": true } }));
    assert_json_eq!(json!({ "a": { "b": true } }), json!({ "a": { "b": true } }),);
}

#[test]
#[should_panic]
fn can_fail_with_exact_match() {
    assert_json_eq!(json!({ "a": { "b": true } }), json!({ "a": {} }));
}

#[test]
fn inclusive_match_without_panicing() {
    assert!(assert_json_include_no_panic(&json!({ "a": 1, "b": 2 }), &json!({ "b": 2})).is_ok());

    assert!(assert_json_include_no_panic(&json!({ "a": 1, "b": 2 }), &json!("foo")).is_err());
}

#[test]
fn exact_match_without_panicing() {
    assert!(assert_json_eq_no_panic(&json!([1, 2, 3]), &json!([1, 2, 3])).is_ok());

    assert!(assert_json_eq_no_panic(&json!([1, 2, 3]), &json!("foo")).is_err());
}
