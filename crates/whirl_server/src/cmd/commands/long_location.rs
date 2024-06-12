use {
  crate::cmd::{
    constants::Command,
    extendable::{Creatable, Parsable},
  },
  byteorder::{BigEndian, ReadBytesExt},
  bytes::{Buf, BufMut, BytesMut},
};

#[derive(Debug)]
pub struct LongLocation {
  pub x:         i16,
  pub y:         i16,
  pub z:         i16,
  pub direction: i16,
}

impl Parsable for LongLocation {
  fn parse(data: Vec<u8>) -> Self {
    let mut data = BytesMut::from(data.as_slice()).reader();

    Self {
      x:         data.read_i16::<BigEndian>().unwrap(),
      y:         data.read_i16::<BigEndian>().unwrap(),
      z:         data.read_i16::<BigEndian>().unwrap(),
      direction: data.read_i16::<BigEndian>().unwrap(),
    }
  }
}

impl Creatable for LongLocation {
  fn create_with_short_object_id(&self, short_object_id: u8) -> Vec<u8> {
    let mut command = BytesMut::new();

    // Header
    command.put_u8(short_object_id); // ObjId
    #[allow(clippy::cast_possible_truncation)]
    command.put_i8(Command::LongLoc as i32 as i8); // Type

    // Content
    command.put_i16(self.x);
    command.put_i16(self.y);
    command.put_i16(self.z);
    command.put_i16(self.direction);

    // Length
    let mut command_as_vec = command.to_vec();
    command_as_vec.insert(0, command.len() as u8 + 1);

    // Return bytes
    command_as_vec
  }
}
