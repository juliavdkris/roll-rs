use std::env;

mod parse;
mod roll;

fn main() {
	let args: Vec<String> = env::args().collect();
	let input = &args[1];

	let dice = parse::multiple_sets(input).unwrap();

	for die in dice {
		let result = die.roll().result;
		println!("Sides: {}, Result: {}", die.sides, result);
	}
}
