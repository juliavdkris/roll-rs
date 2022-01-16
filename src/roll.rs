use rand::Rng;

pub struct Die {
	pub sides: u8,
}

pub struct DieRoll<'a> {
	pub roll: &'a Die,
	pub result: u8,
}

impl Die {
	pub fn new(sides: u8) -> Self {
		Self { sides }
	}

	pub fn roll(&self) -> DieRoll {
		let mut rng = rand::thread_rng();
		let result = rng.gen_range(1..=self.sides);
		DieRoll { roll: self, result }
	}
}
