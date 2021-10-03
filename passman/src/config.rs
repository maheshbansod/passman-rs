use bincode_aes::BincodeCryptor;
use crypto::digest::Digest;
use crypto::scrypt;
use crypto::sha1::Sha1;
use serde::{Deserialize, Serialize};
use std::io::Read;
use std::path::{Path, PathBuf};

use crate::error::Error;

pub struct Config {
    pub cryptor: BincodeCryptor,
    db_location: PathBuf,
}

#[derive(Serialize, Deserialize)]
struct ConfigFile {
    //Plain text now. TODO: make it hash of scrypt of the master password = sha1(scrypt(secret_key)).
    //scrypt(secret_key) = key for AES
    secret_key: Vec<u8>,
    db_location: PathBuf,
}

impl Config {
    /// Create a new config
    pub fn new(key: &str) -> Result<Self, Error> {
        let default_path = ".passman-db";
        let salt = "salt";
        let mut key_bytes: [u8; 32] = [0; 32];
        scrypt::scrypt(
            key.as_bytes(),
            salt.as_bytes(),
            &scrypt::ScryptParams::new(11, 8, 2),
            &mut key_bytes,
        );
        let key = bincode_aes::create_key(key_bytes.to_vec())?;
        Ok(Self {
            cryptor: bincode_aes::with_key(key),
            db_location: PathBuf::from(default_path),
        })
    }

    /// Returns a Config from a file
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let mut file = std::fs::File::open(path)?;
        let mut bytes = vec![];
        file.read_to_end(&mut bytes)?;
        let config: ConfigFile = bincode::deserialize(&bytes).unwrap();
        let key = bincode_aes::create_key(config.secret_key.to_vec())?;
        Ok(Config {
            cryptor: bincode_aes::with_key(key),
            db_location: config.db_location,
        })
    }

    /// Saves a Config to a file
    pub fn to_file<P: AsRef<Path>>(&self, path: P, secret_key: &str) -> Result<(), Error> {
        // SERIOUS VULNERABILITY
        // THIS WILL OVERWRITE SETTINGS NOW. TODO: MAKE SURE TO FIX IT AT SOME POINT TO CHECK IF THE SECRET KEY IS CORRECT
        let salt = "salt";
        let mut key_bytes: [u8; 32] = [0; 32];
        scrypt::scrypt(
            secret_key.as_bytes(),
            salt.as_bytes(),
            &scrypt::ScryptParams::new(11, 8, 2),
            &mut key_bytes,
        );

        // let mut hasher = Sha1::new();

        // hasher.input(&key_bytes);

        // let mut hashed_key = vec![];

        // hasher.result(&mut hashed_key);

        let config = ConfigFile {
            secret_key: key_bytes.to_vec(),
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
