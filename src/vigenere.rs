use std::collections::VecDeque;
use std::iter::FromIterator;

use crate::errors::CipherError;

///! Vigenere Family of Cphers

pub struct Caesar {
    key: u8,
    whitespace: bool,
    punctuation: bool,
    capitalization: bool,
}


fn caesar_enc(c: u8, n: u8) -> u8 {
    if c.is_ascii_uppercase() {
        return (c-65+n)%26+65
    } else {
        return (c-97+n)%26+97
    }
}

fn caesar_dec(c: u8, n: u8) -> u8 {
    if c.is_ascii_uppercase() {
        return (c-65+(26-n))%26+65
    } else {
        return (c-97+(26-n))%26+97
    }
}

impl Caesar {
    pub fn new(key: u8) -> Caesar {
        Caesar{ key, whitespace: false, punctuation: false, capitalization: false }
    }

    pub fn set_punctuation(&mut self, boolean: bool) {
        self.punctuation = boolean
    }

    pub fn set_whitespace(&mut self, boolean: bool) {
        self.whitespace = boolean
    }

    pub fn set_capitalization(&mut self, boolean: bool) {
        self.capitalization = boolean
    }

    pub fn set_key(&mut self, key: u8) {
        self.key = key
    }

    pub fn encode(&self, text: &str) -> Result<String,CipherError> {
        let ch = match self.capitalization {
            true => text.to_string().into_bytes(),
            false => text.to_ascii_uppercase().into_bytes(),
        };
        let mut out = Vec::new();
        for c in ch {
            if c.is_ascii_alphabetic() {
                out.push(caesar_enc(c,self.key))
            } else if c.is_ascii_whitespace() {
                if self.whitespace { out.push(c) }
            } else if c.is_ascii_punctuation() {
                if self.punctuation { out.push(c) }
            } else {
                return Err(CipherError::new("Found char that is not alphabetic, whitespace, or punctuation".to_string()))
            }
        }
        let val = String::from_utf8(out).unwrap();
        Ok(val)
    }

    pub fn decode(&self, text: &str) -> Result<String,CipherError> {
        let ch = match self.capitalization {
            true => text.to_string().into_bytes(),
            false => text.to_ascii_uppercase().into_bytes(),
        };
        let mut out = Vec::new();
        for c in ch {
            if c.is_ascii_alphabetic() {
                out.push(caesar_dec(c,self.key))
            } else if c.is_ascii_whitespace() {
                if self.whitespace { out.push(c) }
            } else if c.is_ascii_punctuation() {
                if self.punctuation { out.push(c) }
            } else {
                return Err(CipherError::new("Found char that is not alphabetic, whitespace, or punctuation".to_string()))
            }
        }
        let val = String::from_utf8(out).unwrap();
        Ok(val)
    }
}





pub struct Vigenere {
    key: Vec::<u8>,
    whitespace: bool,
    punctuation: bool,
    capitalization: bool,
}

impl Vigenere {

    pub fn new(key: Vec<u8>) -> Vigenere {
        Vigenere{ key, whitespace: false, punctuation: false, capitalization: false }
    }

    pub fn set_punctuation(&mut self, boolean: bool) {
        self.punctuation = boolean
    }

    pub fn set_whitespace(&mut self, boolean: bool) {
        self.whitespace = boolean
    }

    pub fn set_capitalization(&mut self, boolean: bool) {
        self.capitalization = boolean
    }

    pub fn set_key(&mut self, key: Vec<u8>) {
        self.key = key
    }

    pub fn encode(&self, text: &str) -> Result<String,CipherError> {
        let ch = match self.capitalization {
            true => text.to_string().into_bytes(),
            false => text.to_ascii_uppercase().into_bytes(),
        };
        let mut out = Vec::new();
        let mut ckey = self.key.iter().cycle();
        for c in ch {
            if c.is_ascii_alphabetic() {
                out.push(caesar_enc(c,*ckey.next().unwrap()))
            } else if c.is_ascii_whitespace() {
                if self.whitespace { out.push(c) }
            } else if c.is_ascii_punctuation() {
                if self.punctuation { out.push(c) }
            } else {
                return Err(CipherError::new("Found char that is not alphabetic, whitespace, or punctuation".to_string()))
            }
            
        }
        let val = String::from_utf8(out).unwrap();
        Ok(val)
    }

    pub fn decode(&self, text: &str) -> Result<String,CipherError> {
        let ch = match self.capitalization {
            true => text.to_string().into_bytes(),
            false => text.to_ascii_uppercase().into_bytes(),
        };
        let mut out = Vec::new();
        let mut ckey = self.key.iter().cycle();
        for c in ch {
            if c.is_ascii_alphabetic() {
                out.push(caesar_dec(c,*ckey.next().unwrap()))
            } else if c.is_ascii_whitespace() {
                if self.whitespace { out.push(c) }
            } else if c.is_ascii_punctuation() {
                if self.punctuation { out.push(c) }
            } else {
                return Err(CipherError::new("Found char that is not alphabetic, whitespace, or punctuation".to_string()))
            }
        }
        let val = String::from_utf8(out).unwrap();
        Ok(val)
    }
}




/* 
pub struct Autokey {
    key: Vec::<u8>,
    whitespace: bool,
    punctuation: bool,
    capitalization: bool,
}

impl Autokey {

    pub fn new(key: u8) -> Autokey {
        Autokey{ key, whitespace: false, punctuation: false, capitalization: false }
    }

    pub fn set_punctuation(&mut self, boolean: bool) {
        self.punctuation = boolean
    }

    pub fn set_whitespace(&mut self, boolean: bool) {
        self.whitespace = boolean
    }

    pub fn set_capitalization(&mut self, boolean: bool) {
        self.capitalization = boolean
    }

    pub fn set_key(&mut self, key: u8) {
        self.key = key
    }

    pub fn encode(&self, text: &str) -> Result<String,CipherError> {
        let ch = text.to_ascii_uppercase().into_bytes();
        let mut out = Vec::new();
        let mut akey = VecDeque::from_iter(&self.key);
        for c in ch.iter() {
            if c.is_ascii_whitespace() {
                if self.whitespace { out.push(*c); }
            } else if c.is_ascii_punctuation() {
                if self.punctuation { out.push(*c); }
            } else {
                println!("{:?}",akey);
                akey.push_back(c);
                out.push(caesar_enc(*c,*akey.pop_front().unwrap())?)
            }
        }
        let val = String::from_utf8(out).unwrap();
        Ok(val)
    }

    pub fn decode(&self, text: &str) -> Result<String,CipherError> {
        let ch = text.to_ascii_uppercase().into_bytes();
        let mut out = Vec::new();
        let mut akey = VecDeque::from_iter(&self.key);
        for c in ch.iter() {
            if c.is_ascii_whitespace() {
                if self.whitespace { out.push(*c); }
            } else if c.is_ascii_punctuation() {
                if self.punctuation { out.push(*c); }
            } else {
                akey.push_back(c);
                out.push(caesar_dec(*c,*akey.pop_front().unwrap())?)
            }
        }
        let val = String::from_utf8(out).unwrap();
        Ok(val)
    }
}
*/