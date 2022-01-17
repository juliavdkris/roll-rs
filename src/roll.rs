use rand::Rng;

use crate::misc;


// TODO: implement ConstantDie

#[derive(PartialEq, Debug)]
pub struct Die {
	pub sides: u32
}

pub struct DieRoll<'a> {
	pub die: &'a Die,
	pub result: u32
}


impl Die {
	pub fn new(sides: u32) -> Self {
		Self { sides }
	}

	pub fn roll(&self) -> DieRoll {
		let mut rng = rand::thread_rng();
		let result = rng.gen_range(1..=self.sides);
		DieRoll { die: self, result }
	}
}

impl<'a> DieRoll<'a> {
	pub fn pretty_result(&self) -> String {
		misc::interpolate_result_color(self.result, self.die.sides)
	}
}
