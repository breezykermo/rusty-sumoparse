use csv::Error as CsvError;
use serde_json::Error as SerdeJsonError;
use std::error;
use std::fmt;
use std::io;

#[derive(Debug)]
pub enum Error {
    General(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::General(msg) => write!(f, "{}", msg),
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::General(err.to_string())
    }
}

impl From<&str> for Error {
    fn from(msg: &str) -> Error {
        Error::General(msg.to_string())
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match self {
            Error::General(msg) => msg,
        }
    }
}

impl From<CsvError> for Error {
    fn from(err: CsvError) -> Error {
        Error::General(err.to_string())
    }
}

impl From<SerdeJsonError> for Error {
    fn from(err: SerdeJsonError) -> Error {
        Error::General(err.to_string())
    }
}
