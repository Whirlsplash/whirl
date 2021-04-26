use std::error::Error;
use crate::server::auto::cmd::property::{
	create_property_update_command,
	create_property_request_command
};
use crate::server::cmd::text::{create_text_command_with_action, create_text_command};
use std::str::from_utf8;
use crate::server::cmd::buddy_list::create_buddy_list_notify_command;
use crate::server::auto::cmd::room::create_room_id_redirect_command;
use crate::server::room::cmd::session::parse_session_initialization_command;
use crate::server::parser::get_commands_from_buffer;
use crate::server::cmd::property::parse_property_set_command;
use crate::config::get_config;
use mio::{Poll, Events, Token, Interest, Registry};
use mio::net::{TcpListener, TcpStream};
use std::collections::{HashMap, HashSet};
use mio::event::Event;
use std::io::{Read, ErrorKind, Write};
use bytes::BytesMut;

pub struct RoomServer {
	pub clients: HashMap<Token, String>,
	pub connections: HashMap<Token, TcpStream>,
	pub room_ids: Vec<String>,
}
impl RoomServer {
	pub fn listen(&mut self, addr: &str) -> Result<(), Box<dyn Error>> {
		let mut listener = TcpListener::bind(addr.parse().unwrap())?;
		let mut poll = Poll::new()?;
		let mut events = Events::with_capacity(1024);
		let mut counter: usize = 0;
		// let mut sockets = HashMap::new();
		let mut requests = HashMap::new();
		let mut buffer = [0 as u8; 1024];
		// let mut room_ids = vec![];

		poll.registry().register(
			&mut listener,
			Token(0),
			Interest::READABLE,
		)?;

		debug!("RoomServer now listening on {}", listener.local_addr().unwrap());

		loop {
			poll.poll(&mut events, None)?;

			for event in &events {
				match event.token() {
					Token(0) => loop {
						match listener.accept() {
							Ok((mut stream, address)) => {
								counter += 1;
								let token = Token(counter);

								poll.registry().register(
									&mut stream,
									token,
									Interest::READABLE,
								)?;

								debug!("registered peer with address '{}' as '{}'", address, token.0);

								// sockets.insert(token, stream);
								self.connections.insert(token, stream);
								requests.insert(token, Vec::with_capacity(192));
							}
							Err(ref err) if err.kind() == ErrorKind::WouldBlock => break,
							Err(err) => {
								error!("unexpected error: {}", err);
								poll.registry().deregister(
									self.connections.get_mut(&Token(counter)).unwrap(),
								)?;
								break;
							}
						}
					},
					token if event.is_readable() => {
						loop {
							let read = self.connections.get_mut(&token).unwrap()
								.read(&mut buffer);
							match read {
								Ok(0) => { self.connections.remove(&token); break; }
								Ok(n) => {
									let req = requests.get_mut(&token).unwrap();
									for b in &buffer[0..n] { req.push(*b); }

									for cmd in get_commands_from_buffer(BytesMut::from(&buffer[..n])) {
										match cmd.get(2).unwrap() {
											10 => { // PROPREQ
												debug!("received property request command from client 'null'");
												self.connections.get_mut(&token).unwrap()
													.write_all(&create_property_update_command()).unwrap();
												debug!("sent property update command to client 'null'");
											}
											6 => { // SESSINIT
												let local_username =
													parse_session_initialization_command(cmd).username;
												self.clients.insert(token, local_username.clone());
												debug!(
													"received session initialization command from client '{}'",
													local_username,
												);
												self.connections.get_mut(&token).unwrap()
													.write_all(&create_property_request_command()).unwrap();
												debug!("sent session initialization command to client '{}'", local_username);
											}
											15 => { // PROPSET
												let avatar = parse_property_set_command(cmd);
												debug!(
													"received property set command from client '{}': {}",
													self.clients.get(&token).unwrap(),
													avatar,
												);
												self.connections.get_mut(&token).unwrap()
													.write_all(&create_text_command_with_action(
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
													received_buddy,
												);
												self.connections.get_mut(&token).unwrap()
													.write_all(&create_buddy_list_notify_command(received_buddy)).unwrap();
												debug!(
													"sent buddy list notify command to client '{}'",
													self.clients.get(&token).unwrap(),
												);
											}
											// 20 => { // ROOMIDRQ
											// 	let room_name = from_utf8(
											// 		cmd.get(4..cmd.get(0).unwrap().to_owned() as usize).unwrap()
											// 	).unwrap();
											// 	debug!(
											// 		"received room id request command from client '{}': {}",
											// 		self.clients.get(&token).unwrap(),
											// 		room_name,
											// 	);
											// 	let room_id;
											// 	if !self.room_ids.contains(&room_name.to_string()) {
											// 		self.room_ids.push(room_name.to_string());
											// 		room_id = self.room_ids.iter()
											// 			.position(|i| i == &room_name.to_string())
											// 			.unwrap();
											// 		trace!("inserted room '{}' as '{}'", room_name, room_id);
											// 	} else {
											// 		let position = self.room_ids.iter()
											// 			.position(|i| i == &room_name.to_string())
											// 			.unwrap();
											// 		trace!("found room '{}' as '{}'", room_name, position);
											// 		room_id = position;
											// 	}
											// 	trace!("room name: {}, room id: {}", room_name, room_id);
											// 	trace!("{:?}", self.room_ids);
											// 	self.connections.get_mut(&token).unwrap()
											// 		.write_all(&create_room_id_redirect_command(
											// 			room_name, room_id,
											// 		)).unwrap();
											// }
											14 => { // TEXT
												let text = from_utf8(
													cmd.get(6..cmd.get(0).unwrap().to_owned() as usize).unwrap()
												).unwrap();
												let username = self.clients.get(&token).unwrap().clone();
												debug!(
													"received text command from client '{}': {}",
													username, format!("room: {}", text),
												);
												self.connections.iter_mut().for_each(|t|
													t.1.write_all(&create_text_command(
														&username,
														text,
													)).unwrap()
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
								Err(ref err) if err.kind() == ErrorKind::WouldBlock =>
									break,
								Err(err) => { error!("unexpected error: {}", err); break; }
							}
						}
					}
					_ => (),
				}
			}
		}
	}

	fn broadcast(
		sockets: &HashMap<Token, TcpStream>,
		cmd: &[u8],
	) -> () {
		for mut socket in sockets {
			socket.1.write_all(cmd).unwrap();
		}
	}

	// fn process(
	// 	&mut self,
	// 	_registry: &Registry,
	// 	event: &Event,
	// 	token: Token,
	// ) -> Result<bool, Box<dyn Error>> {
	// 	if event.is_readable() {
	// 		let mut connection_closed = false;
	// 		let mut received_data = vec![0; 4096];
	// 		let mut bytes_read = 0;
	//
	// 		let stream = self.connections.get_mut(&token).unwrap();
	//
	// 		loop {
	// 			match stream.read(&mut received_data[bytes_read..]) {
	// 				Ok(0) => {
	// 					connection_closed = true;
	// 					break;
	// 				}
	// 				Ok(n) => {
	// 					bytes_read += n;
	// 					if bytes_read ==  received_data.len() {
	// 						received_data.resize(received_data.len() + 1024, 0);
	// 					}
	// 				}
	// 				Err(ref err) if err.kind() == ErrorKind::WouldBlock => break,
	// 				Err(ref err) if err.kind() == ErrorKind::Interrupted => continue,
	// 				Err(err) => return Err(Box::new(err)),
	// 			}
	// 		}
	//
	// 		if bytes_read != 0 {
	// 			self.handle(
	// 				&mut received_data[..bytes_read],
	// 				token,
	// 			);
	// 		}
	// 		if connection_closed {
	// 			println!("de-registered peer with token '{}'", token.0);
	// 			return Ok(true);
	// 		}
	// 	}
	//
	// 	Ok(false)
	// }

	// fn handle(
	// 	&mut self,
	// 	data: &[u8],
	// 	// stream: &mut TcpStream,
	// 	token: Token,
	// ) -> () {
	// 	// trace!("i am client: {:?}", self.clients.get(&token));
	// 	// debug!("{:?}", self.connections);
	// 	for cmd in get_commands_from_buffer(BytesMut::from(data)) {
	// 		debug!("received: {:?}", cmd);
	// 		match cmd.get(2).unwrap() {
	// 			10 => { // PROPREQ
	// 				debug!("received property request command from client 'null'");
	// 				self.connections.get_mut(&token).unwrap()
	// 					.write_all(&create_property_update_command()).unwrap();
	// 				debug!("sent property update command to client 'null'");
	// 			}
	// 			6 => { // SESSINIT
	// 				let local_username =
	// 					parse_session_initialization_command(cmd).username;
	// 				self.clients.insert(token, local_username.clone());
	// 				debug!(
	// 					"received session initialization command from client '{}'",
	// 					local_username,
	// 				);
	// 				self.connections.get_mut(&token).unwrap()
	// 					.write_all(&create_property_request_command()).unwrap();
	// 				debug!("sent session initialization command to client '{}'", local_username);
	// 			}
	// 			15 => { // PROPSET
	// 				let avatar = parse_property_set_command(cmd);
	// 				debug!(
	// 					"received property set command from client '{}': {}",
	// 					self.clients.get(&token).unwrap(),
	// 					avatar,
	// 				);
	// 				self.connections.get_mut(&token).unwrap()
	// 					.write_all(&create_text_command_with_action(
	// 						"WORLDSMASTER", &get_config().unwrap().worldsmaster_greeting,
	// 					)).unwrap();
	// 				debug!(
	// 					"sent session initialization command to client '{}'",
	// 					self.clients.get(&token).unwrap(),
	// 				);
	// 			}
	// 			29 =>  { // BUDDYLISTUPDATE
	// 				let received_buddy = from_utf8(
	// 					cmd.get(4..cmd.get(0).unwrap().to_owned() as usize - 1).unwrap()
	// 				).unwrap();
	// 				debug!(
	// 					"received buddy list update command from client '{}': {}",
	// 					self.clients.get(&token).unwrap(),
	// 					received_buddy,
	// 				);
	// 				self.connections.get_mut(&token).unwrap()
	// 					.write_all(&create_buddy_list_notify_command(received_buddy)).unwrap();
	// 				debug!(
	// 					"sent buddy list notify command to client '{}'",
	// 					self.clients.get(&token).unwrap(),
	// 				);
	// 			}
	// 			20 => { // ROOMIDRQ
	// 				let room_name = from_utf8(
	// 					cmd.get(4..cmd.get(0).unwrap().to_owned() as usize).unwrap()
	// 				).unwrap();
	// 				debug!(
	// 					"received room id request command from client '{}': {}",
	// 					self.clients.get(&token).unwrap(),
	// 					room_name,
	// 				);
	// 				let room_id;
	// 				if !self.room_ids.contains(&room_name.to_string()) {
	// 					self.room_ids.push(room_name.to_string());
	// 					room_id = self.room_ids.iter()
	// 						.position(|i| i == &room_name.to_string())
	// 						.unwrap();
	// 					trace!("inserted room '{}' as '{}'", room_name, room_id);
	// 				} else {
	// 					let position = self.room_ids.iter()
	// 						.position(|i| i == &room_name.to_string())
	// 						.unwrap();
	// 					trace!("found room '{}' as '{}'", room_name, position);
	// 					room_id = position;
	// 				}
	// 				trace!("room name: {}, room id: {}", room_name, room_id);
	// 				trace!("{:?}", self.room_ids);
	// 				self.connections.get_mut(&token).unwrap()
	// 					.write_all(&create_room_id_redirect_command(
	// 						room_name, room_id,
	// 					)).unwrap();
	// 			}
	// 			14 => { // TEXT
	// 				let text = from_utf8(
	// 					cmd.get(6..cmd.get(0).unwrap().to_owned() as usize).unwrap()
	// 				).unwrap();
	// 				let username = self.clients.get(&token).unwrap().clone();
	// 				debug!(
	// 					"received text command from client '{}': {}",
	// 					username, text,
	// 				);
	// 				self.connections.iter_mut().for_each(|t|
	// 					t.1.write_all(&create_text_command(
	// 						&username,
	// 						text,
	// 					)).unwrap()
	// 				);
	// 				debug!("broadcasted text command to clients");
	// 			}
	// 			7 => { // SESSEXIT
	// 				debug!(
	// 					"received session exit command from client '{}'",
	// 					self.clients.get(&token).unwrap(),
	// 				);
	// 			}
	// 			_ => (),
	// 		}
	// 	}
	// }
}
