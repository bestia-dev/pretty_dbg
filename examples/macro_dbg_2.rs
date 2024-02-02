// examples/macro_dbg_2.rs

// cargo run --example macro_dbg_2
use anyhow::Result;
use serde_json;

fn main() -> Result<(), anyhow::Error> {
    let response_text = r#"{
    "id": 1296269,
    "homepage": "https://github.com"
}"#;
    let parsed_json_value: serde_json::Value = serde_json::from_str(response_text)?;
    dbg!(&parsed_json_value);

    Ok(())
}
