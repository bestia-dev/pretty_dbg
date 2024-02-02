// pretty_dbg/src/lib.rs

// region: auto_md_to_doc_comments include README.md A //!

// endregion: auto_md_to_doc_comments include README.md A //!

/// copy of the macro dbg!, just modified :#? to :# for pretty print
#[macro_export]
macro_rules! pretty_dbg {
    () => {
        std::eprintln!("[{}:{}]", std::file!(), std::line!())
    };
    ($val:expr $(,)?) => {
        match $val {
            tmp => {
                std::eprintln!("[{}:{}] {} = {:#}",
                    std::file!(), std::line!(), std::stringify!($val), &tmp);
                tmp
            }
        }
    };
    ($($val:expr),+ $(,)?) => {
        ($(std::pretty_dbg!($val)),+,)
    };
}
