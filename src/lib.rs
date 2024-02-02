// pretty_dbg/src/lib.rs

// region: auto_md_to_doc_comments include README.md A //!
//! # macro pretty_dbg!()  
//!
//! **copy of the macro dbg!, just modified :#? to :# for pretty print**  
//! ***version: 1.0.47 date: 2024-02-02 author: [bestia.dev](https://bestia.dev) repository: [Github](https://github.com/bestia-dev/pretty_dbg)***  
//!
//!  ![status](https://img.shields.io/badge/pre_alpha-red)
//!
//! [![Lines in Rust code](https://img.shields.io/badge/Lines_in_Rust-21-green.svg)](https://github.com/bestia-dev/pretty_dbg/)
//! [![Lines in Doc comments](https://img.shields.io/badge/Lines_in_Doc_comments-1-blue.svg)](https://github.com/bestia-dev/pretty_dbg/)
//! [![Lines in Comments](https://img.shields.io/badge/Lines_in_comments-3-purple.svg)](https://github.com/bestia-dev/pretty_dbg/)
//! [![Lines in examples](https://img.shields.io/badge/Lines_in_examples-65-yellow.svg)](https://github.com/bestia-dev/pretty_dbg/)
//! [![Lines in tests](https://img.shields.io/badge/Lines_in_tests-122-orange.svg)](https://github.com/bestia-dev/pretty_dbg/)
//!
//! [![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/bestia-dev/pretty_dbg/blob/master/LICENSE)
//!
//! Hashtags: #rustlang #tutorial  
//! My projects on GitHub are more like a tutorial than a finished product: [bestia-dev tutorials](https://github.com/bestia-dev/tutorials_rust_wasm).
//!
//! ## Motivation
//!
//! I love using the macro `dbg!()` in Rust. It is an easy way to temporarily print a value on the screen while programming. And when not needed anymore it is easy to search for all `dbg!` and erase or comment them.  
//! In my last project, I had some JSON data. The macro from the standard library printed a humanly unreadable long string. This is not pretty! Even multiline strings are printed in one single line making it unreadable.
//!
//! Run this code in the [rust playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=a8f5eb34d97ec5818550f59af9a3c545):
//!
//! ```rust
//! fn main() {
//!     let json_str = r#"
//! {
//!     owner: 'bestia-dev',
//!     repository_details: {
//!         general: {
//!         description: 'testing the creation of a github repo',
//!         },
//!     },
//! }
//! "#;
//!     dbg!(json_str);
//! }
//! ```
//!
//! This is the unreadable output for a string:
//!
//! ```output
//! [src/main.rs:12] json_str = "\n   {\n    owner: 'bestia-dev',\n    repository_details: {\n      general: {\n        description: 'testing the creation of a github repo',\n      },\n    },\n  }\n"
//! ```
//!
//! Now let's try with the serde_json::Value:
//!
//! Run this code in the [rust playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=c88ade301a7aa16bf27e7b8b9b6790ac):
//!
//! ```rust
//! use serde_json;
//! use anyhow::Result;
//!
//! fn main() -> Result<(), anyhow::Error>{
//!        let response_text =
//! r#"{
//!     "id": 1296269,
//!     "homepage": "https://github.com"
//! }"#;
//!     let parsed_json_value: serde_json::Value = serde_json::from_str(response_text)?;
//!     dbg!(&parsed_json_value);
//!     
//!     Ok(())
//! }
//! ```
//!
//! This returns also an unreadable output:
//!
//! ```output
//! [src/main.rs:11] &parsed_json_value = Object {
//!     "homepage": String("https://github.com"),
//!     "id": Number(1296269),
//! }
//! ```
//!
//! I know that `dbg!` under the hood is just a simple `eprintln!("{:#?}, json")`.
//! And I know that I can print pretty JSON using `eprintln!("{:#}, json")` but then I don't express neatly my intent to `dbg!`. And I lose the possibility to search for `dbg!`.  
//!
//! I found a crate that colors the `dbg!` output and is really pretty: [dbg-pls](https://github.com/conradludgate/dbg-pls). That is maybe too much for my little project.  
//!
//! ## new macro `pretty_dbg!`
//!
//! So I decided to copy the original macro `dbg!`, modify it a little bit, and give it the name `pretty_dbg!`.  
//!
//! Run this code in the [rust playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=e9ef982465f717d9c5b642aead94a4ff):
//!
//! ```rust
//! /// copy of the macro dbg!, just modified :#? to :# for pretty print
//! #[macro_export]
//! macro_rules! pretty_dbg {
//!     () => {
//!         std::eprintln!("[{}:{}]", std::file!(), std::line!())
//!     };
//!     ($val:expr $(,)?) => {
//!         match $val {
//!             tmp => {
//!                 std::eprintln!("[{}:{}] {} = {:#}",
//!                     std::file!(), std::line!(), std::stringify!($val), &tmp);
//!                 tmp
//!             }
//!         }
//!     };
//!     ($($val:expr),+ $(,)?) => {
//!         ($(std::pretty_dbg!($val)),+,)
//!     };
//! }
//!
//! fn main() {
//!     let json_str = r#"
//!    {
//!     owner: 'bestia-dev',
//!     repository_details: {
//!       general: {
//!         description: 'testing the creation of a github repo',
//!       },
//!     },
//!   }
//! "#;
//!     pretty_dbg!(json_str);
//! }
//! ```
//!
//! The output is now very pretty:
//!
//! ```output
//! [src/main.rs:32] json_str =
//!    {
//!     owner: 'bestia-dev',
//!     repository_details: {
//!       general: {
//!         description: 'testing the creation of a github repo',
//!       },
//!     },
//!   }
//! ```
//!
//! Now let's try with the serde_json::Value:
//!
//! Run this code in the [rust playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=d5d5e264b9143f4fde16594eaea1fa09):
//!
//! ```rust
//! use serde_json;
//! use anyhow::Result;
//!
//! /// copy of the macro dbg!, just modified :#? to :# for pretty print
//! #[macro_export]
//! macro_rules! pretty_dbg {
//!     () => {
//!         std::eprintln!("[{}:{}]", std::file!(), std::line!())
//!     };
//!     ($val:expr $(,)?) => {
//!         match $val {
//!             tmp => {
//!                 std::eprintln!("[{}:{}] {} = {:#}",
//!                     std::file!(), std::line!(), std::stringify!($val), &tmp);
//!                 tmp
//!             }
//!         }
//!     };
//!     ($($val:expr),+ $(,)?) => {
//!         ($(std::pretty_dbg!($val)),+,)
//!     };
//! }
//!
//! fn main() -> Result<(), anyhow::Error>{
//!        let response_text =
//! r#"{
//!     "id": 1296269,
//!     "homepage": "https://github.com"
//! }"#;
//!     let parsed_json_value: serde_json::Value = serde_json::from_str(response_text)?;
//!     pretty_dbg!(&parsed_json_value);
//!     
//!     Ok(())
//! }
//! ```
//!
//! The output is now pretty:
//!
//! ```output
//! [src/main.rs:31] &parsed_json_value = {
//!   "homepage": "https://github.com",
//!   "id": 1296269
//! }
//! ```
//!
//! ## new crate or not
//!
//! I think this is maybe too small to be made in a new crate.  
//! It is just a small macro.  
//! For now, I am just adding the code for this macro in my projects where I need it.  
//!
//! ## playing with the Rust Playground
//!
//! Trying short Rust code in the Rust playground is great! It is fast and easy. It works just with a browser. Fantastic to show examples of real code, not just hypothetical code.  
//! I first created Github Gists for my code examples. Every example must be in a separate Gist. Then I copy the Gist identification number into the playground link like this: <https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=d5d5e264b9143f4fde16594eaea1fa09> and it just works. Great!
//!
//! I want to avoid using `unwrap!` in my examples. Unwrap is a bad, bad habit.   Instead, I will use the crate `anyhow` and its types `Result` and `Error` directly in the main() function. So I can use the error propagation symbol `?` in the code like a pro.  
//!
//! ## Open-source and free as a beer
//!
//! My open-source projects are free as a beer (MIT license).  
//! I just love programming.  
//! But I need also to drink. If you find my projects and tutorials helpful, please buy me a beer by donating to my [PayPal](https://paypal.me/LucianoBestia).  
//! You know the price of a beer in your local bar ;-)  
//! So I can drink a free beer for your health :-)  
//! [Na zdravje!](https://translate.google.com/?hl=en&sl=sl&tl=en&text=Na%20zdravje&op=translate) [Alla salute!](https://dictionary.cambridge.org/dictionary/italian-english/alla-salute) [Prost!](https://dictionary.cambridge.org/dictionary/german-english/prost) [Nazdravlje!](https://matadornetwork.com/nights/how-to-say-cheers-in-50-languages/) 🍻
//!
//! [//bestia.dev](https://bestia.dev)  
//! [//github.com/bestia-dev](https://github.com/bestia-dev)  
//! [//bestiadev.substack.com](https://bestiadev.substack.com)  
//! [//youtube.com/@bestia-dev-tutorials](https://youtube.com/@bestia-dev-tutorials)  
//!
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