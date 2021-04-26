use std::{
  collections::HashMap,
  error::Error,
  io::{ErrorKind, Read, Write},
  ops::Deref,
  str::from_utf8,
};

use bytes::BytesMut;
use mio::{
  event::Event,
  net::{TcpListener, TcpStream},
  Events,
  Interest,
  Poll,
  Registry,
  Token,
};

use crate::{
  config::get_config,
  server::{
    auto::cmd::{
      property::{create_property_request_command, create_property_update_command},
      room::create_room_id_redirect_command,
      session::parse_session_initialization_command,
    },
    cmd::{
      action::create_action_command,
      buddy_list::create_buddy_list_notify_command,
      property::parse_property_set_command,
      text::{create_text_command, create_text_command_with_action},
    },
    parser::get_commands_from_buffer,
  },
};

const SERVER: Token = Token(0);

pub struct AutoServer {
  pub clients:     HashMap<Token, String>,
  pub connections: HashMap<Token, TcpStream>,
  pub room_ids:    Vec<String>,
}
impl AutoServer {
  pub fn listen(&mut self, addr: &str) -> Result<(), Box<dyn Error>> {
    let mut server = TcpListener::bind(addr.parse().unwrap())?;
    let mut poll = Poll::new()?;
    let mut events = Events::with_capacity(1024);
    let mut unique_token = Token(SERVER.0 + 1);

    poll
      .registry()
      .register(&mut server, SERVER, Interest::READABLE)?;

    debug!(
      "AutoServer now listening on {}",
      server.local_addr().unwrap()
    );

    loop {
      poll.poll(&mut events, None)?;

      for event in &events {
        match event.token() {
          SERVER => {
            loop {
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

              self.connections.insert(token, stream);

              debug!(
                "registered peer with address '{}' as '{}'",
                address, token.0
              );
            }
          }
          token => {
            let done = self.process(poll.registry(), event, token)?;
            if done {
              self.connections.remove(&token);
            }
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

  fn broadcast(mut self, cmd: &[u8]) -> () {
    self
      .connections
      .iter_mut()
      .for_each(|c| c.1.write_all(cmd).unwrap());
  }

  fn process(
    &mut self,
    _registry: &Registry,
    event: &Event,
    token: Token,
  ) -> Result<bool, Box<dyn Error>> {
    if event.is_readable() {
      let mut connection_closed = false;
      let mut received_data = vec![0; 4096];
      let mut bytes_read = 0;

      let stream = self.connections.get_mut(&token).unwrap();

      loop {
        match stream.read(&mut received_data[bytes_read..]) {
          Ok(0) => {
            connection_closed = true;
            break;
          }
          Ok(n) => {
            bytes_read += n;
            if bytes_read == received_data.len() {
              received_data.resize(received_data.len() + 1024, 0);
            }
          }
          Err(ref err) if err.kind() == ErrorKind::WouldBlock => break,
          Err(ref err) if err.kind() == ErrorKind::Interrupted => continue,
          Err(err) => return Err(Box::new(err)),
        }
      }

      if bytes_read != 0 {
        self.handle(&mut received_data[..bytes_read], token);
      }
      if connection_closed {
        println!("de-registered peer with token '{}'", token.0);
        return Ok(true);
      }
    }

    Ok(false)
  }

  fn handle(&mut self, data: &[u8], token: Token) -> () {
    // trace!("i am client: {:?}", self.clients.get(&token));
    // debug!("{:?}", self.connections);
    for cmd in get_commands_from_buffer(BytesMut::from(data)) {
      debug!("received: {:?}", cmd);
      match cmd.get(2).unwrap() {
        10 => {
          // PROPREQ
          debug!("received property request command from client 'null'");
          self
            .connections
            .get_mut(&token)
            .unwrap()
            .write_all(&create_property_update_command())
            .unwrap();
          debug!("sent property update command to client 'null'");
        }
        6 => {
          // SESSINIT
          let local_username = parse_session_initialization_command(cmd).username;
          self.clients.insert(token, local_username.clone());
          debug!(
            "received session initialization command from client '{}'",
            local_username,
          );
          self
            .connections
            .get_mut(&token)
            .unwrap()
            .write_all(&create_property_request_command())
            .unwrap();
          debug!(
            "sent session initialization command to client '{}'",
            local_username
          );
        }
        15 => {
          // PROPSET
          let avatar = parse_property_set_command(cmd);
          debug!(
            "received property set command from client '{}': {}",
            self.clients.get(&token).unwrap(),
            avatar,
          );
          self
            .connections
            .get_mut(&token)
            .unwrap()
            .write_all(&create_text_command(
              "WORLDSMASTER",
              &get_config().unwrap().worldsmaster_greeting,
            ))
            .unwrap();
          self
            .connections
            .get_mut(&token)
            .unwrap()
            .write_all(&create_action_command())
            .unwrap();
          debug!(
            "sent session initialization command to client '{}'",
            self.clients.get(&token).unwrap(),
          );
        }
        29 => {
          // BUDDYLISTUPDATE
          let received_buddy = from_utf8(
            cmd
              .get(4..cmd.get(0).unwrap().to_owned() as usize - 1)
              .unwrap(),
          )
          .unwrap();
          debug!(
            "received buddy list update command from client '{}': {}",
            self.clients.get(&token).unwrap(),
            received_buddy,
          );
          let buddies = vec![
            "dosfox",
            "Fallen_Angel",
            "Internet_Man",
            "Nexialist",
            "SirGemini",
            "SirGrandpa",
            "Wirlaburla",
          ];
          if buddies.contains(&received_buddy) {
            self
              .connections
              .get_mut(&token)
              .unwrap()
              .write_all(&create_buddy_list_notify_command(received_buddy))
              .unwrap();
            debug!(
              "sent buddy list notify command to client '{}'",
              self.clients.get(&token).unwrap(),
            );
          }
        }
        20 => {
          // ROOMIDRQ
          let room_name =
            from_utf8(cmd.get(4..cmd.get(0).unwrap().to_owned() as usize).unwrap()).unwrap();
          debug!(
            "received room id request command from client '{}': {}",
            self.clients.get(&token).unwrap(),
            room_name,
          );
          let room_id;
          if !self.room_ids.contains(&room_name.to_string()) {
            self.room_ids.push(room_name.to_string());
            room_id = self
              .room_ids
              .iter()
              .position(|i| i == &room_name.to_string())
              .unwrap();
            trace!("inserted room '{}' as '{}'", room_name, room_id);
          } else {
            let position = self
              .room_ids
              .iter()
              .position(|i| i == &room_name.to_string())
              .unwrap();
            trace!("found room '{}' as '{}'", room_name, position);
            room_id = position;
          }
          trace!("room name: {}, room id: {}", room_name, room_id);
          trace!("{:?}", self.room_ids);
          self
            .connections
            .get_mut(&token)
            .unwrap()
            .write_all(&create_room_id_redirect_command(room_name, room_id))
            .unwrap();
        }
        14 => {
          // TEXT
          let text =
            from_utf8(cmd.get(6..cmd.get(0).unwrap().to_owned() as usize).unwrap()).unwrap();
          let username = self.clients.get(&token).unwrap().clone();
          debug!(
            "received text command from client '{}': {}",
            username,
            format!("auto {}", text),
          );
          self.connections.iter_mut().for_each(|t| {
            t.1
              .write_all(&create_text_command(&username, text))
              .unwrap()
          });
          debug!("broadcasted text command to clients");
        }
        7 => {
          // SESSEXIT
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
