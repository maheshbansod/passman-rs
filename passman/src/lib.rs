mod config;
mod error;

use rand::{distributions::Alphanumeric, Rng};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

use config::Config;
use error::Error;

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash)]
struct PassMapKey {
    for_what: String,
    user: String,
}

pub struct PassMan {
    data: HashMap<PassMapKey, String>,
    config: Config,
}

impl PassMan {
    /// Create a new Passman
    pub fn new<P: AsRef<Path>>(config_file: P) -> Result<PassMan, Error> {
        let config = Config::from_file(config_file)?;
        Self::with_config(config)
    }

    pub fn with_config(config: Config) -> Result<Self, Error> {
        let data = if let Ok(data) = std::fs::read_to_string(config.db_path()) {
            let mut bytes = data.as_bytes().to_vec();
            //TODO: remove the unwrap and find out houw to handle Box<dyn Errors>????
            config.cryptor.deserialize(&mut bytes).unwrap()
        } else {
            HashMap::new()
        };
        Ok(Self { data, config })
    }

    /// Save a new user-password in the data file or update existing
    pub fn save_or_update(&mut self, for_what: &str, user: &str, pass: &str) {
        self.data.insert(
            PassMapKey {
                for_what: String::from(for_what),
                user: String::from(user),
            },
            String::from(pass),
        );
    }

    /// Get a password for a specific user on a website if it exists
    pub fn get(&self, for_what: &str, user: &str) -> Option<String> {
        self.data
            .get(&PassMapKey {
                for_what: String::from(for_what),
                user: String::from(user),
            })
            .cloned()
    }

    /// Save the object that is in memory to the data file
    /// Note that without calling this method, nothing will be saved in the file
    pub fn save(&self) -> Result<(), Error> {
        std::fs::write(&self.config.db_path(), bincode::serialize(&self.data)?)?;
        Ok(())
    }
}

/// Generate a random password of the length if provided
pub fn genpass(len: Option<usize>) -> String {
    let len = len.unwrap_or(12);
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(len)
        .map(char::from)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_should_save_a_pass_and_get_it_inmemory() {
        let config = Config::new("secret_key").unwrap();
        let mut passman = crate::PassMan::with_config(config).unwrap();
        passman.save_or_update("test1", "user1", "pass1");
        passman.save_or_update("test2", "user2", "pass2");
        passman.save_or_update("test1", "user3", "pass3");

        assert_eq!(passman.get("test1", "user1"), Some(String::from("pass1")));
        assert_eq!(passman.get("test2", "user2"), Some(String::from("pass2")));
        assert_eq!(passman.get("test1", "user3"), Some(String::from("pass3")));
    }
    #[test]
    fn it_should_save_a_pass_and_get_it_in_a_file() {
        let config = Config::new("secret_key").unwrap();
        let mut passman = crate::PassMan::with_config(config).unwrap();
        passman.save_or_update("test1", "user1", "pass1");
        passman.save_or_update("test2", "user2", "pass2");
        passman.save_or_update("test1", "user3", "pass3");
        passman.save().unwrap();

        let config = Config::new("secret_key").unwrap();
        let passman = crate::PassMan::with_config(config).unwrap();

        assert_eq!(passman.get("test1", "user1"), Some(String::from("pass1")));
        assert_eq!(passman.get("test2", "user2"), Some(String::from("pass2")));
        assert_eq!(passman.get("test1", "user3"), Some(String::from("pass3")));
    }
}
