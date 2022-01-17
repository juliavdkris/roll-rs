use owo_colors::OwoColorize;
use std::env;

mod parse;
mod roll;

fn main() {
	let args: Vec<String> = env::args().collect();
	let input = &args[1];

	let dice = parse::multiple_sets(input).unwrap();
	let rolls = &dice.iter().map(|d| d.roll()).collect::<Vec<roll::DieRoll>>();

	for roll in rolls {
		let result = roll_result_color(roll.result, roll.die.sides);
		println!("Sides: {}, Result: {}", roll.die.sides, result);
	}

	let sum: u32 = rolls.iter().map(|r| r.result).sum();
	let maxsum: u32 = rolls.iter().map(|r| r.die.sides).sum();
	println!("Sum: {}", roll_result_color(sum, maxsum));
}

fn roll_result_color(result: u32, max: u32) -> String {
	if result < max / 2 {
		result.red().to_string()
	} else {
		result.green().to_string()
	}
}
