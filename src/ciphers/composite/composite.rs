use std::fmt;

use crate::auxiliary::Cipher;

/// A composite cipher applies several ciphers in succession. 
pub struct CompositeCipher <'a> {
    ciphers: Vec<&'a dyn Cipher>,
}


impl <'a> CompositeCipher <'a> {
    pub fn new(ciphers: Vec<&'a dyn Cipher>) -> CompositeCipher {
        CompositeCipher{ ciphers }
    }

    pub fn encrypt(&self, text: &str) -> String {
        let mut out = text.to_string();
        for cipher in self.ciphers.iter() {
            out = cipher.encrypt(&out)
        }
        out
    }

    pub fn decrypt(&self, text: &str) -> String {
        let mut out = text.to_string();
        for cipher in self.ciphers.iter().rev() {
            out = cipher.decrypt(&out)
        }
        out
    }
}

impl fmt::Display for CompositeCipher<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Composite Cipher")
    }
}
