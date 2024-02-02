// examples/macro_dbg_4.rs

use pretty_dbg::pretty_dbg;

// cargo run --example macro_dbg_4
fn main() -> Result<(), anyhow::Error> {
    let response_text = r#"{
    "id": 1296269,
    "homepage": "https://github.com"
}"#;
    let parsed_json_value: serde_json::Value = serde_json::from_str(response_text)?;
    pretty_dbg!(&parsed_json_value);

    Ok(())
}
