use std::fmt;
use std::process::exit;

#[derive(Debug, Clone)]
pub enum ErrorKind {
    NotFound,
    InvalidInput,
    UnknownType,
    ParsingError,
    AlreadyRan,
    CommandExitedWithError,
    EncodingError,
    AlreadyExists,
    Other,
}

#[derive(Clone)]
pub struct Error {
    pub msg: String,
    pub kind: ErrorKind,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Process failed due to an underlying error:\n[KIND] {:?}\n[ERROR] {}",
            self.kind, self.msg
        )
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "ERROR:\n{self}\n[DEBUG_INFO]\n[FILE] {}\n[LINE] {}",
            file!(),
            line!()
        )
    }
}

impl Error {
    pub fn new(kind: ErrorKind, msg: String) -> Self {
        Self {
            msg: msg,
            kind: kind,
        }
    }
    pub fn excec(&self) -> Self {
        return (*self).clone();
    }
}
