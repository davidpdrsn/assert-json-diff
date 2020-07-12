//! This crate includes macros for comparing two serializable values by diffing their JSON
//! representations. It is designed to give much more helpful error messages than the standard
//! [`assert_eq!`]. It basically does a diff of the two objects and tells you the exact
//! differences. This is useful when asserting that two large JSON objects are the same.
//!
//! It uses the [serde] and [serde_json] to perform the serialization.
//!
//! [serde]: https://crates.io/crates/serde
//! [serde_json]: https://crates.io/crates/serde_json
//! [`assert_eq!`]: https://doc.rust-lang.org/std/macro.assert_eq.html
//!
//! ## Partial matching
//!
//! If you want to assert that one JSON value is "included" in another use
//! [`assert_json_include`](macro.assert_json_include.html):
//!
//! ```should_panic
//! use assert_json_diff::assert_json_include;
//! use serde_json::json;
//!
//! fn main() {
//!     let a = json!({
//!         "data": {
//!             "users": [
//!                 {
//!                     "id": 1,
//!                     "country": {
//!                         "name": "Denmark"
//!                     }
//!                 },
//!                 {
//!                     "id": 24,
//!                     "country": {
//!                         "name": "Denmark"
//!                     }
//!                 }
//!             ]
//!         }
//!     });
//!
//!     let b = json!({
//!         "data": {
//!             "users": [
//!                 {
//!                     "id": 1,
//!                     "country": {
//!                         "name": "Sweden"
//!                     }
//!                 },
//!                 {
//!                     "id": 2,
//!                     "country": {
//!                         "name": "Denmark"
//!                     }
//!                 }
//!             ]
//!         }
//!     });
//!
//!     assert_json_include!(actual: a, expected: b)
//! }
//! ```
//!
//! This will panic with the error message:
//!
//! ```text
//! json atoms at path ".data.users[0].country.name" are not equal:
//!     expected:
//!         "Sweden"
//!     actual:
//!         "Denmark"
//!
//! json atoms at path ".data.users[1].id" are not equal:
//!     expected:
//!         2
//!     actual:
//!         24
//! ```
//!
//! [`assert_json_include`](macro.assert_json_include.html) allows extra data in `actual` but not in `expected`. That is so you can verify just a part
//! of the JSON without having to specify the whole thing. For example this test passes:
//!
//! ```
//! use assert_json_diff::assert_json_include;
//! use serde_json::json;
//!
//! fn main() {
//!     assert_json_include!(
//!         actual: json!({
//!             "a": { "b": 1 },
//!         }),
//!         expected: json!({
//!             "a": {},
//!         })
//!     )
//! }
//! ```
//!
//! However `expected` cannot contain additional data so this test fails:
//!
//! ```should_panic
//! use assert_json_diff::assert_json_include;
//! use serde_json::json;
//!
//! fn main() {
//!     assert_json_include!(
//!         actual: json!({
//!             "a": {},
//!         }),
//!         expected: json!({
//!             "a": { "b": 1 },
//!         })
//!     )
//! }
//! ```
//!
//! That will print
//!
//! ```text
//! json atom at path ".a.b" is missing from actual
//! ```
//!
//! ## Exact matching
//!
//! If you want to ensure two JSON values are *exactly* the same, use [`assert_json_eq`](macro.assert_json_eq.html).
//!
//! ```rust,should_panic
//! use assert_json_diff::assert_json_eq;
//! use serde_json::json;
//!
//! fn main() {
//!     assert_json_eq!(
//!         json!({ "a": { "b": 1 } }),
//!         json!({ "a": {} })
//!     )
//! }
//! ```
//!
//! This will panic with the error message:
//!
//! ```text
//! json atom at path ".a.b" is missing from lhs
//! ```

#![deny(
    missing_docs,
    unused_imports,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_import_braces,
    unused_qualifications,
    unknown_lints
)]
#![doc(html_root_url = "https://docs.rs/assert-json-diff/1.1.0")]

use diff::{diff, Mode};
use serde::Serialize;

mod core_ext;
mod diff;

/// The macro used to compare two JSON values for an inclusive match.
///
/// It allows `actual` to contain additional data. If you want an exact match use
/// [`assert_json_eq`](macro.assert_json_eq.html) instead.
///
/// See [crate documentation](index.html) for examples.
#[macro_export]
macro_rules! assert_json_include {
    (actual: $actual:expr, expected: $expected:expr) => {{
        let actual = $actual;
        let expected = $expected;
        if let Err(error) = $crate::assert_json_include_no_panic(&actual, &expected) {
            panic!("\n\n{}\n\n", error);
        }
    }};
    (actual: $actual:expr, expected: $expected:expr,) => {{
        $crate::assert_json_include!(actual: $actual, expected: $expected)
    }};
    (expected: $expected:expr, actual: $actual:expr) => {{
        $crate::assert_json_include!(actual: $actual, expected: $expected)
    }};
    (expected: $expected:expr, actual: $actual:expr,) => {{
        $crate::assert_json_include!(actual: $actual, expected: $expected)
    }};
}

/// The macro used to compare two JSON values for an exact match.
///
/// If you want an inclusive match use [`assert_json_include`](macro.assert_json_include.html) instead.
///
/// See [crate documentation](index.html) for examples.
#[macro_export]
macro_rules! assert_json_eq {
    ($lhs:expr, $rhs:expr) => {{
        let lhs = $lhs;
        let rhs = $rhs;
        if let Err(error) = $crate::assert_json_eq_no_panic(&lhs, &rhs) {
            panic!("\n\n{}\n\n", error);
        }
    }};
    ($lhs:expr, $rhs:expr,) => {{
        $crate::assert_json_eq!($lhs, $rhs)
    }};
}

/// Does the same as [`assert_json_include!`](macro.assert_json_include.html) but doesn't panic.
///
/// Instead it returns a `Result` where the error is the message that would be passed to `panic!`.
/// This is might be useful if you want to control how failures are reported and don't want to deal
/// with panics.
pub fn assert_json_include_no_panic<Actual, Expected>(
    actual: &Actual,
    expected: &Expected,
) -> Result<(), String>
where
    Actual: Serialize,
    Expected: Serialize,
{
    assert_json_no_panic(actual, expected, Mode::Lenient)
}

/// Does the same as [`assert_json_eq!`](macro.assert_json_eq.html) but doesn't panic.
///
/// Instead it returns a `Result` where the error is the message that would be passed to `panic!`.
/// This is might be useful if you want to control how failures are reported and don't want to deal
/// with panics.
pub fn assert_json_eq_no_panic<Lhs, Rhs>(lhs: &Lhs, rhs: &Rhs) -> Result<(), String>
where
    Lhs: Serialize,
    Rhs: Serialize,
{
    assert_json_no_panic(lhs, rhs, Mode::Strict)
}

fn assert_json_no_panic<Lhs, Rhs>(lhs: &Lhs, rhs: &Rhs, mode: Mode) -> Result<(), String>
where
    Lhs: Serialize,
    Rhs: Serialize,
{
    let lhs = serde_json::to_value(lhs).unwrap_or_else(|err| {
        panic!(
            "Couldn't convert left hand side value to JSON. Serde error: {}",
            err
        )
    });
    let rhs = serde_json::to_value(rhs).unwrap_or_else(|err| {
        panic!(
            "Couldn't convert right hand side value to JSON. Serde error: {}",
            err
        )
    });

    let diffs = diff(&lhs, &rhs, mode);

    if diffs.is_empty() {
        Ok(())
    } else {
        let msg = diffs
            .into_iter()
            .map(|d| d.to_string())
            .collect::<Vec<_>>()
            .join("\n\n");
        Err(msg)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{json, Value};
    use std::fmt::Write;

    #[test]
    fn boolean_root() {
        let result = test_partial_match(json!(true), json!(true));
        assert_output_eq(result, Ok(()));

        let result = test_partial_match(json!(false), json!(false));
        assert_output_eq(result, Ok(()));

        let result = test_partial_match(json!(false), json!(true));
        assert_output_eq(
            result,
            Err(r#"json atoms at path "(root)" are not equal:
    expected:
        true
    actual:
        false"#),
        );

        let result = test_partial_match(json!(true), json!(false));
        assert_output_eq(
            result,
            Err(r#"json atoms at path "(root)" are not equal:
    expected:
        false
    actual:
        true"#),
        );
    }

    #[test]
    fn string_root() {
        let result = test_partial_match(json!("true"), json!("true"));
        assert_output_eq(result, Ok(()));

        let result = test_partial_match(json!("false"), json!("false"));
        assert_output_eq(result, Ok(()));

        let result = test_partial_match(json!("false"), json!("true"));
        assert_output_eq(
            result,
            Err(r#"json atoms at path "(root)" are not equal:
    expected:
        "true"
    actual:
        "false""#),
        );

        let result = test_partial_match(json!("true"), json!("false"));
        assert_output_eq(
            result,
            Err(r#"json atoms at path "(root)" are not equal:
    expected:
        "false"
    actual:
        "true""#),
        );
    }

    #[test]
    fn number_root() {
        let result = test_partial_match(json!(1), json!(1));
        assert_output_eq(result, Ok(()));

        let result = test_partial_match(json!(0), json!(0));
        assert_output_eq(result, Ok(()));

        let result = test_partial_match(json!(0), json!(1));
        assert_output_eq(
            result,
            Err(r#"json atoms at path "(root)" are not equal:
    expected:
        1
    actual:
        0"#),
        );

        let result = test_partial_match(json!(1), json!(0));
        assert_output_eq(
            result,
            Err(r#"json atoms at path "(root)" are not equal:
    expected:
        0
    actual:
        1"#),
        );
    }

    #[test]
    fn null_root() {
        let result = test_partial_match(json!(null), json!(null));
        assert_output_eq(result, Ok(()));

        let result = test_partial_match(json!(null), json!(1));
        assert_output_eq(
            result,
            Err(r#"json atoms at path "(root)" are not equal:
    expected:
        1
    actual:
        null"#),
        );

        let result = test_partial_match(json!(1), json!(null));
        assert_output_eq(
            result,
            Err(r#"json atoms at path "(root)" are not equal:
    expected:
        null
    actual:
        1"#),
        );
    }

    #[test]
    fn into_object() {
        let result = test_partial_match(json!({ "a": true }), json!({ "a": true }));
        assert_output_eq(result, Ok(()));

        let result = test_partial_match(json!({ "a": false }), json!({ "a": true }));
        assert_output_eq(
            result,
            Err(r#"json atoms at path ".a" are not equal:
    expected:
        true
    actual:
        false"#),
        );

        let result =
            test_partial_match(json!({ "a": { "b": true } }), json!({ "a": { "b": true } }));
        assert_output_eq(result, Ok(()));

        let result = test_partial_match(json!({ "a": true }), json!({ "a": { "b": true } }));
        assert_output_eq(
            result,
            Err(r#"json atoms at path ".a" are not equal:
    expected:
        {
          "b": true
        }
    actual:
        true"#),
        );

        let result = test_partial_match(json!({}), json!({ "a": true }));
        assert_output_eq(
            result,
            Err(r#"json atom at path ".a" is missing from actual"#),
        );

        let result = test_partial_match(json!({ "a": { "b": true } }), json!({ "a": true }));
        assert_output_eq(
            result,
            Err(r#"json atoms at path ".a" are not equal:
    expected:
        true
    actual:
        {
          "b": true
        }"#),
        );
    }

    #[test]
    fn into_array() {
        let result = test_partial_match(json!([1]), json!([1]));
        assert_output_eq(result, Ok(()));

        let result = test_partial_match(json!([2]), json!([1]));
        assert_output_eq(
            result,
            Err(r#"json atoms at path "[0]" are not equal:
    expected:
        1
    actual:
        2"#),
        );

        let result = test_partial_match(json!([1, 2, 4]), json!([1, 2, 3]));
        assert_output_eq(
            result,
            Err(r#"json atoms at path "[2]" are not equal:
    expected:
        3
    actual:
        4"#),
        );

        let result = test_partial_match(json!({ "a": [1, 2, 3]}), json!({ "a": [1, 2, 4]}));
        assert_output_eq(
            result,
            Err(r#"json atoms at path ".a[2]" are not equal:
    expected:
        4
    actual:
        3"#),
        );

        let result = test_partial_match(json!({ "a": [1, 2, 3]}), json!({ "a": [1, 2]}));
        assert_output_eq(result, Ok(()));

        let result = test_partial_match(json!({ "a": [1, 2]}), json!({ "a": [1, 2, 3]}));
        assert_output_eq(
            result,
            Err(r#"json atom at path ".a[2]" is missing from actual"#),
        );
    }

    #[test]
    fn exact_matching() {
        let result = test_exact_match(json!(true), json!(true));
        assert_output_eq(result, Ok(()));

        let result = test_exact_match(json!("s"), json!("s"));
        assert_output_eq(result, Ok(()));

        let result = test_exact_match(json!("a"), json!("b"));
        assert_output_eq(
            result,
            Err(r#"json atoms at path "(root)" are not equal:
    lhs:
        "a"
    rhs:
        "b""#),
        );

        let result = test_exact_match(
            json!({ "a": [1, { "b": 2 }] }),
            json!({ "a": [1, { "b": 3 }] }),
        );
        assert_output_eq(
            result,
            Err(r#"json atoms at path ".a[1].b" are not equal:
    lhs:
        2
    rhs:
        3"#),
        );
    }

    #[test]
    fn exact_match_output_message() {
        let result = test_exact_match(json!({ "a": { "b": 1 } }), json!({ "a": {} }));
        assert_output_eq(
            result,
            Err(r#"json atom at path ".a.b" is missing from rhs"#),
        );

        let result = test_exact_match(json!({ "a": {} }), json!({ "a": { "b": 1 } }));
        assert_output_eq(
            result,
            Err(r#"json atom at path ".a.b" is missing from lhs"#),
        );
    }

    fn assert_output_eq(actual: Result<(), String>, expected: Result<(), &str>) {
        match (actual, expected) {
            (Ok(()), Ok(())) => {}

            (Err(actual_error), Ok(())) => {
                let mut f = String::new();
                writeln!(f, "Did not expect error, but got").unwrap();
                writeln!(f, "{}", actual_error).unwrap();
                panic!("{}", f);
            }

            (Ok(()), Err(expected_error)) => {
                let expected_error = expected_error.to_string();
                let mut f = String::new();
                writeln!(f, "Expected error, but did not get one. Expected error:").unwrap();
                writeln!(f, "{}", expected_error).unwrap();
                panic!("{}", f);
            }

            (Err(actual_error), Err(expected_error)) => {
                let expected_error = expected_error.to_string();
                if actual_error != expected_error {
                    let mut f = String::new();
                    writeln!(f, "Errors didn't match").unwrap();
                    writeln!(f, "Expected:").unwrap();
                    writeln!(f, "{}", expected_error).unwrap();
                    writeln!(f, "Got:").unwrap();
                    writeln!(f, "{}", actual_error).unwrap();
                    panic!("{}", f);
                }
            }
        }
    }

    fn test_partial_match(lhs: Value, rhs: Value) -> Result<(), String> {
        assert_json_no_panic(&lhs, &rhs, Mode::Lenient)
    }

    fn test_exact_match(lhs: Value, rhs: Value) -> Result<(), String> {
        assert_json_no_panic(&lhs, &rhs, Mode::Strict)
    }
}
