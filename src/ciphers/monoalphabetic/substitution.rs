use std::fmt;
use std::collections::HashMap;

use crate::errors::CipherError;



pub struct Substitution {
    key1: String,
    key2: String,
    map: HashMap<char,char>,
    map_inv: HashMap<char,char>,
    whitespace: bool,
}

impl Substitution {
    pub fn new(key1: &str, key2: &str) -> Substitution {
        let mut map = HashMap::new();
        let mut map_inv = HashMap::new();
        for (a, b) in key1.chars().zip(key2.chars()) {
            map.insert(a, b);
            map_inv.insert(b, a);
        }
        Substitution{ key1: key1.to_string(), key2: key2.to_string(), map, map_inv, whitespace: false }
    }

    pub fn set_whitespace(&mut self, boolean: bool) {
        self.whitespace = boolean
    }

    pub fn set_key(&mut self, key: (u32,u32)) {

    }

    pub fn encode(&self, text: &str) -> Result<String,CipherError> {

        Ok("".to_string())
    }

    pub fn decode(&self, text: &str) -> Result<String,CipherError> {

        Ok("".to_string())
    }
}

impl fmt::Display for Substitution {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Substitution Cipher\nkey:\n{}\n{}",self.key1,self.key2)
    }
}

#[test]
fn substitution() {

    
}