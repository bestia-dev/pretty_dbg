// tests/integration_test.rs

use anyhow::Result;
use pretty_dbg::*;
use serde_json;

/// How to test functions that use println!()
/// https://users.rust-lang.org/t/how-to-test-functions-that-use-println/67188/5
/// this macro will redirect the stderr and capture it
/// the test must be run with the --nocapture option like
/// cargo test -- --nocapture
/// preferably use cargo auto test and write this option inside the automation task
#[macro_export]
macro_rules! assert_stderr_eq {
    ($test:expr, $expected:literal) => {{
        use gag::BufferRedirect;
        use std::io::Read;

        let mut buf = BufferRedirect::stderr().unwrap();

        $test;

        let mut output = String::new();
        buf.read_to_string(&mut output).unwrap();
        drop(buf);

        assert_eq!(&output[..], $expected);
    }};
}

/// check if macro assert_stderr_eq works
#[test]
fn test_00_check_if_macro_assert_stderr_eq_works() {
    // check if the macro works as expected
    assert_stderr_eq!(eprintln!("xxx"), "xxx\n");
}

/// not pretty dbg! for multiline string
#[test]
fn test_01_macro_dbg_1() {
    fn macro_dbg_1() {
        let json_str = r#"{
    owner: 'bestia-dev',
    repository_details: {
        general: {
        description: 'testing the creation of a github repo',
        },
    },
}"#;
        dbg!(json_str);
    }
    assert_stderr_eq!(macro_dbg_1(),
"[tests/integration_test.rs:50:9] json_str = \"{\\n    owner: 'bestia-dev',\\n    repository_details: {\\n        general: {\\n        description: 'testing the creation of a github repo',\\n        },\\n    },\\n}\"\n");
}

/// not pretty dbg! for serde_json::Value
#[test]
fn test_02_macro_dbg_2() {
    fn macro_dbg_2() -> Result<(), anyhow::Error> {
        let response_text = r#"{
     "id": 1296269,
     "homepage": "https://github.com"
 }"#;
        let parsed_json_value: serde_json::Value = serde_json::from_str(response_text)?;
        dbg!(&parsed_json_value);

        Ok(())
    }
    assert_stderr_eq!(macro_dbg_2().unwrap(),
"[tests/integration_test.rs:65:9] &parsed_json_value = Object {\n    \"homepage\": String(\"https://github.com\"),\n    \"id\": Number(1296269),\n}\n");
}

/// pretty_dbg! for multiline string
#[test]
fn test_03_macro_pretty_dbg_1() {
    fn macro_pretty_dbg_1() {
        let json_str = r#"{
    owner: 'bestia-dev',
    repository_details: {
    general: {
        description: 'testing the creation of a github repo',
        },
    },
}"#;
        pretty_dbg!(json_str);
    }
    assert_stderr_eq!(
        macro_pretty_dbg_1(),
        r#"[tests/integration_test.rs:85:9] json_str = {
    owner: 'bestia-dev',
    repository_details: {
    general: {
        description: 'testing the creation of a github repo',
        },
    },
}
"#
    );
}

/// pretty_dbg! for serde_json::Value
#[test]
fn test_04_macro_pretty_dbg_2() {
    fn macro_pretty_dbg_2() -> Result<(), anyhow::Error> {
        let response_text = r#"{
    "id": 1296269,
    "homepage": "https://github.com"
}"#;
        let parsed_json_value: serde_json::Value = serde_json::from_str(response_text)?;
        pretty_dbg!(&parsed_json_value);

        Ok(())
    }
    assert_stderr_eq!(
        macro_pretty_dbg_2().unwrap(),
        r#"[tests/integration_test.rs:110:9] &parsed_json_value = {
  "homepage": "https://github.com",
  "id": 1296269
}
"#
    );
}

/// format_dbg! in comparison with dbg! and pretty_dbg!
#[test]
fn test_05_macro_format_dbg_1() {
    fn macro_format_dbg_1() -> Result<(), anyhow::Error> {
        let val = "123456789";

        dbg!("using the dbg! macro : {val}");
        pretty_dbg!("using the pretty_dbg! macro : {val}");
        format_dbg!("using the format_dbg! macro : {val}");
        Ok(())
    }
    assert_stderr_eq!(
        macro_format_dbg_1().unwrap(),
        r#"[tests/integration_test.rs:130:9] "using the dbg! macro : {val}" = "using the dbg! macro : {val}"
[tests/integration_test.rs:131:9] "using the pretty_dbg! macro : {val}" = using the pretty_dbg! macro : {val}
[tests/integration_test.rs:132:9] using the format_dbg! macro : 123456789
"#
    );
}
