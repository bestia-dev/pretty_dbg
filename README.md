[//]: # (auto_md_to_doc_comments segment start A)

# macro pretty_dbg!()  

[//]: # (auto_cargo_toml_to_md start)

**copy of the macro dbg!, just modified :#? to :# for pretty print**  
***version: 1.0.40 date: 2024-02-01 author: [bestia.dev](https://bestia.dev) repository: [GitHub](https://github.com/bestia-dev/pretty_dbg)***  

[//]: # (auto_cargo_toml_to_md end)

 ![status](https://img.shields.io/badge/pre_alpha-red)

[//]: # (auto_lines_of_code start)

[//]: # (auto_lines_of_code end)

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/bestia-dev/pretty_dbg/blob/master/LICENSE)

Hashtags: #rustlang #tutorial  
My projects on GitHub are more like a tutorial than a finished product: [bestia-dev tutorials](https://github.com/bestia-dev/tutorials_rust_wasm).

## Motivation

I love using the macro `dbg!()` in Rust. It is an easy way to temporarily print a value on the screen while programming. And when not needed anymore it is easy to search for all `dbg!` and erase or comment them.  
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

## new crate or not

I think this is maybe too small to be made in a new crate.  
It is just a small macro.  
I am just adding the code for this macro in my projects where I need it.  

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
