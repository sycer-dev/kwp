//! An input parser for positive and negative keywords input (e.g: +foo,-bar,+baz)
//! ## Getting Started
//! ```
//! use kwp::{Parser, Prefixes};
//! use std::env;
//!
//! fn main() {
//!     let input = "+foo,-bar,+baz,-bak";
//!
//!     let parser = Parser::new(
//!         &input,
//!         Prefixes::default()
//!     );
//!
//!     let resp = parser.parse();
//!     println!("{:#?}", resp);
//! }
//! ```
// to test this, cargo test -- +foo,-bar,+baz

use std::str::Split;

/// Shorthand for parsed data from the parse function.
pub type Parsed = Vec<String>;
/// The response type from the parse function - basically a 3-tuple of `Vec<String>`.
pub type ParsedResponse = (Parsed, Parsed, Parsed);

/// Represents the positive and negative keyword prefixes for parsing.
#[derive(Debug, Copy, Clone)]
pub struct Prefixes<'a> {
    pub positive: &'a str,
    pub negative: &'a str,
}

#[derive(Debug, Clone)]
pub struct Keywords {
    pub positive: Parsed,
    pub negative: Parsed,
    pub other: Parsed,
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
    /// ```
    /// use kwp::{Parser, Prefixes};
    ///
    /// let mut parser = Parser::new("+foo", Prefixes::default());
    /// parser.should_retain_prefix(true);
    ///
    /// let keywords = parser.parse();
    /// assert_eq!(keywords.positive, vec!["+foo"]);
    /// // assert_ne!(keywords.positive, vec!["foo"]);
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
                if !self.retain_prefix {
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
    pub fn parse(&self) -> Keywords {
        let split = self.input.clone();
        let split = split.split(",");

        let positive = self.parse_with_prefix(split.clone(), self.prefixes.positive);
        let negative = self.parse_with_prefix(split.clone(), self.prefixes.negative);

        let other = split
            .filter(|x| {
                !positive.iter().any(|y| x.contains(y)) && !negative.iter().any(|y| x.contains(y))
            })
            .map(|x| x.to_string())
            .collect();

        return Keywords {
            positive,
            negative,
            other,
        };
    }

    /// Finds products that match the provided positive & negative keywords.  
    /// ⚠️ Case insensitive 
    /// ## Example
    /// ```
    /// use kwp::{Parser, Prefixes};
    /// 
    /// let products = vec!["MyProduct Adult", "MyProduct Youth"];
    ///     let parser = Parser::new(
    ///         "+myproduct,-youth",
    ///         Prefixes {
    ///             positive: "+",
    ///             negative: "-",
    ///         },
    ///     );
    /// let keywords = parser.parse();
    ///
    /// let products = parser.match_products(products.clone(), keywords.clone());
    /// assert_eq!(products, vec!["MyProduct Adult"]);
    /// ```
    pub fn match_products(&self, products: Vec<&str>, keywords: Keywords) -> Vec<String> {
        let mut found: Vec<String> = vec![];
        for product in products {
            let p_lower = &product.to_lowercase();
            if keywords.positive.iter().any(|e| p_lower.to_lowercase().contains(&e.to_lowercase()))
                && !keywords.negative.iter().any(|e| p_lower.contains(&e.to_lowercase()))
            {
                found.push(product.to_string());
            }
        }
        return found;
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
        let keywords = parser.parse();
        assert_eq!(keywords.positive, vec!["foo", "baz"]);
        assert_eq!(keywords.negative, vec!["bar"]);
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

        let keywords = parser.parse();
        assert_eq!(keywords.positive, vec!["foo", "baz"]);
        assert_eq!(keywords.negative, vec!["bar"]);
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
        let keywords = parser.parse();
        assert_eq!(keywords.positive, vec!["foo", "baz"]);
        assert_eq!(keywords.negative, vec!["bar"]);
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
        let keywords = parser.parse();
        assert_eq!(keywords.other, vec!["bak"]);
    }

    #[test]
    fn basic_products() {
        let products = vec!["MyProduct Adult", "MyProduct Youth"];
        let parser = Parser::new(
            "+myproduct,-youth",
            Prefixes {
                positive: "+",
                negative: "-",
            },
        );
        let keywords = parser.parse();
        let products = parser.match_products(products.clone(), keywords.clone());
        assert_eq!(products, vec!["MyProduct Adult"]);
    }
}
