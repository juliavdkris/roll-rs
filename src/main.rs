mod parse;
mod roll;

fn main() {
	let input = String::from("3d8");
	let dice = parse::set(&input).unwrap();

	for die in dice {
		let result = die.roll().result;
		println!("Sides: {}, Result: {}", die.sides, result);
	}
}
