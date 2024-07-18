use zp_core::prelude::*;
use zp_service::prelude::*;




#[actix_web::main]
async fn main() -> ZpServiceResult {


	// Start tracing
	init_logging();

	let mut e1 = Epoch::from((0, 99, 11, 30));
	e1  += Epoch::from((0, 0, 1, 1));
	info!("res: {}", e1);

	// Start main loop
	run_service().await
}
