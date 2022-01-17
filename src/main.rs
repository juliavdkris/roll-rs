use std::env;

mod misc;
mod parse;
mod roll;


fn main() {
	let args: Vec<String> = env::args().collect();
	let input = &args[1];

	let dice = parse::multiple_sets(input).unwrap();
	let rolls = &dice.iter().map(|d| d.roll()).collect::<Vec<roll::DieRoll>>();

	for roll in rolls {
		println!("Sides: {}, Result: {}", roll.die.sides, roll.pretty_result());
	}

	let sum: u32 = rolls.iter().map(|r| r.result).sum();
	let max_sum: u32 = rolls.iter().map(|r| r.die.sides).sum();
	println!("Sum: {}", misc::interpolate_result_color(sum, max_sum));
}
