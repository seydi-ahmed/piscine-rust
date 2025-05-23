use std::fmt;
use std::fmt::Display;
use std::error::Error;

#[derive(Debug)]
pub enum ParseErr {
    Empty,
    Malformed(Box<dyn Error>),
}

impl std::fmt::Display for ParseErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Fail to parse todo")
    }
}

impl Error for ParseErr {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ParseErr::Empty => None,
            ParseErr::Malformed(_) => Some(self)
        }
    }
}

#[derive(Debug)]
pub struct ReadErr {
    pub child_err: Box<dyn Error>,
}

impl Display for ReadErr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Fail to read todo file")
    }
}

impl Error for ReadErr {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(self.child_err.as_ref())
    }
}