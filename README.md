[//]: # (auto_md_to_doc_comments segment start A)

# Rust macro pretty_dbg!()  

[//]: # (auto_cargo_toml_to_md start)

**copy of the macro dbg!, just modified :#? to :# for pretty print**  
***version: 1.0.53 date: 2024-02-22 author: [bestia.dev](https://bestia.dev) repository: [GitHub](https://github.com/bestia-dev/pretty_dbg)***  

[//]: # (auto_cargo_toml_to_md end)

 ![maintained](https://img.shields.io/badge/maintained-green)
 ![ready_for_use](https://img.shields.io/badge/ready_for_use-green)

 [![crates.io](https://img.shields.io/crates/v/pretty_dbg.svg)](https://crates.io/crates/pretty_dbg)
 [![Documentation](https://docs.rs/pretty_dbg/badge.svg)](https://docs.rs/pretty_dbg/)
 [![crev reviews](https://web.crev.dev/rust-reviews/badge/crev_count/pretty_dbg.svg)](https://web.crev.dev/rust-reviews/crate/pretty_dbg/)
 [![RustActions](https://github.com/bestia-dev/pretty_dbg/actions/workflows/rust_fmt_auto_build_test.yml/badge.svg)](https://github.com/bestia-dev/pretty_dbg/)

 [![latest doc](https://img.shields.io/badge/latest_docs-GitHub-orange.svg)](https://bestia-dev.github.io/pretty_dbg/pretty_dbg/index.html)
 [![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/bestia-dev/pretty_dbg/blob/main/LICENSE)
 ![pretty_dbg](https://bestia.dev/webpage_hit_counter/get_svg_image/1350166552.svg)

[//]: # (auto_lines_of_code start)
[![Lines in Rust code](https://img.shields.io/badge/Lines_in_Rust-30-green.svg)](https://github.com/bestia-dev/pretty_dbg/)
[![Lines in Doc comments](https://img.shields.io/badge/Lines_in_Doc_comments-3-blue.svg)](https://github.com/bestia-dev/pretty_dbg/)
[![Lines in Comments](https://img.shields.io/badge/Lines_in_comments-8-purple.svg)](https://github.com/bestia-dev/pretty_dbg/)
[![Lines in examples](https://img.shields.io/badge/Lines_in_examples-77-yellow.svg)](https://github.com/bestia-dev/pretty_dbg/)
[![Lines in tests](https://img.shields.io/badge/Lines_in_tests-142-orange.svg)](https://github.com/bestia-dev/pretty_dbg/)

[//]: # (auto_lines_of_code end)

Hashtags: #rustlang #tutorial  
My projects on GitHub are more like a tutorial than a finished product: [bestia-dev tutorials](https://github.com/bestia-dev/tutorials_rust_wasm).

## Motivation

I love using the macro `dbg!()` in Rust. It is an easy way to temporarily print a value on the screen while programming and debugging.  
When not needed anymore it is easy to search for all `dbg!` and erase or comment them.  
In my last project, I had some JSON data. The macro from the standard library printed a humanly unreadable long string. This is not pretty! Even multiline strings are printed in one single line making it unreadable.

Run this code in the [rust playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=a8f5eb34d97ec5818550f59af9a3c545):

```rust
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
    dbg!(json_str);
}
```

This is the unreadable output for a string:

```output
[src/main.rs:12] json_str = "\n   {\n    owner: 'bestia-dev',\n    repository_details: {\n      general: {\n        description: 'testing the creation of a github repo',\n      },\n    },\n  }\n"
```

Now let's try with the serde_json::Value:

Run this code in the [rust playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=c88ade301a7aa16bf27e7b8b9b6790ac):

```rust
use serde_json;
use anyhow::Result;

fn main() -> Result<(), anyhow::Error>{
       let response_text =
r#"{
    "id": 1296269,
    "homepage": "https://github.com"
}"#;
    let parsed_json_value: serde_json::Value = serde_json::from_str(response_text)?;
    dbg!(&parsed_json_value);
    
    Ok(())
}
```

This returns also an unreadable output:

```output
[src/main.rs:11] &parsed_json_value = Object {
    "homepage": String("https://github.com"),
    "id": Number(1296269),
}
```

I know that `dbg!` under the hood is just a simple `eprintln!("{:#?}, json")`.
And I know that I can print pretty JSON using `eprintln!("{:#}, json")` but then I don't express neatly my intent to `dbg!`. And I lose the possibility to search for `dbg!`.  

I found a crate that colors the `dbg!` output and is really pretty: [dbg-pls](https://github.com/conradludgate/dbg-pls). That is maybe too much for my little project.  

## new macro `pretty_dbg!`

So I decided to copy the original macro `dbg!`, modify it a little bit, and give it the name `pretty_dbg!`.  

Run this code in the [rust playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=e9ef982465f717d9c5b642aead94a4ff):

```rust
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
```

The output is now very pretty:

```output
[src/main.rs:32] json_str = 
   {
    owner: 'bestia-dev',
    repository_details: {
      general: {
        description: 'testing the creation of a github repo',
      },
    },
  }
```

Now let's try with the serde_json::Value:

Run this code in the [rust playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=d5d5e264b9143f4fde16594eaea1fa09):

```rust
use serde_json;
use anyhow::Result;

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

fn main() -> Result<(), anyhow::Error>{
       let response_text =
r#"{
    "id": 1296269,
    "homepage": "https://github.com"
}"#;
    let parsed_json_value: serde_json::Value = serde_json::from_str(response_text)?;
    pretty_dbg!(&parsed_json_value);
    
    Ok(())
}
```

The output is now pretty:

```output
[src/main.rs:31] &parsed_json_value = {
  "homepage": "https://github.com",
  "id": 1296269
}
```

## format_dbg! macro

Sometimes when debugging I want to write some string to the output and not only a variable.  
The macro `dbg!` and consequently `pretty_dbg!` are not the right tools for that.  

Again I could use simply the `eprintln!`, but then it is not easy to find and remove this debugging code.  
Let's make another macro `format_dbg!`. It is just a simpler `dbg!` with another name.

Run this code in the [rust playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=89ac8ec47ac972d4812e28846c803628):

```rust
fn main() {
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

/// format_dbg! is a version of dbg! that uses the formatting rules from the macro eprintln!
/// Just like dbg!, it prefixes the stderr output with file! and line!
#[macro_export]
macro_rules! format_dbg {
    ($($arg:tt)*) => {{
        std::eprint!("[{}:{}] ", std::file!(), std::line!());
        std::eprintln!($($arg)*);
    }};
}
    let val="123456789";

    dbg!("using the dbg! macro : {val}");
    pretty_dbg!("using the pretty_dbg! macro : {val}");
    format_dbg!("using the format_dbg! macro : {val}");
}
```

## New crate in crates.io or not

I think this is maybe too small to be made in a new crate.  
It is just a small macro.  
For now, I am just adding the code for this macro in my projects where I need it.  

I changed my mind. I will publish a micro crate and I will make it exemplary because it is so small.  
I will add tests, examples, playground code, documentation,... as well as I could.

## Playing with the Rust Playground

Trying short Rust code in the Rust playground is great! It is fast and easy. It works just with a browser. Fantastic to show examples of real code, not just hypothetical code.  
I first created Github Gists for my code examples. Every example must be in a separate Gist. Then I copy the Gist identification number into the playground link like this: <https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=d5d5e264b9143f4fde16594eaea1fa09> and it just works. Great!

I want to avoid using `unwrap!` in my examples. Unwrap is a bad, bad habit.   Instead, I will use the crate `anyhow` and its types `Result` and `Error` directly in the main() function. So I can use the error propagation symbol `?` in the code like a pro.  

## Integration tests for err/std output

Integration tests by default capture the std output and run in parallel. In my case, this is exactly what I don't want.  
I need to capture the err/std output because this is the whole point of how the pretty_dbg! macro works. When working with the std/err output, code must not run in parallel because it would mix the output from different code and make it like scrambled eggs.  
It took a while to modify the code in my automation tasks and the calling parameters to run the tests in this non-standard way.  
I found the crate `gag` that can capture err/std output and I used a macro for my integration tests. This will come in handy for a lot of tests for CLI apps.  

## Open-source and free as a beer

My open-source projects are free as a beer (MIT license).  
I just love programming.  
But I need also to drink. If you find my projects and tutorials helpful, please buy me a beer by donating to my [PayPal](https://paypal.me/LucianoBestia).  
You know the price of a beer in your local bar ;-)  
So I can drink a free beer for your health :-)  
[Na zdravje!](https://translate.google.com/?hl=en&sl=sl&tl=en&text=Na%20zdravje&op=translate) [Alla salute!](https://dictionary.cambridge.org/dictionary/italian-english/alla-salute) [Prost!](https://dictionary.cambridge.org/dictionary/german-english/prost) [Nazdravlje!](https://matadornetwork.com/nights/how-to-say-cheers-in-50-languages/) 🍻

[//bestia.dev](https://bestia.dev)  
[//github.com/bestia-dev](https://github.com/bestia-dev)  
[//bestiadev.substack.com](https://bestiadev.substack.com)  
[//youtube.com/@bestia-dev-tutorials](https://youtube.com/@bestia-dev-tutorials)  

[//]: # (auto_md_to_doc_comments segment end A)
