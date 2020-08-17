
use kwp::{Parser, Prefixes};

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
