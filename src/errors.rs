use std::{error::Error, fmt};

#[derive(Debug)]
pub struct CipherError {
    desc: String,
}

impl CipherError {
    pub fn new(desc: &str) -> CipherError {
        CipherError{ desc: desc.to_string() }
    }
}

impl fmt::Display for CipherError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CipherError: {}",self.desc)
    }
}

impl Error for CipherError {}