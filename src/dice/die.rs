use super::DieRoll;

pub trait Die {
	fn roll(self) -> DieRoll<Self>
	where
		Self: std::marker::Sized;

	fn max_result(&self) -> u32;
}
