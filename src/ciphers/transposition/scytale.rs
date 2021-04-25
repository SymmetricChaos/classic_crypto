use std::fmt;
use std::collections::HashMap;
use num::Integer;

use crate::errors::CipherError;

pub fn pad_with_char(text: &str, length: usize, symbol: char) -> String {
    let mut text = text.to_string();
    while text.chars().count() % length != 0 {
        text.push(symbol)
    }
    text
}


pub struct Scytale {
    key: usize,
}

impl Scytale {
    pub fn new(key: usize) -> Scytale {
        Scytale{ key }
    }

/*     pub fn encode(&self, text: &str) -> String {
        let n_cols = text.len().div_ceil(&self.key);
        let symbols = text.chars();
        let mut rows = Vec::new();

        Ok("".to_string())
    } */

/*     pub fn decode(&self, text: &str) -> String {

    } */
}

impl fmt::Display for Scytale {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}",self.key)
    }
}

/* #[test]
fn scytale() {

    let scytale = Scytale::new(3);
    println!("Scytale Cipher: {}",scytale);
    let plaintext = "WEAREDISCOVEREDFLEEATONCEQKJEU";
    let ciphertext = scytale.encode(plaintext);

} */