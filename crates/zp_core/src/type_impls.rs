

impl crate::data_types::Epoch {
	pub fn new(e: i8, y: i8, m: i8, d: i8) -> Self {
		Self {
			era: e,
			year: y,
			month: m,
			day: d,
		}
	}
}