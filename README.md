# assert-json-diff

[![Build Status](https://travis-ci.org/davidpdrsn/assert-json-diff.svg?branch=master)](https://travis-ci.org/davidpdrsn/assert-json-diff)
[![Crates.io](https://img.shields.io/crates/v/assert-json-diff.svg)](https://crates.io/crates/assert-json-diff)
[![Documentation](https://docs.rs/assert-json-diff/badge.svg)](https://docs.rs/assert-json-diff/)

This crate includes an assert macro for comparing two JSON values. It is designed to give much
more helpful error messages than the standard [`assert_eq!`]. It basically does a diff of the
two objects and tells you the exact differences. This is useful when asserting that two large
JSON objects are the same.

It uses the [`serde_json::Value`] type to represent JSON.

[`serde_json::Value`]: https://docs.serde.rs/serde_json/value/enum.Value.html
[`assert_eq!`]: https://doc.rust-lang.org/std/macro.assert_eq.html

### Example

```rust
#[macro_use]
extern crate assert_json_diff;
#[macro_use]
extern crate serde_json;

// probably with #[test] attribute
fn some_test() {
    let a = json!({
        "data": {
            "users": [
                {
                    "id": 1,
                    "country": {
                        "name": "Denmark"
                    }
                },
                {
                    "id": 24,
                    "country": {
                        "name": "Denmark"
                    }
                }
            ]
        }
    });

    let b = json!({
        "data": {
            "users": [
                {
                    "id": 1,
                    "country": {
                        "name": "Sweden"
                    }
                },
                {
                    "id": 2,
                    "country": {
                        "name": "Denmark"
                    }
                }
            ]
        }
    });

    assert_json_eq!(actual: a, expected: b)
}
```

This will panic with the error message:

```
json atoms at path ".data.users[0].country.name" are not equal:
    expected:
        "Sweden"
    actual:
        "Denmark"

json atoms at path ".data.users[1].id" are not equal:
    expected:
        2
    actual:
        24
```

### Additional data

It allows extra data in `actual` but not in `expected`. That is so you can verify just a part
of the JSON without having to specify the whole thing. For example this test passes:

```rust
#[macro_use]
extern crate assert_json_diff;
#[macro_use]
extern crate serde_json;

// probably with #[test] attribute
fn some_test() {
    assert_json_eq!(
        actual: json!({
            "a": { "b": 1 },
        }),
        expected: json!({
            "a": {},
        })
    )
}
```

However `expected` cannot contain additional data so this test fails:

```rust
#[macro_use]
extern crate assert_json_diff;
#[macro_use]
extern crate serde_json;

// probably with #[test] attribute
fn some_test() {
    assert_json_eq!(
        actual: json!({
            "a": {},
        }),
        expected: json!({
            "a": { "b": 1 },
        })
    )
}
```

That will print

```
json atom at path ".a.b" is missing from expected
```

### The macro

The `assert_json_eq!` macro can be called with or without typing `actual:` and `expected:`

```rust
#[macro_use]
extern crate assert_json_diff;
#[macro_use]
extern crate serde_json;

// probably with #[test] attribute
fn some_test() {
    assert_json_eq!(
        actual: json!(true),
        expected: json!(true)
    )
}

// probably with #[test] attribute
fn some_other_test() {
    assert_json_eq!(
        json!(true),
        json!(true)
    )
}
```

The version that includes `actual:` and `expected:` is preferred because it makes it very clear
which is which and there [which can contain additional data](#additional-data)

License: MIT
