pub fn create_text_command(user: &str, message: &str) -> Vec<u8> {
  let mut text = Vec::with_capacity(6 + user.len() + message.len());
  text.push(0x01); // ?
  text.push(0x0E); // Command type
  text.push(0x00); // Assumed to be a divider.
  text.push(user.len() as u8); // 'user' length
  for i in user.bytes() {
    text.push(i);
  } // Pushing 'user'
  text.push(message.len() as u8); // 'message' length
  for i in message.bytes() {
    text.push(i);
  } // Pushing `message`
  text.insert(0, text.len() as u8 + 1); // Insert data length as first byte.

  text // Return created array
}

// TODO: Get this working!
// pub fn get_message_from_text_command(buffer: &'static [u8; 1024]) -> &'static
// str { 	from_utf8(
// 		&buffer[6..*&buffer.get(0).unwrap().to_owned() as usize]
// 	).unwrap()
// }

pub fn create_text_command_with_action(
  user: &str,
  message: &str,
  // action: &str // Not accepting input until I figure out how actions work.
) -> Vec<u8> {
  let mut text = Vec::with_capacity(6 + user.len() + message.len());
  text.push(0x01); // ?
  text.push(0x0E); // Command type
  text.push(0x00); // Assumed to be a divider.
  text.push(user.len() as u8); // 'user' length
  for i in user.bytes() {
    text.push(i);
  } // Pushing 'user'
  text.push(message.len() as u8); // 'message' length
  for i in message.bytes() {
    text.push(i);
  } // Pushing `message`

  let action: [u8; 18] = [
    0x12, 0x01, 0x11, 0x00, 0x05, 0x54, 0x52, 0x41, 0x44, 0x45, 0x07, 0x26, 0x7c, 0x2b, 0x69, 0x6e,
    0x76, 0x3e,
  ];
  for i in action.iter() {
    text.push(*i);
  }

  text.insert(0, text.len() as u8 + 1); // Insert data length as first byte.

  text // Return created array
}
