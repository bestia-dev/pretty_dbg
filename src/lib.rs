// pretty_dbg/src/lib.rs

// logo for docs.rs in png
// #![doc(html_logo_url = "https://github.com/bestia-dev/cargo-auto/raw/main/images/logo/logo_cargo_auto.svg")]
// even favicon ico can be changed
// #![doc(html_favicon_url = "/logo.ico")]
// playground for examples. Warning: It didn't work well for me. And it works only in docs. Not in Github and not in crates.io.
// I will not use it. I will crate a gist and use that for the playground. That works flawlessly. Maybe create an automation task?
// #![doc(html_playground_url = "https://play.rust-lang.org/")]
// example how to insert a svg file inside the documentation
// #![doc=include_str!("shared-bus.svg")]
#![doc=include_str!("../README.md")]

/// pretty_dbg! is a copy of the macro dbg!, just with :#? instead of :# for pretty print
#[macro_export]
macro_rules! pretty_dbg {
    () => {
        std::eprintln!("[{}:{}:{}]", std::file!(), std::line!(), std::column!())
    };
    ($val:expr $(,)?) => {
        match $val {
            tmp => {
                std::eprintln!("[{}:{}:{}] {} = {:#}",
                    std::file!(), std::line!(), std::column!(), std::stringify!($val), &tmp);
                tmp
            }
        }
    };
    ($($val:expr),+ $(,)?) => {
        ($(std::pretty_dbg!($val)),+,)
    };
}

/// format_dbg! is a version of dbg! that uses the formatting rules from the macro eprintln!
/// Just like dbg!, it prefixes the stderr output with file!, line! and column!
#[macro_export]
macro_rules! format_dbg {
    ($($arg:tt)*) => {{
        std::eprint!("[{}:{}:{}] ", std::file!(), std::line!(), std::column!());
        std::eprintln!($($arg)*);
    }};
}
