//! An input parser for positive and negative keywords input (e.g: +foo,-bar,+baz)
//! ## Getting Started
//! ```
//! use kwp::{Parser, Prefixes};
//! use std::env;
//!
//! fn main() {
//!     let input = env::args_os()
//!         .nth(1)
//!         .expect("No input provided.")
//!         .into_string()
//!         .unwrap();
//! 
//!     let parser = Parser::new(
//!         &input,
//!         Prefixes::default()
//!     );
//! 
//!     let (pos, neg, _) = parser.parse();
//!     println!(
//!         "Input: {}\nPositive: {:#?}\nNegative: {:#?}",
//!         input, pos, neg
//!     );
//! }
//! ```
//! # to test this, cargo test -- +foo,-bar,+baz

use std::str::Split;

/// Shorthand for parsed data from the parse function.
pub type Parsed = Vec<String>;
/// The response type from the parse function - basically a 3-tuple of `Vec<String>`.
pub type ParsedResponse = (Parsed, Parsed, Parsed);

/// Represents the positive and negative keyword prefixes for parsing.
pub struct Prefixes<'a> {
    pub positive: &'a str,
    pub negative: &'a str,
}

/// Default options for the Prefixes structure.
impl<'a> Default for Prefixes<'a> {
    fn default() -> Self {
        Self {
            positive: "+",
            negative: "-",
        }
    }
}

/// Represents the main parser
pub struct Parser<'a> {
    input: String,
    pub prefixes: Prefixes<'a>,
    retain_prefix: bool,
}

impl<'a> Parser<'a> {
    /// Creates a new parser
    /// ## Example
    /// ```
    /// use kwp::{Parser, Prefixes};
    ///
    /// let parser = Parser::new("+foo,-bar", Prefixes::default());
    /// ```
    pub fn new(input: &str, prefixes: Prefixes<'a>) -> Self {
        Self {
            input: input.to_string(),
            prefixes,
            retain_prefix: false,
        }
    }

    /// Wether or not to retain the prefix when parsing keywords.
    /// If set to true, the prefix of values will be stripped upon parsing.
    ///
    /// ## Example
    /// ```rs
    /// use kwp::{Parser, Prefixes};
    ///
    /// let mut parser = Parser::new("+foo", Prefixes::default());
    /// parser.should_retain_prefix(true);
    ///
    /// let (pos, _, _) = parser.parse();
    /// assert_eq!(pos, vec!["+foo"]);
    /// assert_ne!(pos, vec!["foo"]);
    /// ```
    pub fn should_retain_prefix(&mut self, bool: bool) -> bool {
        self.retain_prefix = bool;
        bool
    }

    /// Parses the provided split with the prefix
    fn parse_with_prefix(&self, split: Split<&str>, prefix: &str) -> Vec<String> {
        return split
            .filter(|e| e.starts_with(&prefix))
            .map(|e| {
                if self.retain_prefix {
                    e.replace(&prefix, "")
                } else {
                    e.to_string()
                }
            })
            .collect();
    }

    /// Parses the input.
    /// ## Example
    /// ```
    /// use kwp::{Parser, Prefixes};
    ///
    /// let parser = Parser::new("+foo,-bar,-baz", Prefixes::default());
    /// println!("{:#?}", parser.parse());
    /// ```
    pub fn parse(&self) -> ParsedResponse {
        let split = self.input.clone();
        let split = split.split(",");

        let pos = self.parse_with_prefix(split.clone(), self.prefixes.positive);
        let neg = self.parse_with_prefix(split.clone(), self.prefixes.negative);

        let other = split
            .filter(|x| !pos.iter().any(|y| x.contains(y)) && !neg.iter().any(|y| x.contains(y)))
            .map(|x| x.to_string())
            .collect();

        return (pos, neg, other);
    }
}

#[cfg(test)]
mod test {
    use crate::{Parser, Prefixes};
    #[test]
    fn basic_text() {
        let parser = Parser::new(
            "+foo,-bar,+baz",
            Prefixes {
                positive: "+",
                negative: "-",
            },
        );
        let (pos, neg, _) = parser.parse();
        assert_eq!(pos, vec!["+foo", "+baz"]);
        assert_eq!(neg, vec!["-bar"]);
    }

    #[test]
    fn do_not_retain_prefix() {
        let mut parser = Parser::new(
            "+foo,-bar,+baz",
            Prefixes {
                positive: "+",
                negative: "-",
            },
        );
        parser.should_retain_prefix(false);

        let (pos, neg, _) = parser.parse();
        assert_eq!(pos, vec!["+foo", "+baz"]);
        assert_eq!(neg, vec!["-bar"]);
    }

    #[test]
    fn weird_prefixes() {
        let parser = Parser::new(
            "yes!!foo,no!!bar,yes!!baz",
            Prefixes {
                positive: "yes!!",
                negative: "no!!",
            },
        );
        let (pos, neg, _) = parser.parse();
        assert_eq!(pos, vec!["yes!!foo", "yes!!baz"]);
        assert_eq!(neg, vec!["no!!bar"]);
    }

    #[test]
    fn unparsed() {
        let parser = Parser::new(
            "+foo,-bar,+baz,bak",
            Prefixes {
                positive: "+",
                negative: "-",
            },
        );
        let (_, _, other) = parser.parse();
        assert_eq!(other, vec!["bak"]);
    }
}
