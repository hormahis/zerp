mod data_types;
mod type_impls;
mod type_impls_std;



pub mod prelude {
	
	pub use tracing::{info, error, warn, debug, trace};

	pub fn init_logging() {
		// Init subscriber
		let subscriber = tracing_subscriber::FmtSubscriber::builder()
			.with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
			.with_target(true)
			.finish();

		// Init tracing
		tracing::subscriber::set_global_default(subscriber)
			.expect("Failed to set default subscriber");
		
	}

	pub use crate::data_types::Epoch;

}
