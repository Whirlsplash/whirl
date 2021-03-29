use serde_derive::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
	pub worldsmaster_greeting: String,
}
impl Default for Config {
	fn default() -> Self {
		Config {
			worldsmaster_greeting: "Welcome to Whirlsplash!".to_string(),
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
