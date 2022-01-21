use super::Die;

pub struct DieRoll<T: Die> {
	die: T,
	result: u32
}


impl<T: Die> DieRoll<T> {
	pub fn new(die: T, result: u32) -> Self {
		Self { die, result }
	}
}
