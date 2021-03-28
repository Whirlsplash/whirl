use crate::server::cmd::session::SessionInitializationCommand;
use bytes::BytesMut;
use std::str::from_utf8;

struct SessionInitializationCommandServer {
	pub error: usize,
	pub app_name: String,
	pub protocol: usize,
	pub server_type: usize,
	pub serial: String,
	pub private: usize,
	pub channel: String,
}

pub fn parse_session_initialization_command(
	command: BytesMut
) -> SessionInitializationCommand {
	SessionInitializationCommand {
		// protocol: command.get(4..4 + command.get(4)).unwrap().to_owned() as usize,
		// client: "".to_string(),
		username: from_utf8(
			command.get(
				21..(20 + command.get(20).unwrap().to_owned() as usize + 1)
			).unwrap()
		).unwrap().to_string(),
		// password: "".to_string()
	}
}

// pub fn create_session_initialization_command() -> SessionInitializationCommandServer {
// 	SessionInitializationCommandServer {
//
// 	}
// }
