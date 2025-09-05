use std::{
    fs::{File, read_to_string},
    io::{Error, Write},
};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub path: String,
    pub check_guid: bool,
    pub check_short_id: bool,
    pub check_media_id: bool,
}

impl Config {
    pub fn new() -> Config {
        Self {
            path: "".to_string(),
            check_guid: true,
            check_short_id: false,
            check_media_id: false,
        }
    }

    pub fn load(config_file: &str) -> Result<Config, Error> {
        let content = read_to_string(config_file)?;
        let c: Config = serde_json::from_str(&content)?;
        Ok(c)
    }

    pub fn save(&self, config_file: &str) -> Result<(), Error> {
        let json_str = serde_json::to_string_pretty(&self)?;
        let mut file = File::create(config_file)?;
        file.write_all(json_str.as_bytes())?;
        Ok(())
    }
}
