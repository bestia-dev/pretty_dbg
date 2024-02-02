// examples/macro_dbg_3.rs

use pretty_dbg::pretty_dbg;

// cargo run --example macro_dbg_1
fn main() {
    let json_str = r#"
   {
    owner: 'bestia-dev',
    repository_details: {
      general: {
        description: 'testing the creation of a github repo',
      },
    },
  }
"#;
    pretty_dbg!(json_str);
}
