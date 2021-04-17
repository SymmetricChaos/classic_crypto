use std::collections::VecDeque;
use std::iter::FromIterator;

use crate::errors::CipherError;

///! Vigenere Family of Cphers

pub struct Caesar {
    key: u8,
    whitespace: bool,
}


fn caesar_enc(c: u8, n: u8) -> Result<u8,CipherError> {
    if !c.is_ascii_uppercase() {
        return Err(CipherError::new(format!("{} is not a valid character",c as char)))
    } else {
        return Ok((c-65+n)%26+65)
    }
}

fn caesar_dec(c: u8, n: u8) -> Result<u8,CipherError> {
    if !c.is_ascii_uppercase() {
        return Err(CipherError::new(format!("{} is not a valid character",c)))
    } else {
        return Ok((c-65+(26-n))%26+65)
    }
}

impl Caesar {
    pub fn new(key: u8) -> Caesar {
        Caesar{ key, whitespace: false }
    }

    pub fn set_whitespace(&mut self, boolean: bool) {
        self.whitespace = boolean
    }

    pub fn set_key(&mut self, key: u8) {
        self.key = key
    }

    pub fn encode(&self, text: &str) -> Result<String,CipherError> {
        let ch = text.to_ascii_uppercase().into_bytes();
        let mut out = Vec::new();
        for c in ch {
            if self.whitespace && c.is_ascii_whitespace() {
                out.push(c);
            } else if c.is_ascii_punctuation() {
                continue
            } else {
                out.push(caesar_enc(c,self.key)?)
            }
        }
        let val = String::from_utf8(out).unwrap();
        Ok(val)
    }

    pub fn decode(&self, text: &str) -> Result<String,CipherError> {
        let ch = text.to_ascii_uppercase().into_bytes();
        let mut out = Vec::new();
        for c in ch {
            if self.whitespace && c.is_ascii_whitespace() {
                out.push(c);
                continue
            } else if c.is_ascii_punctuation() {
                continue
            } else {
                out.push(caesar_dec(c,self.key)?)
            }
        }
        let val = String::from_utf8(out).unwrap();
        Ok(val)
    }
}





pub struct Vigenere {
    key: Vec::<u8>,
    whitespace: bool,
}

impl Vigenere {

    pub fn set_whitespace(&mut self, boolean: bool) {
        self.whitespace = boolean
    }

    pub fn set_key(&mut self, key: Vec::<u8>) {
        self.key = key
    }

    pub fn new(key: Vec::<u8>) -> Vigenere {
        Vigenere{ key, whitespace: false }
    }

    pub fn encode(&self, text: &str) -> Result<String,CipherError> {
        let ch = text.to_ascii_uppercase().into_bytes();
        let mut out = Vec::new();
        let mut ckey = self.key.iter().cycle();
        for c in ch {
            if self.whitespace && c.is_ascii_whitespace() {
                out.push(c);
                continue
            } else if c.is_ascii_punctuation() {
                continue
            } else {
                out.push(caesar_enc(c,*ckey.next().unwrap())?)
            }
            
        }
        let val = String::from_utf8(out).unwrap();
        Ok(val)
    }

    pub fn decode(&self, text: &str) -> Result<String,CipherError> {
        let ch = text.to_ascii_uppercase().into_bytes();
        let mut out = Vec::new();
        let mut ckey = self.key.iter().cycle();
        for c in ch {
            if self.whitespace && c.is_ascii_whitespace() {
                out.push(c);
                continue
            } else if c.is_ascii_punctuation() {
                continue
            } else {
                out.push(caesar_dec(c,*ckey.next().unwrap())?)
            }
        }
        let val = String::from_utf8(out).unwrap();
        Ok(val)
    }
}





pub struct Autokey {
    key: Vec::<u8>,
    whitespace: bool,
}

impl Autokey {

    pub fn set_whitespace(&mut self, boolean: bool) {
        self.whitespace = boolean
    }

    pub fn set_key(&mut self, key: Vec::<u8>) {
        self.key = key
    }

    pub fn new(key: Vec::<u8>) -> Autokey {
        Autokey{ key, whitespace: false }
    }

    pub fn encode(&self, text: &str) -> Result<String,CipherError> {
        let ch = text.to_ascii_uppercase().into_bytes();
        let mut out = Vec::new();
        let mut akey = VecDeque::from_iter(&self.key);
        for c in ch.iter() {
            if self.whitespace && c.is_ascii_whitespace() {
                out.push(*c);
                continue
            }
            akey.push_back(&c);
            out.push(caesar_enc(*c,*akey.pop_front().unwrap())?)
        }
        let val = String::from_utf8(out).unwrap();
        Ok(val)
    }

    pub fn decode(&self, text: &str) -> Result<String,CipherError> {
        let ch = text.to_ascii_uppercase().into_bytes();
        let mut out = Vec::new();
        for c in ch {
            if self.whitespace && c.is_ascii_whitespace() {
                out.push(c);
                continue
            }

        }
        let val = String::from_utf8(out).unwrap();
        Ok(val)
    }
}
