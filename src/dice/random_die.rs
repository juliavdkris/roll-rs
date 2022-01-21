use rand::Rng;

use super::DieRoll;

pub struct RandomDie {
	sides: u32
}


impl super::Die for RandomDie {
	fn roll(self) -> DieRoll<Self> {
		let mut rng = rand::thread_rng();
		let result = rng.gen_range(1..=self.sides);
		DieRoll::new(self, result)
	}
}


impl RandomDie {
    pub fn new(sides: u32) -> Self {
        Self { sides }
    }
}
