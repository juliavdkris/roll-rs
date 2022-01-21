use std::env;

mod dice;
mod misc;


fn main() {
	let args: Vec<String> = env::args().collect();
	let input = &args.get(1);

	let dice = vec![
		dice::RandomDie::new(6),
		dice::RandomDie::new(6),
		dice::RandomDie::new(8),
		dice::RandomDie::new(8),
		dice::RandomDie::new(8),
	];

	let rolls = dice.into_iter().map(dice::Die::roll);

	for r in rolls {
		r.fancy_result();
	}
}
