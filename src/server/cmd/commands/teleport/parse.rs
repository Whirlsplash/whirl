// Copyleft (ɔ) 2021-2021 The Whirlsplash Collective
// SPDX-License-Identifier: GPL-3.0-only

use byteorder::{BigEndian, ReadBytesExt};
use bytes::{Buf, BytesMut};

use crate::server::cmd::commands::teleport::structure::{Teleport, TeleportLiteral};

pub fn parse_teleport(data: Vec<u8>) -> Teleport {
  // https://stackoverflow.com/questions/41034635/how-do-i-convert-between-string-str-vecu8-and-u8
  let mut data = BytesMut::from(data.as_slice()).reader();

  Teleport {
    room_id:    data.read_u16::<BigEndian>().unwrap() as i8,
    exit_type:  data.read_u8().unwrap(),
    entry_type: data.read_u8().unwrap(),
    x:          data.read_i16::<BigEndian>().unwrap(),
    y:          data.read_i16::<BigEndian>().unwrap(),
    z:          data.read_i16::<BigEndian>().unwrap(),
    direction:  data.read_i16::<BigEndian>().unwrap(),
  }
}

pub fn parse_teleport_literal(data: Vec<u8>) -> TeleportLiteral {
  let teleport = parse_teleport(data);

  TeleportLiteral {
    room_id:    teleport.room_id,
    exit_type:  teleport.exit_type,
    entry_type: teleport.entry_type,
    x:          teleport.x as f32,
    y:          teleport.y as f32,
    z:          teleport.z as f32,
    direction:  teleport.direction as f32,
  }
}
