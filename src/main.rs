use zp_core::prelude::*;
use zp_service::prelude::*;




#[actix_web::main]
async fn main() -> ZpServiceResult {


	// Start tracing
	init_logging();


	// Start main loop
	run_service().await
}
