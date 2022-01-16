use regex::Regex;

use crate::roll::Die;

/// Parse a single set of rolls, and return a list of (unrolled) dice.
///
/// # Example
/// ```
/// let dice: Vec<Roll> = roll("2d6")?;
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
	let rolls = caps["rolls"].parse::<u8>().unwrap_or(1);
	let sides = caps["sides"].parse::<u8>()?;

	Ok((0..rolls).map(|_| Die::new(sides)).collect())
}

// Unit tests
#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn set_single_explicit_1d6() {
		let expected = vec![Die::new(6)];
		assert_eq!(expected, set("1d6").unwrap());
	}

	#[test]
	fn set_single_implicit_d6() {
		let expected = vec![Die::new(6)];
		assert_eq!(expected, set("d6").unwrap());
	}

	#[test]
	fn set_multiple_2d6() {
		let expected = vec![Die::new(6), Die::new(6)];
		assert_eq!(expected, set("2d6").unwrap());
	}
}
