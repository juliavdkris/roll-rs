use rand::Rng;

#[derive(PartialEq, Debug)]
pub struct Die {
	pub sides: u32,
}

pub struct DieRoll<'a> {
	pub die: &'a Die,
	pub result: u32,
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
