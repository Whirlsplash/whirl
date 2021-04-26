use std::error::Error;
use crate::server::auto::cmd::property::{
	create_property_update_command,
	create_property_request_command
};
use crate::server::cmd::text::{create_text_command_with_action, create_text_command};
use std::str::from_utf8;
use crate::server::cmd::buddy_list::create_buddy_list_notify_command;
use crate::server::auto::cmd::room::create_room_id_redirect_command;
use crate::server::auto::cmd::session::parse_session_initialization_command;
use crate::server::parser::get_commands_from_buffer;
use crate::server::cmd::property::parse_property_set_command;
use crate::config::get_config;
use mio::{Poll, Events, Token, Interest, Registry};
use mio::net::{TcpListener, TcpStream};
use std::collections::HashMap;
use mio::event::Event;
use std::io::{Read, ErrorKind, Write};
use bytes::BytesMut;

const SERVER: Token = Token(0);

pub struct AutoServer {
	pub clients: HashMap<Token, String>,
	pub connections: HashMap<Token, TcpStream>,
}
impl AutoServer {
	pub fn listen(&mut self, addr: &str) -> Result<(), Box<dyn Error>> {
		let mut server = TcpListener::bind(addr.parse().unwrap())?;
		let mut poll = Poll::new()?;
		let mut events = Events::with_capacity(1024);
		let mut connections = HashMap::new();
		let mut unique_token = Token(SERVER.0 + 1);

		poll.registry().register(
			&mut server,
			SERVER,
			Interest::READABLE
		)?;

		debug!("AutoServer now listening on {}", server.local_addr().unwrap());

		loop {
			poll.poll(&mut events, None)?;

			for event in events.iter() {
				match event.token() {
					SERVER => loop {
						let (mut stream, address) = match server.accept() {
							Ok((stream, address)) => (stream, address),
							Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => break,
							Err(e) => return Err(Box::new(e)),
						};

						let token = AutoServer::next(&mut unique_token);
						poll.registry().register(
							&mut stream,
							token,
							Interest::READABLE, //.add(Interest::WRITABLE),
						)?;

						connections.insert(token, stream);

						println!("registered peer with address '{}' as '{}'", address, token.0);
					},
					token => {
						let done = if let Some(stream) = connections.get_mut(&token) {
							self.process(
								poll.registry(),
								stream,
								event,
								token,
							)?
						} else {
							false
						};
						if done { connections.remove(&token); }
					}
				}
			}
		}
	}

	fn next(current: &mut Token) -> Token {
		let next = current.0;
		current.0 += 1;
		Token(next)
	}

	fn broadcast(self, cmd: &[u8]) -> () {
		for mut connection in self.connections {
			connection.1.write(cmd).unwrap();
		}
	}

	fn process(
		&mut self,
		_registry: &Registry,
		stream: &mut TcpStream,
		event: &Event,
		token: Token,
	) -> Result<bool, Box<dyn Error>> {
		if event.is_readable() {
			let mut connection_closed = false;
			let mut received_data = vec![0; 4096];
			let mut bytes_read = 0;

			loop {
				match stream.read(&mut received_data[bytes_read..]) {
					Ok(0) => {
						connection_closed = true;
						break;
					}
					Ok(n) => {
						bytes_read += n;
						if bytes_read ==  received_data.len() {
							received_data.resize(received_data.len() + 1024, 0);
						}
					}
					Err(ref err) if err.kind() == ErrorKind::WouldBlock => break,
					Err(ref err) if err.kind() == ErrorKind::Interrupted => continue,
					Err(err) => return Err(Box::new(err)),
				}
			}

			if bytes_read != 0 {
				self.handle(
					&mut received_data[..bytes_read],
					stream,
					token,
				);
			}
			if connection_closed {
				debug!("connection closed");
				return Ok(true);
			}
		}

		Ok(false)
	}

	fn handle(
		&mut self,
		data: &[u8],
		stream: &mut TcpStream,
		token: Token,
	) -> () {
		trace!("i am client: {:?}", self.clients.get(&token));
		// let local_client = self.clients.get(&token)
		// 	.unwrap_or(&"null".to_string());
		for cmd in get_commands_from_buffer(BytesMut::from(data)) {
			debug!("received: {:?}", cmd);
			match cmd.get(2).unwrap() {
				10 => { // PROPREQ
					debug!("received property request command from client 'null'");
					stream.write_all(&create_property_update_command()).unwrap();
					debug!("sent property update command to client 'null'");
				}
				6 => { // SESSINIT
					let username =
						parse_session_initialization_command(cmd).username;
					self.clients.insert(token, username.clone());
					debug!(
						"received session initialization command from client '{}'",
						username,
					);
					stream.write_all(&create_property_request_command()).unwrap();
					debug!("sent session initialization command to client '{}'", username);
				}
				15 => { // PROPSET
					let avatar = parse_property_set_command(cmd);
					debug!(
						"received property set command from client '{}': {}",
						self.clients.get(&token).unwrap(),
						avatar
					);
					stream.write_all(&create_text_command_with_action(
						"WORLDSMASTER", &get_config().unwrap().worldsmaster_greeting,
					)).unwrap();
					debug!(
						"sent session initialization command to client '{}'",
						self.clients.get(&token).unwrap(),
					);
				}
				29 =>  { // BUDDYLISTUPDATE
					let received_buddy = from_utf8(
						cmd.get(4..cmd.get(0).unwrap().to_owned() as usize - 1).unwrap()
					).unwrap();
					debug!(
						"received buddy list update command from client '{}': {}",
						self.clients.get(&token).unwrap(),
						received_buddy
					);
					stream.write_all(&create_buddy_list_notify_command(received_buddy)).unwrap();
					debug!(
						"sent buddy list notify command to client '{}'",
						self.clients.get(&token).unwrap(),
					);
				}
				20 => { // ROOMIDRQ
					let room_name = from_utf8(
						cmd.get(4..cmd.get(0).unwrap().to_owned() as usize).unwrap()
					).unwrap();
					debug!(
						"received room id request command from client '{}': {}",
						self.clients.get(&token).unwrap(), room_name,
					);
				}
				14 => { // TEXT
					let text = from_utf8(
						cmd.get(6..cmd.get(0).unwrap().to_owned() as usize).unwrap()
					).unwrap();
					debug!(
						"received text command from client '{}': {}",
						self.clients.get(&token).unwrap(),
						text,
					);
					debug!("broadcasted text command to clients");
				}
				7 => { // SESSEXIT
					debug!(
						"received session exit command from client '{}'",
						self.clients.get(&token).unwrap(),
					);
				}
				_ => (),
			}
		}
	}
}
