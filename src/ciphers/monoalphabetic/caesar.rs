use std::fmt;

use crate::alphabet::CipherAlphabet;
use crate::modulus::*;


pub struct Caesar {
    key: Modulo,
    alpha: CipherAlphabet,
    whitespace: bool,
}

impl Caesar {
    pub fn new(key: u32, alpha: CipherAlphabet) -> Caesar {
        let key = key.to_modulo(alpha.len() as u32);
        Caesar{ key, alpha, whitespace: false }
    }

    pub fn set_whitespace(&mut self, boolean: bool) {
        self.whitespace = boolean
    }

    pub fn set_alpha(&mut self, alpha: CipherAlphabet) {
        self.alpha = alpha
    }

    pub fn set_key(&mut self, key: u32) {
        self.key = key.to_modulo(self.alpha.len() as u32);
    }

    pub fn encode(&self, text: &str) -> String {
        let ch = text.to_ascii_uppercase();
        let mut out = Vec::new();
        for c in ch.chars() {
            if c.is_ascii_whitespace() {
                if self.whitespace {
                    out.push(c);
                }
            } else {
                let v = match self.alpha.char_to_val(c) {
                    Some(m) => m,
                    None => continue
                };
                let x = v + self.key;
                out.push(self.alpha.val_to_char(x))
            }
        }
        let val: String = out.iter().collect();
        val
    }

    pub fn decode(&self, text: &str) -> String {
        let ch = text.to_ascii_uppercase();
        let mut out = Vec::new();
        for c in ch.chars() {
            if c.is_ascii_whitespace() && self.whitespace {
                out.push(c);
            } else {
                let v = match self.alpha.char_to_val(c) {
                    Some(m) => m,
                    None => panic!("unknown character encountered while decoding")
                };
                let x = v - self.key;
                out.push(self.alpha.val_to_char(x))
            }

        }
        let val: String = out.iter().collect();
        val
    }
}

impl fmt::Display for Caesar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Caesar Cipher\nkey: {}",self.key.value())
    }
}

#[test]
fn caesar() {
    let alpha = CipherAlphabet::new("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
    let mut aff = Caesar::new(1, alpha);
    aff.set_whitespace(true);
    let plaintext = "the quick brown fox jumps over the lazy dog";
    let ciphertext = aff.encode(plaintext);
    let cleartext = aff.decode(&ciphertext);

    println!("{}\n{}\n{}",plaintext,ciphertext,cleartext);
    
}