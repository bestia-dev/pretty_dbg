// examples/macro_format_dbg_1.rs

use pretty_dbg::{format_dbg, pretty_dbg};

// cargo run --example macro_format_dbg_1
fn main() {
    let val = "123456789";

    dbg!("using the dbg! macro : {val}");
    pretty_dbg!("using the pretty_dbg! macro : {val}");
    format_dbg!("using the format_dbg! macro : {val}");
}
