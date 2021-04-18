use crate::errors::CipherError;

pub trait Cipher {
    fn encode(&self, text: &str) -> Result<String,CipherError>;

    fn decode(&self, text: &str) -> Result<String,CipherError>;

    pub fn set_punctuation(&mut self, boolean: bool);

    pub fn set_whitespace(&mut self, boolean: bool);

    pub fn set_capitalization(&mut self, boolean: bool);
    
}