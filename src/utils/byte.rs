pub fn convert_u16_to_two_u8s_be(integer: u16) -> [u8; 2] {
	[(integer >> 8) as u8, integer as u8]
}
