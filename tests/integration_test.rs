#[macro_use]
extern crate assert_json_diff;
#[macro_use]
extern crate serde_json;

#[test]
fn boolean_root() {
    let result = assert_json_eq_no_panic!(actual: json!(true), expected: json!(true));
    assert_output_eq(result, Ok(()));

    let result = assert_json_eq_no_panic!(actual: json!(false), expected: json!(false));
    assert_output_eq(result, Ok(()));

    let result = assert_json_eq_no_panic!(actual: json!(false), expected: json!(true));
    assert_output_eq(
        result,
        Err(r#"json atoms at path "(root)" are not equal:
    expected:
        true
    actual:
        false"#),
    );

    let result = assert_json_eq_no_panic!(actual: json!(true), expected: json!(false));
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
    let result = assert_json_eq_no_panic!(actual: json!("true"), expected: json!("true"));
    assert_output_eq(result, Ok(()));

    let result = assert_json_eq_no_panic!(actual: json!("false"), expected: json!("false"));
    assert_output_eq(result, Ok(()));

    let result = assert_json_eq_no_panic!(actual: json!("false"), expected: json!("true"));
    assert_output_eq(
        result,
        Err(r#"json atoms at path "(root)" are not equal:
    expected:
        "true"
    actual:
        "false""#),
    );

    let result = assert_json_eq_no_panic!(actual: json!("true"), expected: json!("false"));
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
    let result = assert_json_eq_no_panic!(actual: json!(1), expected: json!(1));
    assert_output_eq(result, Ok(()));

    let result = assert_json_eq_no_panic!(actual: json!(0), expected: json!(0));
    assert_output_eq(result, Ok(()));

    let result = assert_json_eq_no_panic!(actual: json!(0), expected: json!(1));
    assert_output_eq(
        result,
        Err(r#"json atoms at path "(root)" are not equal:
    expected:
        1
    actual:
        0"#),
    );

    let result = assert_json_eq_no_panic!(actual: json!(1), expected: json!(0));
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
    // can also be called without `expected:` and `actual:`
    let result = assert_json_eq_no_panic!(json!(null), json!(null));
    assert_output_eq(result, Ok(()));

    let result = assert_json_eq_no_panic!(actual: json!(null), expected: json!(1));
    assert_output_eq(
        result,
        Err(r#"json atoms at path "(root)" are not equal:
    expected:
        1
    actual:
        null"#),
    );

    let result = assert_json_eq_no_panic!(actual: json!(1), expected: json!(null));
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
    let result = assert_json_eq_no_panic!(json!({ "a": true }), json!({ "a": true }));
    assert_output_eq(result, Ok(()));

    let result = assert_json_eq_no_panic!(
        actual: json!({ "a": false }),
        expected: json!({ "a": true })
    );
    assert_output_eq(
        result,
        Err(r#"json atoms at path ".a" are not equal:
    expected:
        true
    actual:
        false"#),
    );

    let result = assert_json_eq_no_panic!(
        actual: json!({ "a": { "b": true } }),
        expected: json!({ "a": { "b": true } })
    );
    assert_output_eq(result, Ok(()));

    let result = assert_json_eq_no_panic!(
        actual: json!({ "a": true }),
        expected: json!({ "a": { "b": true } })
    );
    assert_output_eq(
        result,
        Err(r#"json atom at path ".a.b" is missing from expected"#),
    );

    let result = assert_json_eq_no_panic!(
        actual: json!({ "a": { "b": true } }),
        expected: json!({ "a": true })
    );
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
    let result = assert_json_eq_no_panic!(actual: json!([1]), expected: json!([1]));
    assert_output_eq(result, Ok(()));

    let result = assert_json_eq_no_panic!(actual: json!([2]), expected: json!([1]));
    assert_output_eq(
        result,
        Err(r#"json atoms at path "[0]" are not equal:
    expected:
        1
    actual:
        2"#),
    );

    let result = assert_json_eq_no_panic!(actual: json!([1, 2, 4]), expected: json!([1, 2, 3]));
    assert_output_eq(
        result,
        Err(r#"json atoms at path "[2]" are not equal:
    expected:
        3
    actual:
        4"#),
    );

    let result = assert_json_eq_no_panic!(
        actual: json!({ "a": [1, 2, 3]}),
        expected: json!({ "a": [1, 2, 4]})
    );
    assert_output_eq(
        result,
        Err(r#"json atoms at path ".a[2]" are not equal:
    expected:
        4
    actual:
        3"#),
    );

    let result = assert_json_eq_no_panic!(
        actual: json!({ "a": [1, 2, 3]}),
        expected: json!({ "a": [1, 2]})
    );
    assert_output_eq(result, Ok(()));

    let result = assert_json_eq_no_panic!(
        actual: json!({ "a": [1, 2]}),
        expected: json!({ "a": [1, 2, 3]})
    );
    assert_output_eq(
        result,
        Err(r#"json atom at path ".a[2]" is missing from expected"#),
    );
}

fn assert_output_eq(actual: Result<(), String>, expected: Result<(), &str>) {
    match (actual, expected) {
        (Ok(()), Ok(())) => return,

        (Err(actual_error), Ok(())) => {
            println!("Did not expect error, but got");
            println!("{}", actual_error);
        }

        (Ok(()), Err(expected_error)) => {
            let expected_error = expected_error.to_string();
            println!("Expected error, but did not get one. Expected error:");
            println!("{}", expected_error);
        }

        (Err(actual_error), Err(expected_error)) => {
            let expected_error = expected_error.to_string();
            if actual_error == expected_error {
                return;
            } else {
                println!("Errors didn't match");
                println!("Actual:");
                println!("{}", actual_error);
                println!("Expected:");
                println!("{}", expected_error);
            }
        }
    }

    panic!("assertion error, see stdout");
}
