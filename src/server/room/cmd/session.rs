use crate::server::cmd::session::SessionInitializationCommand;
use bytes::BytesMut;
use std::str::from_utf8;

pub fn parse_session_initialization_command(
	command: BytesMut
) -> SessionInitializationCommand {
	SessionInitializationCommand {
		// protocol: command.get(4..4 + command.get(4)).unwrap().to_owned() as usize,
		// client: "".to_string(),
		username: from_utf8(
			command.get(
				25..(24 + command.get(24).unwrap().to_owned() as usize + 1)
			).unwrap()
		).unwrap().to_string(),
		// password: "".to_string()
	}
}
