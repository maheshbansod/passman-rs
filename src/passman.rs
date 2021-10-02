use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash)]
struct PassMapKey {
    for_what: String,
    user: String,
}

pub struct PassMan {
    dfile: std::path::PathBuf,
    data: HashMap<PassMapKey, String>,
}

const PASSFILE: &str = ".passman-rs-db";

impl PassMan {
    /// Create a new Passman
    pub fn new(filename: &std::path::PathBuf) -> PassMan {
        let data = if let Ok(data) = std::fs::read_to_string(filename) {
            bincode::deserialize(data.as_bytes()).unwrap()
        } else {
            HashMap::new()
        };
        let filename = filename.clone();
        PassMan {
            dfile: (*filename).to_path_buf(),
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
            .map(|x| x.clone())
    }

    /// Save the object that is in memory to the data file
    /// Note that without calling this method, nothing will be saved in the file
    pub fn save(&self) -> Result<(), Error> {
        std::fs::write(&self.dfile, bincode::serialize(&self.data)?)?;
        Ok(())
    }
}

//TODO: implement proper error handling
#[derive(Debug)]
pub struct Error {}
impl From<std::boxed::Box<bincode::ErrorKind>> for Error {
    fn from(_: std::boxed::Box<bincode::ErrorKind>) -> Error {
        Error {}
    }
}

impl From<std::io::Error> for Error {
    fn from(_: std::io::Error) -> Error {
        Error {}
    }
}
