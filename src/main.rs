use whirl::server::auto::server::AutoServer;
use std::error::Error;
use whirl::server::room::server::RoomServer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
	dotenv::dotenv().ok();
	pretty_env_logger::init();

	let mut threads = vec![];
	threads.push(tokio::spawn(async move {
		let _ = AutoServer::new("0.0.0.0:6650").await;
	}));
	threads.push(tokio::spawn(async move {
		let _ = RoomServer::new("0.0.0.0:5673").await;
	}));
	for thread in threads {
		let _ = thread.await;
	}

	Ok(())
}