use tracing::*;

fn main() {
	ready_z_go::app_tracing::init_debug_tools("example-name=debug").unwrap();
	debug!("Logging started");

	info!("Hello, world!");
}
