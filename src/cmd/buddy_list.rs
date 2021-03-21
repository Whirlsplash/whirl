/// In the future, this will take a `Vec` of buddies and dynamically
/// create a response packet based on the amount of buddies given.
pub fn create_buddy_list_notify_command(buddy: &str) -> Vec<u8> {
	let mut buddy_list_notify = Vec::with_capacity(5 + buddy.len());
	buddy_list_notify.push(0x01); // ?
	buddy_list_notify.push(0x1E); // BUDDYLISTNOTIFY
	// The number of buddies you are being notified of?
	buddy_list_notify.push(0x0A); // 01 // ^
	for i in buddy.bytes() { buddy_list_notify.push(i); } // Buddy name
	buddy_list_notify.push(0x01); // Is buddy logged on?
	// Insert data length as first byte.
	buddy_list_notify.insert(0, buddy_list_notify.len() as u8 + 1); // ^

	buddy_list_notify // Return created array
}