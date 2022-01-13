use regex::Regex;

use crate::roll::DiceRoll;


pub fn singular_roll(text: &str) -> Result<DiceRoll, Box<dyn std::error::Error>> {
	let re = Regex::new(
		r"(?x)
		(?P<rolls>\d*)
		d
		(?P<sides>\d+)
	"
	)?;
	let caps = re.captures(text).ok_or(std::fmt::Error)?;
	let sides = caps["rolls"].parse::<u8>()?;

	Ok(DiceRoll::new(sides))
}
