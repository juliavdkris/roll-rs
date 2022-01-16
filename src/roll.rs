use rand::Rng;

pub struct DiceRoll {
	pub sides: u8,
}

pub struct DiceRollResult<'a> {
	pub roll: &'a DiceRoll,
	pub result: u8,
}

impl DiceRoll {
	pub fn new(sides: u8) -> Self {
		Self { sides }
	}

	pub fn roll(&self) -> DiceRollResult {
		let mut rng = rand::thread_rng();
		let result = rng.gen_range(1..=self.sides);
		DiceRollResult { roll: self, result }
	}
}
