use std::str::from_utf8;

use bytes::BytesMut;

pub fn parse_property_set_command(command: BytesMut) -> String {
  from_utf8(command.get(8..).unwrap()).unwrap().to_string()
}
