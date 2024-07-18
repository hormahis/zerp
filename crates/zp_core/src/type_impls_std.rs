use std::fmt;
use auto_ops::*;


use crate::data_types::Epoch;


impl fmt::Display for Epoch {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "e{:0>2}:y{:0>2}:m{:0>2}:d{:0>2}",
				self.era, self.year, self.month, self.day)
	}
}

impl From<(u32, u32, u8, u8)> for Epoch {
	fn from(d: (u32, u32, u8, u8)) -> Epoch {
		Epoch::new(d.0, d.1, d.2, d.3, 1000, 12, 30)
	}
}


#[inline]
fn epoch_calc_adds(a: &Epoch, b: &Epoch) -> (u32, u32, u32, u32) {
	// Deal with days
	let cnd = (a.day + b.day) as u32 % a.max_day as u32;
	let cfd = (a.day + b.day) as u32 / a.max_day as u32;

	// .. months
	let cnm = ((a.month + b.month) as u32 + cfd) % a.max_month as u32;
	let cfm = ((a.month + b.month) as u32 + cfd) / a.max_month as u32;

	// and years
	let cny = (a.year + b.year + cfm as u32) % a.max_year;
	let cfy = (a.year + b.year + cfm as u32) / a.max_year;

	(cnd, cnm, cny, cfy)
}

// REF + REF
impl_op!(+ |a: &Epoch, b: &Epoch| -> Epoch {
	let adds = epoch_calc_adds(a, b);

	Epoch {
		day: adds.0 as u8,
		month: adds.1 as u8,
		year: adds.2,
		era: a.era + b.era + adds.3,

		max_day: a.max_day,
		max_month: a.max_month,
		max_year: a.max_year,
	}
});

// VAL + VAL
impl_op!(+ |a: Epoch, b: Epoch| -> Epoch {
	let adds = epoch_calc_adds(&a, &b);

	Epoch {
		day: adds.0 as u8,
		month: adds.1 as u8,
		year: adds.2,
		era: a.era + b.era + adds.3,

		max_day: a.max_day,
		max_month: a.max_month,
		max_year: a.max_year,
	}
});

// VAL <+> REF
impl_op_commutative!(+ |a: &Epoch, b: Epoch| -> Epoch {
	let adds = epoch_calc_adds(a, &b);

	Epoch {
		day: adds.0 as u8,
		month: adds.1 as u8,
		year: adds.2,
		era: a.era + b.era + adds.3,

		max_day: a.max_day,
		max_month: a.max_month,
		max_year: a.max_year,
	}
});

// SELF += REF
impl_op!(+= |a: &mut Epoch, b: &Epoch| {
	let adds = epoch_calc_adds(a, b);

	a.day = adds.0 as u8;
	a.month = adds.1 as u8;
	a.year = adds.2;
	a.era += b.era + adds.3;
});

// SELF += VAL
impl_op!(+= |a: &mut Epoch, b: Epoch| {
	let adds = epoch_calc_adds(a, &b);

	a.day = adds.0 as u8;
	a.month = adds.1 as u8;
	a.year = adds.2;
	a.era += b.era + adds.3;
});
