use std::fmt;


impl fmt::Display for crate::data_types::Epoch {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "e{:0>2}:y{:0>2}:m{:0>2}:d{:0>2}",
				self.era, self.year, self.month, self.day)
	}
}

impl From<(i8, i8, i8, i8)> for crate::data_types::Epoch {
	fn from(d: (i8, i8, i8, i8)) -> crate::data_types::Epoch {
		crate::data_types::Epoch::new(d.0, d.1, d.2, d.3)
	}
}
