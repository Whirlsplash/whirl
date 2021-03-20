pub fn _is_double_crnl(window: &[u8]) -> bool {
	window.len() >= 4
		&& (window[0] == '\r' as u8)
		&& (window[1] == '\n' as u8)
		&& (window[2] == '\r' as u8)
		&& (window[3] == '\n' as u8)
}
