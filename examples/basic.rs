use kwp::{Parser, Prefixes};

fn main() {
    let input = "+foo,-bar,+baz,-bak";

    let parser = Parser::new(&input, Prefixes::default());
    let res = parser.parse();
    println!("{:#?}", res);
}
