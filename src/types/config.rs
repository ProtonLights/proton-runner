use toml::{Parser, Table};

use error::Error;
use utils;


/// Struct that matches yaml config found in config.yaml
pub struct Config {
	pub music_dir: String,
	pub output_dir: String,
	pub playlists_dir: String,
}


impl Config {
	pub fn new(cfg_path: &str) -> Result<Config, Error> {
		// Read in config file
		let cfg_str = try!(utils::file_as_string(cfg_path));

		// Parse toml
		let mut parser = Parser::new(&cfg_str);
		let config = match parser.parse() {
			Some(config) => try!(Config::extract_config(config)),
			None => {
				// Guaranteed to have at least one error, so unwrap is safe
				let err = parser.errors.get(0).unwrap();
				return Err(Error::TomlParseError(err.to_owned()));
			},
		};

		Ok(config)
	}

	/// Extracts the proton-runner configuration from the toml table,
	/// checking if values are valid and paths exist, throwing errors
	// if they aren't or don't
	fn extract_config(toml_cfg: Table) -> Result<Config, Error> {
		let music_dir = try!(Config::extract_path(&toml_cfg, "music_dir"));
		let output_dir = try!(Config::extract_path(&toml_cfg, "output_dir"));
		let playlists_dir = try!(Config::extract_path(&toml_cfg, "playlists_dir"));

		Ok(Config {
			music_dir: music_dir,
			output_dir: output_dir,
			playlists_dir: playlists_dir,
		})
	}

	/// Extracts a path from a toml table, checking that it is a string
	/// and the path exists. Returns error if anything out of sorts
	fn extract_path(toml_cfg: &Table, key: &str) -> Result<String, Error> {
		toml_cfg.get(key)
			.ok_or(Error::ConfigFieldMissing(key.to_owned()))
			.and_then(|path| path.as_str().ok_or(Error::ConfigInvalidType("&str".to_string(), path.to_owned())))
			.and_then(|path_str| utils::check_path(path_str).map(|_| path_str.to_owned()))
	}
}
