mod error;

use rand::{distributions::Alphanumeric, Rng};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

use error::Error;

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash)]
struct PassMapKey {
    for_what: String,
    user: String,
}

pub struct PassMan {
    dfile: std::path::PathBuf,
    data: HashMap<PassMapKey, String>,
}

impl PassMan {
    /// Create a new Passman
    pub fn new<P: AsRef<Path>>(filename: P) -> PassMan {
        let data = if let Ok(data) = std::fs::read_to_string(&filename) {
            bincode::deserialize(data.as_bytes()).unwrap()
        } else {
            HashMap::new()
        };
        let mut filenamebuff = PathBuf::new();
        filenamebuff.push(filename);
        PassMan {
            dfile: (*filenamebuff).to_path_buf(),
            data,
        }
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
        std::fs::write(&self.dfile, bincode::serialize(&self.data)?)?;
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
        let mut passman = crate::PassMan::new(&"testdb.test");
        passman.save_or_update("test1", "user1", "pass1");
        passman.save_or_update("test2", "user2", "pass2");
        passman.save_or_update("test1", "user3", "pass3");

        assert_eq!(passman.get("test1", "user1"), Some(String::from("pass1")));
        assert_eq!(passman.get("test2", "user2"), Some(String::from("pass2")));
        assert_eq!(passman.get("test1", "user3"), Some(String::from("pass3")));
    }
    #[test]
    fn it_should_save_a_pass_and_get_it_in_a_file() {
        let mut passman = PassMan::new(&std::path::PathBuf::from("testdb.test"));
        passman.save_or_update("test1", "user1", "pass1");
        passman.save_or_update("test2", "user2", "pass2");
        passman.save_or_update("test1", "user3", "pass3");
        passman.save().unwrap();

        let passman = PassMan::new(&std::path::PathBuf::from("testdb.test"));

        assert_eq!(passman.get("test1", "user1"), Some(String::from("pass1")));
        assert_eq!(passman.get("test2", "user2"), Some(String::from("pass2")));
        assert_eq!(passman.get("test1", "user3"), Some(String::from("pass3")));
    }
}
