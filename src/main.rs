#[macro_use]
extern crate log;

use mio::net::TcpListener;
use whirl::server;

fn main() {
	dotenv::dotenv().ok(); // Adds ability to use environment variables.
	pretty_env_logger::init(); // Adds pretty logging.

	std::thread::spawn(|| {
		server::world::WorldServer::new(
			TcpListener::bind(
				&"0.0.0.0:6650".parse().unwrap()
			).unwrap()
		);
	}).join().unwrap();
	debug!("spawned WorldServer thread");

	// POC, unimplemented.
	// std::thread::spawn(move || {
	// 	server::auto::AutoServer::new(
	// 		TcpListener::bind(
	// 			&"0.0.0.0:1337".parse().unwrap()
	// 		).unwrap()
	// 	);
	// });
	// debug!("spawned AutoServer thread");
}
