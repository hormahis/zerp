mod data_types;
mod type_impls;
mod type_impls_std;

pub mod prelude {

	// Support logging init
	use tracing_subscriber::{
		FmtSubscriber,
		EnvFilter,
		filter::LevelFilter
	};

	pub use tracing::{info, error, warn, debug, trace};

	pub fn init_logging() {
		let filter = EnvFilter::builder()
			.with_default_directive(LevelFilter::INFO.into())
			.from_env_lossy();
		let filter_str = filter.to_string().to_uppercase();

		// Init subscriber
		let subscriber = FmtSubscriber::builder()
			.with_env_filter(filter)
			.with_target(true)
			.finish();

		// Init tracing
		tracing::subscriber::set_global_default(subscriber)
			.expect("Failed to set default subscriber");

		println!("Logging initialized with level filter `{filter_str}`");

	}

	pub use crate::data_types::Epoch;

}
