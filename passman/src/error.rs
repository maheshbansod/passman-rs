#[derive(Debug)]
pub struct Error {
    pub kind: ErrorKind,
}

#[non_exhaustive]
#[derive(Debug)]
pub enum ErrorKind {
    Serialization(std::boxed::Box<bincode::ErrorKind>),
    IOError(std::io::Error),
    CryptorError(bincode_aes::CryptorError),
}
impl From<std::boxed::Box<bincode::ErrorKind>> for Error {
    fn from(err: std::boxed::Box<bincode::ErrorKind>) -> Error {
        Error {
            kind: ErrorKind::Serialization(err),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        Error {
            kind: ErrorKind::IOError(err),
        }
    }
}

impl From<bincode_aes::CryptorError> for Error {
    fn from(err: bincode_aes::CryptorError) -> Self {
        Self {
            kind: ErrorKind::CryptorError(err),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for Error {}
