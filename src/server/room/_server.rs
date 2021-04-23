use std::error::Error;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::AsyncWriteExt;
use crate::server::room::cmd::property::{
	create_property_update_command,
	create_property_request_command
};
use tokio_util::codec::{BytesCodec, Decoder};
use tokio_stream::StreamExt;
use crate::server::cmd::text::{create_text_command_with_action, create_text_command};
use std::str::from_utf8;
use crate::server::cmd::buddy_list::create_buddy_list_notify_command;
use crate::server::auto::cmd::room::create_room_id_redirect_command;
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::server::shared::Shared;
use crate::server::peer::Peer;
use std::net::SocketAddr;
use crate::server::room::cmd::session::parse_session_initialization_command;
use crate::server::parser::get_commands_from_buffer;
use crate::server::cmd::property::parse_property_set_command;


pub struct RoomServer;
impl RoomServer {
	pub async fn listen(addr: &str) -> Result<(), Box<dyn Error>> {
		let listener = TcpListener::bind(addr).await?;
		debug!("RoomServer now listening on {}", listener.local_addr().unwrap());
		let state = Arc::new(Mutex::new(Shared::new()));
		let mut counter = 0;

		loop {
			let (stream, address) = listener.accept().await?;
			counter += 1;
			let state = Arc::clone(&state);

			tokio::spawn(async move {
				if let Err(e) = RoomServer::handle(
					state,
					stream,
					address,
					counter
				).await {
					error!("an error occurred: {}", e);
				}
			});
		}
	}

	pub async fn handle(
		state: Arc<Mutex<Shared>>,
		stream: TcpStream,
		address: SocketAddr,
		count: usize,
	) -> Result<(), Box<dyn Error>> {
		let bytes = BytesCodec::new().framed(stream);
		let mut peer = Peer::new(state.clone(), bytes, count.to_string()).await?;
		debug!("registered peer with address '{}' as '{}'", address, count);
		let mut room_ids: Vec<String> = Vec::new();
		let mut username: String = String::new();

		loop {
			tokio::select! {
				Some(msg) = peer.rx.recv() => {
					// debug!("received bytes from peer: {:?}", &msg);
					peer.bytes.get_mut().write_all(&msg).await?;
				}
				result = peer.bytes.next() => match result {
					Some(Ok(msg)) => {
						// let msg: BytesMut = msg;
						for msg in get_commands_from_buffer(msg) {
							match msg.get(2).unwrap() {
								10 => { // PROPREQ
									debug!("received property request command from client");
									peer.bytes.get_mut()
										.write_all(&create_property_update_command()).await?;
									debug!("sent property update command to client");
								}
								6 => { // SESSINIT
									username = parse_session_initialization_command(msg.clone()).username;
									debug!(
										"received session initialization command from client: {}",
										username
									);
									peer.bytes.get_mut()
										.write_all(&create_property_request_command()).await?;
									debug!("sent session initialization command to client");
								}
								15 => { // PROPSET
									let avatar = parse_property_set_command(msg.clone());
									debug!("received property set command from client: {}", avatar);
									peer.bytes.get_mut()
										.write_all(&create_text_command_with_action(
											"WORLDSMASTER", &std::env::var("WORLDSMASTER_GREETING")?
										)).await?;
									debug!("sent worldsmaster greeting to client");
								}
								29 => { // BUDDYLISTUPDATE
									let received_buddy = from_utf8(
										msg.get(4..msg.get(0).unwrap().to_owned() as usize - 1).unwrap()
									).unwrap();
									debug!(
										"received buddy list update command from client: {}",
										received_buddy
									);
									peer.bytes.get_mut()
										.write_all(&create_buddy_list_notify_command(received_buddy))
										.await?;
									debug!("sent buddy list notify command to client: {}", received_buddy);
								}
								20 => { // ROOMIDRQ
									let room_name = from_utf8(
										msg.get(4..msg.get(0).unwrap().to_owned() as usize).unwrap()
									).unwrap();
									debug!("received room id request command from client: {}", room_name);
									let room_id;
									if !room_ids.contains(&room_name.to_string()) {
										room_ids.push(room_name.to_string());
										room_id = room_ids.iter()
											.position(|i| i == &room_name.to_string())
											.unwrap();
										trace!("inserted room '{}' as '{}'", room_name, room_id);
									} else {
										let position = room_ids.iter()
											.position(|i| i == &room_name.to_string())
											.unwrap();
										trace!("found room '{}' as '{}'", room_name, position);
										room_id = position;
									}
									trace!("room name: {}, room id: {}", room_name, room_id);
									trace!("{:?}", room_ids);
									peer.bytes.get_mut()
										.write_all(&create_room_id_redirect_command(room_name, room_id))
										.await?;
									debug!("sent redirect id command to client: {} == {}", room_name, room_id);
								}
								14 => {
									let text = from_utf8(
										msg.get(6..msg.get(0).unwrap().to_owned() as usize).unwrap()
									).unwrap();
									debug!("received text command from client: {}", text);
									let mut state = state.lock().await;
									state.broadcast(&create_text_command(&username, text)).await;
									debug!("broadcasted text command from client");
								}
								7 => { // SESSEXIT
									debug!("received session exit command from client")
								}
								_ => (),
							}
						}
					}
					Some(Err(e)) => {
						error!("error while processing messages: {}", e); break;
					}
					None => break,
				}
			}
		}

		{ // De-register client
			state.lock().await.peers.remove(&count.to_string());
			debug!("removed peer: {}", count)
		}

		Ok(())
	}
}
