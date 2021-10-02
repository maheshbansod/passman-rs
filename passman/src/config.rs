use bincode_aes::BincodeCryptor;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

use crate::error::Error;

pub struct Config {
    pub cryptor: BincodeCryptor,
    db_location: PathBuf,
}

#[derive(Serialize, Deserialize)]
struct ConfigFile {
    secret_key: String,
    db_location: PathBuf,
}

impl Config {
    /// Create a new config
    pub fn new(key: &str) -> Result<Self, Error> {
        let default_path = ".passman-db";
        let key = bincode_aes::create_key(key.as_bytes().to_vec())?;
        Ok(Self {
            cryptor: bincode_aes::with_key(key),
            db_location: PathBuf::from(default_path),
        })
    }

    /// Returns a Config from a file
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let config: ConfigFile =
            bincode::deserialize(std::fs::read_to_string(path)?.as_bytes()).unwrap();
        let key = bincode_aes::create_key(config.secret_key.as_bytes().to_vec())?;
        Ok(Config {
            cryptor: bincode_aes::with_key(key),
            db_location: config.db_location,
        })
    }

    /// Saves a Config to a file
    pub fn to_file<P: AsRef<Path>>(&self, path: P, secret_key: &str) -> Result<(), Error> {
        let config = ConfigFile {
            secret_key: secret_key.to_string(),
            db_location: self.db_location.clone(),
        };
        std::fs::write(path, bincode::serialize(&config)?)?;
        Ok(())
    }

    /// Set the secret key
    pub fn set_key(mut self, key: &str) -> Result<Self, Error> {
        let key = bincode_aes::create_key(key.as_bytes().to_vec())?;
        self.cryptor = bincode_aes::with_key(key);
        Ok(self)
    }

    /// Set the database file location
    pub fn set_db<P: AsRef<Path>>(mut self, path: P) -> Self {
        let mut db_loc = PathBuf::new();
        db_loc.push(path);
        self.db_location = db_loc;
        self
    }

    /// Get the database file location
    pub fn db_path(&self) -> &Path {
        self.db_location.as_path()
    }
}
