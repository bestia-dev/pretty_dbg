// examples/macro_dbg_1.rs

// cargo run --example macro_dbg_1
fn main() {
    let json_str = r#"
{
    github_owner: 'bestia-dev',
    repository_details: {
        general: {
        description: 'testing the creation of a github repo',
        },
    },
}
"#;
    dbg!(json_str);
}
