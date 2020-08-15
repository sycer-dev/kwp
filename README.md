# kw-parser
> An input parser for positive and negative keywords input (e.g: +foo,-bar,+baz)
[![Actions](https://img.shields.io/github/workflow/status/sycer-dev/kwp/kwp?style=flat)](https://github.com/sycer-dev/kwp/actions)
[![Crate](https://img.shields.io/crates/v/kwp.svg?style=flat)](https://crates.io/crates/kwp)
[![Downloads](https://img.shields.io/crates/d/kwp.svg?style=flat)](https://crates.io/crates/kwp)

## installation
```yml
# within Cargo.toml

kwp = "0.1.0"
```

## `example`
```rs
// cargo run -- +my,-keywords,+here

use kwp::{Parser, Prefixes};
use std::env;

fn main() {
    let input = env::args_os()
        .nth(1)
        .expect("No input provided.")
        .into_string()
        .unwrap();

    let parser = Parser::new(
        &input,
        Prefixes::default()
    );
    let (pos, neg, other) = parser.parse();
    println!(
        "Input: {}\nPositive: {:#?}\nNegative: {:#?}\nOther: {:#?}",
        input, pos, neg, other
    );
}
```