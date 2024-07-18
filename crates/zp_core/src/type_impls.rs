

impl crate::data_types::Epoch {
	pub fn new(era: u32, year: u32, month: u8, day: u8, max_year: u32, max_month: u8, max_day: u8) -> Self {
		Self {
			era,
			year,
			month,
			day,
			max_year,
			max_month,
			max_day,
		}
	}
}