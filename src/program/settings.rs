use config::{Config, ConfigError, File};
use dirs;
use serde::Deserialize;
use std::path::Path;

use super::formatting;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub data_file_path: String,
    pub config_file_path: String,
    pub task_format: String,
}

impl Settings {
    /// Ensures we have the correct paths to the config and data files. Doesn't create any files/directories.
    pub fn new() -> Result<Self, ConfigError> {
        let mut s = Config::new();

        let home = match dirs::home_dir() {
            Some(pb) => pb,
            None => {
                return Err(ConfigError::Message(String::from(
                    "Cannot find user's home directory",
                )))
            }
        };

        // Config path can only be in one location
        let config_path = home.join(".effconfig");
        s.set_default("config_file_path", config_path.to_str())?;

        let default_data_path = home.join(".efficacy");

        // Defaults
        s.set_default("data_file_path", default_data_path.to_str())?;
        s.set_default("task_format", "%b %d %i")?;

        if config_path.exists() {
            s.merge(File::from(config_path))?;
        }

        // Ensures the data path that is set is valid
        let set_data_file_path: String = s.get("data_file_path")?;
        if !Path::new(&set_data_file_path).exists() {
            s.set("data_file_path", default_data_path.to_str())?;
        }

        // Ensures the format string is valid
        if !formatting::valid_task_format(&s.get("task_format")?) {
            s.set("task_format", "%b %d")?;
        }

        s.try_into()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_settings() {
        let settings = super::Settings::new();
        println!("{:#?}", settings);
    }
}
