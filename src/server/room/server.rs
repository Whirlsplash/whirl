use mio::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use mio::{Poll, Token, Ready, PollOpt, Events};
use std::collections::HashMap;
use std::str::from_utf8;
use crate::cmd::buddy_list::create_buddy_list_notify_command;
use crate::cmd::text::create_text_command;
// use crate::cmd::property::{create_property_update_command, create_property_request_command};
use crate::server::room::cmd::property::{create_property_update_command, create_property_request_command};
use rand::Rng;
use crate::server::utils::broadcast_to_all_clients;

pub struct RoomServer;
impl RoomServer {
	pub fn new(listener: TcpListener) {
		let poll = Poll::new().unwrap();
		poll.register(
			&listener,
			Token(0),
			Ready::readable(),
			PollOpt::edge()
		).unwrap();

		let mut counter: usize = 0;
		let mut sockets: HashMap<Token, TcpStream> = HashMap::new();
		let mut requests: HashMap<Token, Vec<u8>> = HashMap::new();
		let mut buffer = [0 as u8; 1024];

		let mut events = Events::with_capacity(1024);
		loop {
			poll.poll(&mut events, None).unwrap();
			for event in &events {
				match event.token() {
					Token(0) => {
						loop {
							match listener.accept() {
								Ok((socket, address)) => {
									counter += 1;
									let token = Token(counter);

									poll.register(
										&socket,
										token,
										Ready::readable(),
										PollOpt::edge()
									).unwrap();

									info!(
										"registered client with ip '{}' as token '{}'",
										address.ip(), token.0
									);

									sockets.insert(token, socket);
									requests.insert(token, Vec::with_capacity(192));
								}
								Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock =>
									break,
								Err(e) => {
									error!("unexpected error: {}", e);
									poll.deregister(sockets.get(&Token(counter)).unwrap()).unwrap();
									break;
								}
							}
						}
					},
					token if event.readiness().is_readable() => {
						loop {
							let read = sockets.get_mut(&token).unwrap().read(&mut buffer);
							match read {
								Ok(0) => { sockets.remove(&token); break; }
								Ok(n) => {
									let req = requests.get_mut(&token).unwrap();
									for b in &buffer[0..n] {
										req.push(*b);
									}

									// First byte means how long data section of the packet is.
									// Second byte is to be determined.
									// Third byte is the request type.

									// Match packet type by descriptor; **third** byte.
									match &buffer.get(2).unwrap() { // Third byte is 2 because Rust is zero-indexed.
										// PROPREQ
										10 => {
											debug!(
												"received property request command from client with token '{}'",
												token.0
											);
											sockets.get_mut(&token).unwrap()
												.write_all(&create_property_update_command()).unwrap();
											debug!(
												"sent property update from client with token '{}'",
												token.0
											);
										}
										// SESSINIT
										6 => {
											debug!(
												"received session initialization command from client with token '{}'",
												token.0
											);
											sockets.get_mut(&token).unwrap()
												.write_all(&create_property_request_command()).unwrap();
											debug!(
												"sent session initialization command from client with token '{}'",
												token.0
											);
										}
										// PROPSET
										15 => debug!(
											"received property set command from client with token '{}'",
											token.0
										),
										// BUDDYLISTUPDATE
										29 => {
											debug!(
												"received buddy list update command from client with token '{}'",
												token.0
											);
											sockets.get_mut(&token).unwrap()
												.write_all(&create_buddy_list_notify_command("Wirlaburla"))
												.unwrap();
											debug!(
												"sent buddy notify update command from client with token '{}'",
												token.0
											);
										}
										// ROOMIDRQ
										20 => debug!(
											"received room id request command from client with token '{}'",
											token.0
										),
										// TEXT
										14 => {
											debug!(
												"received text command from client with token '{}'",
												token.0
											);

											// TODO: Make this into a command!
											let message = from_utf8(
												&buffer[6..*&buffer.get(0).unwrap().to_owned() as usize]
											).unwrap();
											trace!("message: {}", message);

											// Using User as a placeholder. Ideally, this would print out the username of
											// the one who sent it.
											broadcast_to_all_clients(
												&sockets,
												&create_text_command(
													// Random integer is added to the end of "User", just a development
													// proof-of-concept. Since at this stage usernames aren't exactly kept,
													// we can identify message senders as their connection token; `token.0`.
													&format!("User{}", rand::thread_rng().gen_range(1..150).to_string()),
													message
												)
											);
											debug!(
												"broadcasted text command from client with token '{}'",
												token.0
											);
										}
										// SESSEXIT
										7 => {
											debug!(
												"received session termination command from client with token '{}'",
												token.0
											);
											poll.deregister(sockets.get(&token).unwrap()).unwrap();
											debug!("de-registered client with token '{}'", token.0);
										}
										// SUBSCRIB
										16 => {
											debug!(
												"received room subscription command from client with token '{}'",
												token.0
											);
										}
										// Anything else, do nothing.
										_ => ()
									}
								}
								Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock =>
									break,
								Err(e) => { error!("unexpected error: {}", e); break; }
							}
						}

						// Unimplemented
						// let ready = requests.get(&token).unwrap()
						// 	.windows(4)
						// 	.find(|window| is_double_crnl(*window))
						// 	.is_some();
						//
						// if ready {
						// 	let socket = sockets.get(&token).unwrap();
						// 	poll.reregister(
						// 		socket,
						// 		token,
						// 		Ready::writable(),
						// 		PollOpt::edge() | PollOpt::oneshot()).unwrap();
						// }
					},
					// Unimplemented
					// token if event.readiness().is_writable() => {
					// 	println!("writeable");
					// 	requests.get_mut(&token).unwrap().clear();
					// 	sockets.get_mut(&token).unwrap().write_all("test".as_bytes()).unwrap();
					//
					// 	// Re-use existing connection ("keep-alive") - switch back to reading
					// 	poll.reregister(
					// 		sockets.get(&token).unwrap(),
					// 		token,
					// 		Ready::readable(),
					// 		PollOpt::edge()).unwrap();
					// },
					_ => ()
				}
			}
		}
	}
}
