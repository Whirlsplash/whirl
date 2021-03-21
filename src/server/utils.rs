use std::collections::HashMap;
use mio::Token;
use mio::net::TcpStream;
use std::io::Write;

pub fn broadcast_to_all_clients(
	sockets: &HashMap<Token, TcpStream>,
	message: &[u8]
) -> () {
	for mut socket in sockets {
		socket.1.write_all(message).unwrap();
	}
}
