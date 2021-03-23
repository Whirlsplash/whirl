#[macro_use]
extern crate log;

use mio::net::TcpListener;
use std::thread;
use whirl::server;

fn main() {
	dotenv::dotenv().ok(); // Adds ability to use environment variables.
	pretty_env_logger::init(); // Adds pretty logging.

	let mut threads = vec![];
	threads.push(thread::spawn(move || {
		debug!("spawned WorldServer thread");
		server::world::server::WorldServer::new(
			TcpListener::bind(
				&"0.0.0.0:6650".parse().unwrap()
			).unwrap()
		);
	}));
	threads.push(thread::spawn(move || {
		debug!("spawned AutoServer thread");
		server::auto::server::AutoServer::new(
			TcpListener::bind(
				&"0.0.0.0:5673".parse().unwrap()
			).unwrap()
		);
	}));
	for thread in threads {
		let _ = thread.join();
	}
}
