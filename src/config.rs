use serde_derive::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
	pub worldsmaster_greeting: String,
	pub auto_server_port: i32,
	pub room_server_port: i32,
}
impl Default for Config {
	fn default() -> Self {
		Config {
			worldsmaster_greeting: "Welcome to Whirlsplash!".to_string(),
			auto_server_port: 6650,
			room_server_port: 5673,
		}
	}
}

pub fn get_config() -> Result<Config, confy::ConfyError> {
	let config: Config = confy::load_path("./whirl.toml").unwrap();

	Ok(config)
}

pub fn store_config(config: Config) -> Result<(), confy::ConfyError> {
	confy::store_path("./whirl.toml", config)
}
