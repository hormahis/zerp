
#[derive(Debug, Clone, PartialEq)]
pub struct Epoch {
	pub era: u32,
	pub year: u32,
	pub month: u8,
	pub day: u8,
	
	pub max_year: u32,
	pub max_month: u8,
	pub max_day: u8,
}
