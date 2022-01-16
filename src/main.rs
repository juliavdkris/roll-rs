mod parse;
mod roll;

fn main() {
	let input = String::from("3d8");
	let roll = parse::singular_roll(&input).unwrap();
	let result = roll.roll().result;

	println!("Sides: {}, Result: {}", roll.sides, result);
}
