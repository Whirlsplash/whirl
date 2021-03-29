use whirl::server::auto::server::AutoServer;
use std::error::Error;
use whirl::server::room::server::RoomServer;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
enum Command {
	Run,
}

#[derive(StructOpt, Debug)]
#[structopt(
	name = env!("CARGO_PKG_NAME"),
	about = env!("CARGO_PKG_DESCRIPTION"),
	version = env!("CARGO_PKG_VERSION"),
	author = env!("CARGO_PKG_AUTHORS"),
)]
struct Opt {
	#[structopt(short, long)]
	debug: bool,

	#[structopt(short, long, parse(from_occurrences))]
	verbose: u8,

	#[structopt(subcommand)] // help
	command: Command,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
	let opt = Opt::from_args();

	// Set logging level
	let mut log_level = "whirl=error,whirl=warn,whirl=info".to_string();
	if opt.debug { log_level += ",whirl=debug"; }
	if opt.verbose >= 1 { log_level += ",whirl=trace"; };
	std::env::set_var("RUST_LOG", log_level);

	// Setup logging
	dotenv::dotenv().ok();
	pretty_env_logger::init();

	// Handle CLI command
	match opt.command {
		Command::Run => run().await,
	}

	Ok(())
}

async fn run() {
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
}
