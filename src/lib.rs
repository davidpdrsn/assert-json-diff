//! This crate includes an assert macro for comparing two JSON values. It is designed to give much
//! more helpful error messages than the standard [`assert_eq!`]. It basically does a diff of the
//! two objects and tells you the exact differences. This is useful when asserting that two large
//! JSON objects are the same.
//!
//! It uses the [`serde_json::Value`] type to represent JSON.
//!
//! [`serde_json::Value`]: https://docs.serde.rs/serde_json/value/enum.Value.html
//! [`assert_eq!`]: https://doc.rust-lang.org/std/macro.assert_eq.html
//!
//! ## Example
//!
//! ```should_panic
//! #[macro_use]
//! extern crate assert_json_diff;
//! #[macro_use]
//! extern crate serde_json;
//! # fn main() { some_test() }
//!
//! // probably with #[test] attribute
//! fn some_test() {
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
//!     assert_json_eq!(actual: a, expected: b)
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
//! ## Additional data
//!
//! It allows extra data in `actual` but not in `expected`. That is so you can verify just a part
//! of the JSON without having to specify the whole thing. For example this test passes:
//!
//! ```
//! #[macro_use]
//! extern crate assert_json_diff;
//! #[macro_use]
//! extern crate serde_json;
//! # fn main() { some_test() }
//!
//! // probably with #[test] attribute
//! fn some_test() {
//!     assert_json_eq!(
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
//! #[macro_use]
//! extern crate assert_json_diff;
//! #[macro_use]
//! extern crate serde_json;
//! # fn main() { some_test() }
//!
//! // probably with #[test] attribute
//! fn some_test() {
//!     assert_json_eq!(
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
//! json atom at path ".a.b" is missing from expected
//! ```
//!
//! ## The macro
//!
//! The `assert_json_eq!` macro can be called with or without typing `actual:` and `expected:`
//!
//! ```
//! #[macro_use]
//! extern crate assert_json_diff;
//! #[macro_use]
//! extern crate serde_json;
//! # fn main() { some_test() }
//!
//! // probably with #[test] attribute
//! fn some_test() {
//!     assert_json_eq!(
//!         actual: json!(true),
//!         expected: json!(true)
//!     )
//! }
//!
//! // probably with #[test] attribute
//! fn some_other_test() {
//!     assert_json_eq!(
//!         json!(true),
//!         json!(true)
//!     )
//! }
//! ```
//!
//! The version that includes `actual:` and `expected:` is preferred because it makes it very clear
//! which is which and there [which can contain additional data](#additional-data)

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
    unused_qualifications
)]
#![doc(html_root_url = "https://docs.rs/assert-json-diff/0.1.0")]

extern crate serde;
extern crate serde_json;

use serde::{Serialize, Serializer};
use serde_json::Value;
use std::default::Default;
use std::fmt;

mod core_ext;
use core_ext::Indent;

/// The macro used to compare two JSON values.
///
/// See [crate documentation](index.html) for examples.
#[macro_export]
macro_rules! assert_json_eq {
    (actual: $actual:expr, expected: $expected:expr) => {{
        use $crate::{Actual, Expected};
        $crate::assert_json_eq(Actual($actual), Expected($expected))
    }};
    ($actual:expr, $expected:expr) => {{
        use $crate::{Actual, Expected};
        $crate::assert_json_eq(Actual($actual), Expected($expected))
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! assert_json_eq_no_panic {
    (actual: $actual:expr, expected: $expected:expr) => {{
        use $crate::{Actual, Expected};
        $crate::assert_json_eq_no_panic(Actual($actual), Expected($expected))
    }};
    ($actual:expr, $expected:expr) => {{
        use $crate::{Actual, Expected};
        $crate::assert_json_eq_no_panic(Actual($actual), Expected($expected))
    }};
}

#[doc(hidden)]
#[derive(Clone, Debug)]
pub struct Actual(pub Value);

impl std::ops::Deref for Actual {
    type Target = Value;
    fn deref(&self) -> &Value {
        &self.0
    }
}

impl Serialize for Actual {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        <Value>::serialize(self, serializer)
    }
}

impl From<Value> for Actual {
    fn from(v: Value) -> Actual {
        Actual(v)
    }
}

#[doc(hidden)]
#[derive(Clone, Debug)]
pub struct Expected(pub Value);

impl std::ops::Deref for Expected {
    type Target = Value;
    fn deref(&self) -> &Value {
        &self.0
    }
}

impl Serialize for Expected {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        <Value>::serialize(self, serializer)
    }
}

impl From<Value> for Expected {
    fn from(v: Value) -> Expected {
        Expected(v)
    }
}

#[doc(hidden)]
pub fn assert_json_eq(actual: Actual, expected: Expected) {
    if let Err(error) = assert_json_eq_no_panic(actual, expected) {
        panic!("\n\n{}\n\n", error.indent(4));
    }
}

#[doc(hidden)]
pub fn assert_json_eq_no_panic(actual: Actual, expected: Expected) -> Result<(), String> {
    let mut errors = MatchErrors::default();
    match_at_path(actual, expected, Path::Root, &mut errors);
    errors.to_output()
}

fn match_at_path(actual: Actual, expected: Expected, path: Path, errors: &mut MatchErrors) {
    if let Some(expected) = expected.as_object() {
        let keys = expected.keys();
        match_with_keys(keys, &actual, expected, path, errors);
    } else if let Some(expected) = expected.as_array() {
        let keys = if expected.is_empty() {
            vec![]
        } else {
            (0..=expected.len() - 1).collect::<Vec<_>>()
        };

        match_with_keys(keys.iter(), &actual, expected, path, errors);
    } else {
        if expected.0 != actual.0 {
            errors.push(ErrorType::NotEq(actual.clone(), expected.clone(), path));
        }
    }
}

fn match_with_keys<
    Key: Copy,
    Keys: Iterator<Item = Key>,
    Path: Dot<Key>,
    ActualCollection: Collection<Key, Item = ActualValue>,
    ActualValue: Clone + Into<Actual>,
    ExpectedCollection: Collection<Key, Item = ExpectedValue>,
    ExpectedValue: Clone + Into<Expected>,
>(
    keys: Keys,
    actual: &ActualCollection,
    expected: &ExpectedCollection,
    path: Path,
    errors: &mut MatchErrors,
) {
    for key in keys {
        match (expected.get(key), actual.get(key)) {
            (Some(expected), Some(actual)) => {
                match_at_path(
                    actual.clone().into(),
                    expected.clone().into(),
                    path.dot(key),
                    errors,
                );
            }

            (Some(_), None) => {
                errors.push(ErrorType::MissingPath(path.dot(key)));
            }

            (None, _) => unreachable!(),
        }
    }
}

trait Collection<Idx> {
    type Item;
    fn get(&self, index: Idx) -> Option<&Self::Item>;
}

impl<'a> Collection<&'a String> for serde_json::Map<String, Value> {
    type Item = Value;

    fn get(&self, index: &'a String) -> Option<&Self::Item> {
        self.get(index)
    }
}

impl<'a> Collection<&'a usize> for Vec<Value> {
    type Item = Value;

    fn get(&self, index: &'a usize) -> Option<&Self::Item> {
        <[Value]>::get(self, index.clone())
    }
}

impl<'a> Collection<&'a String> for Actual {
    type Item = Value;

    fn get(&self, index: &'a String) -> Option<&Self::Item> {
        <Value>::get(self, index.clone())
    }
}

impl<'a> Collection<&'a usize> for Actual {
    type Item = Value;

    fn get(&self, index: &'a usize) -> Option<&Self::Item> {
        <Value>::get(self, index.clone())
    }
}

#[derive(Clone)]
enum Path {
    Root,
    Trail(Vec<PathComp>),
}

impl fmt::Display for Path {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Path::Root => write!(f, "(root)"),
            Path::Trail(trail) => write!(
                f,
                "{}",
                trail
                    .iter()
                    .map(|comp| comp.to_string())
                    .collect::<Vec<_>>()
                    .join("")
            ),
        }
    }
}

impl Path {
    fn extend(&self, next: PathComp) -> Path {
        match self {
            Path::Root => Path::Trail(vec![next]),
            Path::Trail(trail) => {
                let mut trail = trail.clone();
                trail.push(next);
                Path::Trail(trail)
            }
        }
    }
}

trait Dot<T> {
    fn dot(&self, next: T) -> Path;
}

impl<'a> Dot<&'a String> for Path {
    fn dot(&self, next: &'a String) -> Path {
        let comp = PathComp::String(next.to_string());
        self.extend(comp)
    }
}

impl<'a> Dot<&'a str> for Path {
    fn dot(&self, next: &'a str) -> Path {
        let comp = PathComp::String(next.to_string());
        self.extend(comp)
    }
}

impl Dot<usize> for Path {
    fn dot(&self, next: usize) -> Path {
        let comp = PathComp::Index(next);
        self.extend(comp)
    }
}

impl<'a> Dot<&'a usize> for Path {
    fn dot(&self, next: &'a usize) -> Path {
        let comp = PathComp::Index(next.clone());
        self.extend(comp)
    }
}

#[derive(Clone)]
enum PathComp {
    String(String),
    Index(usize),
}

impl fmt::Display for PathComp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PathComp::String(s) => write!(f, ".{}", s),
            PathComp::Index(i) => write!(f, "[{}]", i),
        }
    }
}

struct MatchErrors {
    errors: Vec<ErrorType>,
}

impl Default for MatchErrors {
    fn default() -> Self {
        MatchErrors { errors: vec![] }
    }
}

impl MatchErrors {
    fn to_output(self) -> Result<(), String> {
        if self.errors.is_empty() {
            Ok(())
        } else {
            let messages = self
                .errors
                .iter()
                .map(|error| match error {
                    ErrorType::NotEq(actual, expected, path) => format!(
                        r#"json atoms at path "{}" are not equal:
    expected:
{}
    actual:
{}"#,
                        path,
                        serde_json::to_string_pretty(expected)
                            .expect("failed to pretty print JSON")
                            .indent(8),
                        serde_json::to_string_pretty(actual)
                            .expect("failed to pretty print JSON")
                            .indent(8),
                    ),
                    ErrorType::MissingPath(path) => {
                        format!(r#"json atom at path "{}" is missing from expected"#, path)
                    }
                })
                .collect::<Vec<_>>();
            Err(messages.join("\n\n"))
        }
    }

    fn push(&mut self, error: ErrorType) {
        self.errors.push(error);
    }
}

enum ErrorType {
    NotEq(Actual, Expected, Path),
    MissingPath(Path),
}
