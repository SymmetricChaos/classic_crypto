use std::fmt;

use crate::errors::CipherError;
use crate::alphabet::CipherAlphabet;
use crate::modulus::*;



pub struct Affine {
    key1: Modulo,
    key2: Modulo,
    akey2: Modulo,
    alpha: CipherAlphabet,
    whitespace: bool,
}

impl Affine {
    pub fn new(key: (u32,u32), alpha: CipherAlphabet) -> Affine {
        let key1 = key.0.to_modulo(alpha.len() as u32);
        let key2 = key.1.to_modulo(alpha.len() as u32);
        let akey2 = match key2.mul_inv() {
            Some(m) => m,
            None => panic!("{} has no multiplicative inverse modulo {}",key2.value(),key2.modulus()),
        };

        Affine{ key1, key2, akey2, alpha, whitespace: false }
    }

    pub fn set_whitespace(&mut self, boolean: bool) {
        self.whitespace = boolean
    }

    pub fn set_alpha(&mut self, alpha: CipherAlphabet) {
        self.alpha = alpha
    }

    pub fn set_key(&mut self, key: (u32,u32)) {
        let key1 = key.0.to_modulo(self.alpha.len());
        let key2 = key.1.to_modulo(self.alpha.len());
        let akey2 = match key2.mul_inv() {
            Some(m) => m,
            None => panic!("{} has no multiplicative inverse modulo {}",key2.value(),key2.modulus()),
        };
        self.key1 = key1;
        self.key2 = key2;
        self.akey2 = akey2;
    }

    pub fn encode(&self, text: &str) -> Result<String,CipherError> {
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
                    None => continue,
                };
                let x = v * self.key2 + self.key1;
                out.push(self.alpha.val_to_char(x))
            }
        }
        let val: String = out.iter().collect();
        Ok(val)
    }

    pub fn decode(&self, text: &str) -> Result<String,CipherError> {
        let ch = text.to_ascii_uppercase();
        let mut out = Vec::new();
        for c in ch.chars() {
            if c.is_ascii_whitespace() && self.whitespace {
                out.push(c);
            } else {
                let v = match self.alpha.char_to_val(c) {
                    Some(m) => m,
                    None => return Err(CipherError::new("unknown character encountered")),
                };
                let x = (v - self.key1)*self.akey2;
                out.push(self.alpha.val_to_char(x))
            }

        }
        let val: String = out.iter().collect();
        Ok(val)
    }
}

impl fmt::Display for Affine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Affine Cipher\nkey: ({},{})",self.key1.value(),self.key2.value())
    }
}

#[test]
fn affine() {
    let alpha = CipherAlphabet::new("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
    let mut aff = Affine::new((1,3),alpha);
    aff.set_whitespace(true);
    let plaintext = "the quick brown fox jumps over the lazy dog";
    let ciphertext = aff.encode(plaintext).unwrap();
    let cleartext = aff.decode(&ciphertext).unwrap();

    println!("{}\n{}\n{}",plaintext,ciphertext,cleartext);
    
}