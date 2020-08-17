use kwp::{Parser, Prefixes};

fn main() {
    let input = "-youth,+hoodie,+hat";
	let products = vec!["Youth Tee", "Blue Tee - Youth", "Blurple Hoodie", "Wumpus Hat"];
	
    let parser = Parser::new(
        &input,
        Prefixes::default()
    );
	let res = parser.parse();
	
	let ext = parser.match_products(products.clone(), res.clone());
    println!("{:#?}\n{:#?}", res, ext);
}