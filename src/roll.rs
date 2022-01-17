use owo_colors::{OwoColorize, Rgb};
use rand::Rng;

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
		let interpolation = ((self.result as f32) / (self.die.sides as f32) * 256.) as u8;
		let red = 255 - interpolation;
		let green = interpolation;
		let color = Rgb(red, green, 0);
		self.result.color(color).to_string()
	}
}
