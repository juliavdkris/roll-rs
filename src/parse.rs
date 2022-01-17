use regex::Regex;

use crate::roll::Die;

/// Parse a single set of rolls, and return a list of (unrolled) dice.
///
/// # Example
/// ```
/// let dice: Vec<Die> = set("2d6")?;
/// ```
pub fn set(text: &str) -> Result<Vec<Die>, Box<dyn std::error::Error>> {
	let re = Regex::new(
		r"^(?x)
		(?P<rolls>\d*)
		d
		(?P<sides>\d+)
		$
	",
	)?;
	let caps = re.captures(text).ok_or(std::fmt::Error)?;
	let rolls = caps["rolls"].parse::<u8>().unwrap_or(1);
	let sides = caps["sides"].parse::<u8>()?;

	Ok((0..rolls).map(|_| Die::new(sides)).collect())
}

/// Parse multiple sets of rolls, and return a list of (unrolled) dice.
///
/// # Example
/// ```
/// let dice: Vec<Die> = multiple_sets("2d6 + 3d8")?;
/// ```
pub fn multiple_sets(text: &str) -> Result<Vec<Die>, Box<dyn std::error::Error>> {
	let rolls = text.split(" + ").map(set).collect::<Result<Vec<Vec<Die>>, _>>()?;
	let flattened = rolls.into_iter().flatten().collect::<Vec<Die>>();
	Ok(flattened)
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

	#[test]
	fn set_invalid() {
		let result = set("fuckyou");
		assert!(result.is_err());
	}

	#[test]
	fn set_invalid_contains_valid() {
		let result = set("fuckyou1d6nahm8");
		assert!(result.is_err());
	}
}
