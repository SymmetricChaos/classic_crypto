use std::{collections::VecDeque, fmt};
use std::iter::FromIterator;

use crate::errors::CipherError;
use crate::alphabet::ALPHA;

///! Vigenere Family of Cphers



fn caesar_enc(c: u8, n: u8) -> u8 {
    if c.is_ascii_uppercase() {
        return (ALPHA[&c] + ALPHA[&n]) % 26 + 65
    } else {
        return (ALPHA[&c] + ALPHA[&n]) % 26 + 97
    }
}

fn caesar_dec(c: u8, n: u8) -> u8 {
    if c.is_ascii_uppercase() {
        return (ALPHA[&c] + 26 - ALPHA[&n]) % 26 + 65
    } else {
        return (ALPHA[&c] + 26 - ALPHA[&n]) % 26 + 97
    }
}




pub struct Caesar {
    key: u8,
    whitespace: bool,
    punctuation: bool,
    capitalization: bool,
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
            } else if c.is_ascii_digit() {
                out.push(c)
            } else if c.is_ascii_whitespace() {
                if self.whitespace { out.push(c) }
            } else if c.is_ascii_punctuation() {
                if self.punctuation { out.push(c) }
            } else {
                return Err(CipherError::new("Found char that is not alphanumeric, whitespace, or punctuation".to_string()))
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
            } else if c.is_ascii_digit() {
                out.push(c)
            } else if c.is_ascii_whitespace() {
                if self.whitespace { out.push(c) }
            } else if c.is_ascii_punctuation() {
                if self.punctuation { out.push(c) }
            } else {
                return Err(CipherError::new("Found char that is not alphanumeric, whitespace, or punctuation".to_string()))
            }
        }
        let val = String::from_utf8(out).unwrap();
        Ok(val)
    }
}

impl fmt::Display for Caesar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Caesar Cipher\nkey: {}",self.key)
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
            } else if c.is_ascii_digit() {
                out.push(c)
            } else if c.is_ascii_whitespace() {
                if self.whitespace { out.push(c) }
            } else if c.is_ascii_punctuation() {
                if self.punctuation { out.push(c) }
            } else {
                return Err(CipherError::new("Found char that is not alphanumeric, whitespace, or punctuation".to_string()))
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
            } else if c.is_ascii_digit() {
                out.push(c)
            } else if c.is_ascii_whitespace() {
                if self.whitespace { out.push(c) }
            } else if c.is_ascii_punctuation() {
                if self.punctuation { out.push(c) }
            } else {
                return Err(CipherError::new("Found char that is not alphanumeric, whitespace, or punctuation".to_string()))
            }
        }
        let val = String::from_utf8(out).unwrap();
        Ok(val)
    }
}

impl fmt::Display for Vigenere {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let k: Vec<u8> = self.key.iter().map(|x| ALPHA[x]).collect();
        write!(f, "Vigenere Cipher\nkey: {:?}",k)
    }
}





pub struct Autokey {
    key: Vec::<u8>,
    whitespace: bool,
    punctuation: bool,
    capitalization: bool,
}

impl Autokey {

    pub fn new(key: Vec::<u8>) -> Autokey {
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

    pub fn set_key(&mut self, key: Vec::<u8>) {
        self.key = key
    }

    pub fn encode(&self, text: &str) -> Result<String,CipherError> {
        let ch = match self.capitalization {
            true => text.to_string().into_bytes(),
            false => text.to_ascii_uppercase().into_bytes(),
        };
        let mut out = Vec::new();
        let mut akey = VecDeque::from_iter(&self.key);
        for c in ch.iter() {
            if c.is_ascii_alphabetic() {
                akey.push_back(c);
                out.push(caesar_enc(*c,*akey.pop_front().unwrap()))
            } else if c.is_ascii_digit() {
                out.push(*c)
            } else if c.is_ascii_whitespace() {
                if self.whitespace { out.push(*c) }
            } else if c.is_ascii_punctuation() {
                if self.punctuation { out.push(*c) }
            } else {
                return Err(CipherError::new("Found char that is not alphanumeric, whitespace, or punctuation".to_string()))
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
        let mut akey: VecDeque<u8> = self.key.clone().into_iter().collect();
        for c in ch.iter() {
            if c.is_ascii_alphabetic() {
                let k = caesar_dec(*c,akey.pop_front().unwrap());
                akey.push_back(k);
                out.push(k)
            } else if c.is_ascii_digit() {
                out.push(*c)
            } else if c.is_ascii_whitespace() {
                if self.whitespace { out.push(*c) }
            } else if c.is_ascii_punctuation() {
                if self.punctuation { out.push(*c) }
            } else {
                return Err(CipherError::new("Found char that is not alphanumeric, whitespace, or punctuation".to_string()))
            }
        }
        let val = String::from_utf8(out).unwrap();
        Ok(val)
    }
}

impl fmt::Display for Autokey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let k: Vec<u8> = self.key.iter().map(|x| ALPHA[x]).collect();
        write!(f, "Autokey Cipher\nkey start: {:?}",k)
    }
}