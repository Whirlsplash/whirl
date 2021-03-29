use whirl::server::auto::server::AutoServer;
use std::error::Error;
use whirl::server::room::server::RoomServer;
use structopt::StructOpt;
use whirl::config;
use whirl::cli::Command;
use whirl::config::get_config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
	let opt = whirl::cli::Opt::from_args();

	// Set logging level
	let mut log_level = "whirl=error,whirl=warn,whirl=info".to_string();
	if opt.debug { log_level += ",whirl=debug"; }
	if opt.verbose >= 1 { log_level += ",whirl=trace"; };
	std::env::set_var("RUST_LOG", log_level);

	// Set database URL
	std::env::set_var("DATABASE_URL", "sqlite:whirl.db");

	// Setup logging
	dotenv::dotenv().ok();
	pretty_env_logger::init();

	// Handle CLI command
	match opt.command {
		Command::Run => run().await.unwrap(),
		Command::Config => println!("{:#?}", config::get_config()),
	}

	Ok(())
}

async fn run() -> Result<(), Box<dyn Error>> {
	let mut threads = vec![];
	threads.push(tokio::spawn(async move {
		let _ = AutoServer::new(
			&*format!("0.0.0.0:{}", get_config().unwrap().auto_server_port)
		).await;
	}));
	threads.push(tokio::spawn(async move {
		let _ = RoomServer::new(
			&*format!("0.0.0.0:{}", get_config().unwrap().room_server_port)
		).await;
	}));
	for thread in threads {
		let _ = thread.await;
	}

	Ok(())
}
