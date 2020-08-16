# kw-parser
> An input parser for positive and negative keywords input (e.g: +foo,-bar,+baz)  
[![Actions](https://img.shields.io/github/workflow/status/sycer-dev/kwp/kwp?style=flat)](https://github.com/sycer-dev/kwp/actions)
[![Crate](https://img.shields.io/crates/v/kwp.svg?style=flat)](https://crates.io/crates/kwp)
[![Downloads](https://img.shields.io/crates/d/kwp.svg?style=flat)](https://crates.io/crates/kwp)

## installation
```yml
# within Cargo.toml
kwp = "0.2"
```

## example
```rust
use kwp::{Parser, Prefixes};

fn main() {
    let input = "+foo,-bar,+baz,-bak";

    let parser = Parser::new(
        &input,
        Prefixes::default()
    );
    let res = parser.parse();
    println!("{:#?}", res);
}
```