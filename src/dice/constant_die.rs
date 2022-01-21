use super::DieRoll;


pub struct ConstantDie {
	num: u32
}


impl super::Die for ConstantDie {
	fn roll(self) -> DieRoll<Self> {
		let num = self.num;
		DieRoll::new(self, num)
	}

	fn max_result(&self) -> u32 {
		self.num
	}
}


impl ConstantDie {
	pub fn new(num: u32) -> Self {
		Self { num }
	}
}
