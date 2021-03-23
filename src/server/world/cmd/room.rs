pub fn create_room_id_redirect_command(room_name: &str, room_id: &i32) -> Vec<u8> {
	let mut room_id_redirect = Vec::new();
	// room_id_redirect.push(room_id_redirect.len() as u8 + 1); // Data length
	room_id_redirect.push(0x01); // ?
	room_id_redirect.push(0x1A); // Command type1
	room_id_redirect.push(room_name.len() as u8); // UTF/ room name length
	for i in room_name.bytes() { room_id_redirect.push(i); } // Push `room_name`
	// for i in "<dimension-1>".bytes() { room_id_redirect.push(i); } // Push room number

	// Room number
	room_id_redirect.push(0x00);
	room_id_redirect.push(*room_id as u8);

	// IP
	room_id_redirect.push(0x00);
	room_id_redirect.push(0x00);
	room_id_redirect.push(0x00);
	room_id_redirect.push(0x00);

	// Port
	room_id_redirect.push(0x16);
	room_id_redirect.push(0x29);

	room_id_redirect.insert(0, room_id_redirect.len() as u8 + 1); // Data length

	room_id_redirect
}
