use std::fmt;

use crate::errors::CipherError;
use crate::alphabet::ModularAlphabet;
use crate::modulus::*;



pub struct Vigenere {
    key: Vec<Modulo>,
    key_name: String,
    alpha: ModularAlphabet,
    whitespace: bool,
}

impl Vigenere {
    pub fn new(key: &str, alpha: ModularAlphabet) -> Vigenere {
        let key_name = key.to_string();
        let key = key.chars().map(|x| *alpha.char_to_val(x).unwrap()).collect();
        Vigenere{ key, key_name, alpha, whitespace: false }
    }

    pub fn set_whitespace(&mut self, boolean: bool) {
        self.whitespace = boolean
    }

    pub fn set_alpha(&mut self, alpha: ModularAlphabet) {
        self.alpha = alpha
    }

    pub fn encrypt(&self, text: &str) -> Result<String,CipherError> {
        let ch = text.to_ascii_uppercase();
        let mut out = Vec::new();
        let mut ckey = self.key.iter().cycle();
        for c in ch.chars() {
            if c.is_ascii_whitespace() {
                if self.whitespace {
                    out.push(c);
                }
            } else {
                let v = match self.alpha.char_to_val(c) {
                    Some(m) => *m,
                    None => continue
                };
                let x = v + *ckey.next().unwrap();
                out.push(*self.alpha.val_to_char(x).unwrap())
            }
        }
        let val: String = out.iter().collect();
        Ok(val)
    }

    pub fn decrypt(&self, text: &str) -> Result<String,CipherError> {
        let ch = text.to_ascii_uppercase();
        let mut out = Vec::new();
        let mut ckey = self.key.iter().cycle();
        for c in ch.chars() {
            if c.is_ascii_whitespace() {
                if self.whitespace {
                    out.push(c);
                }
            } else {
                let v = match self.alpha.char_to_val(c) {
                    Some(m) => *m,
                    None => continue
                };
                let x = v - *ckey.next().unwrap();
                out.push(*self.alpha.val_to_char(x).unwrap())
            }
        }
        let val: String = out.iter().collect();
        Ok(val)
    }
}

impl fmt::Display for Vigenere {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Vigenere Cipher\nkey: {:?}",self.key_name)
    }
}

#[test]
fn affine() {
    let alpha = ModularAlphabet::new("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
    let mut aff = Vigenere::new("SECRET", alpha);
    aff.set_whitespace(true);
    let plaintext = "the quick brown fox jumps over the lazy dog";
    let ciphertext = aff.encrypt(plaintext).unwrap();
    let cleartext = aff.decrypt(&ciphertext).unwrap();

    println!("{}\n{}\n{}",plaintext,ciphertext,cleartext);
    
}