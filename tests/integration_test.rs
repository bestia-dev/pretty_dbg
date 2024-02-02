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
"[tests/integration_test.rs:50] json_str = \"{\\n    owner: 'bestia-dev',\\n    repository_details: {\\n        general: {\\n        description: 'testing the creation of a github repo',\\n        },\\n    },\\n}\"\n");
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
"[tests/integration_test.rs:65] &parsed_json_value = Object {\n    \"homepage\": String(\"https://github.com\"),\n    \"id\": Number(1296269),\n}\n");
}

/// pretty_dbg! for multiline string
#[test]
fn test_03_macro_dbg_3() {
    fn macro_dbg_3() {
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
        macro_dbg_3(),
        r#"[tests/integration_test.rs:85] json_str = {
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
fn test_04_macro_dbg_4() {
    fn macro_dbg_4() -> Result<(), anyhow::Error> {
        let response_text = r#"{
    "id": 1296269,
    "homepage": "https://github.com"
}"#;
        let parsed_json_value: serde_json::Value = serde_json::from_str(response_text)?;
        pretty_dbg!(&parsed_json_value);

        Ok(())
    }
    assert_stderr_eq!(
        macro_dbg_4().unwrap(),
        r#"[tests/integration_test.rs:110] &parsed_json_value = {
  "homepage": "https://github.com",
  "id": 1296269
}
"#
    );
}
