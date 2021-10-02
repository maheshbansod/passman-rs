#[derive(Debug)]
pub struct Error {
    pub kind: ErrorKind,
}

#[non_exhaustive]
#[derive(Debug)]
pub enum ErrorKind {
    Serialization(std::boxed::Box<bincode::ErrorKind>),
    IOError(std::io::Error),
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
