pub fn create_room_id_redirect_command(room_name: &str, room_id: usize) -> Vec<u8> {
  let mut room_id_redirect = vec![
    0x01, // ?
    0x1A, // Command type
  ];

  // room_id_redirect.push(room_id_redirect.len() as u8 + 1); // Data length
  room_id_redirect.push(room_name.len() as u8); // UTF/ room name length
  for i in room_name.bytes() {
    room_id_redirect.push(i);
  } // Push `room_name`
    // for i in "<dimension-1>".bytes() { room_id_redirect.push(i); } // Push room
    // number

  // Room number
  room_id_redirect.push(0x00);
  room_id_redirect.push(room_id as u8);

  // IP
  room_id_redirect.push(0x00);
  room_id_redirect.push(0x00);
  room_id_redirect.push(0x00);
  room_id_redirect.push(0x00);

  // Port
  for byte in 5673_u16.to_be_bytes().iter() {
    room_id_redirect.push(*byte);
  }

  room_id_redirect.insert(0, room_id_redirect.len() as u8 + 1); // Data length

  room_id_redirect
}
