use regex::Regex;

use crate::roll::Die;

/// Parse a single set of rolls, and return a list of (unrolled) dice.
///
/// # Example
/// ```
/// let dice = roll("2d6")?;
/// ```
pub fn set(text: &str) -> Result<Vec<Die>, Box<dyn std::error::Error>> {
	let re = Regex::new(
		r"(?x)
		(?P<rolls>\d*)
		d
		(?P<sides>\d+)
	",
	)?;
	let caps = re.captures(text).ok_or(std::fmt::Error)?;
	let rolls = caps["rolls"].parse::<u8>()?;
	let sides = caps["sides"].parse::<u8>()?;

	Ok((0..rolls).map(|_| Die::new(sides)).collect())
}
