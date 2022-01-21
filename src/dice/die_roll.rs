use super::Die;
use crate::misc;


pub struct DieRoll<T: Die> {
	die: T,
	result: u32
}


impl<T: Die> DieRoll<T> {
	pub fn new(die: T, result: u32) -> Self {
		Self { die, result }
	}

	pub fn fancy_result(&self) -> String {
		misc::interpolate_result_color(self.result, self.die.max_result())
	}

	pub fn max_result(&self) -> u32 {
		self.die.max_result()
	}
}
