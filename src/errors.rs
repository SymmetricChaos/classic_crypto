use std::{error::Error, fmt};

#[derive(Debug)]
pub struct CipherError {
    desc: String,
}

impl CipherError {
    pub fn new(desc: String) -> CipherError {
        CipherError{ desc }
    }
}

impl fmt::Display for CipherError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CipherError: {}",self.desc)
    }
}

impl Error for CipherError {}