use std::fmt;
use std::{fs::File, io::{Error, Read, Write}};

use crate::auxiliary::mul_inv;


/// The Affine Cipher augments the Caesar Cipher by including a multiplicative aspect to the key.
pub struct Affine {
    key1: usize,
    key2: usize,
    key2_inv: usize,
    alphabet: String,
    length: usize
}

impl Affine {
    pub fn new(key: (usize,usize), alphabet: &str) -> Affine {
        let key1 = key.0;
        let key2 = key.1;
        let length = alphabet.chars().count();
        let key2_inv = match mul_inv(key2, length) {
            Some(m) => m,
            None => panic!("{} has no multiplicative inverse modulo {}",key2,length),
        };

        Affine{ key1, key2, key2_inv, alphabet: alphabet.to_string(), length }
    }

    fn char_to_val(&self, c: char) -> usize {
        self.alphabet.chars().position(|x| x == c).unwrap()
    }

    fn val_to_char(&self, v: usize) -> char {
        self.alphabet.chars().nth(v).unwrap()
    }

}

impl crate::Cipher for Affine {

    fn encrypt(&self, text: &str) -> String {
        let symbols = text.chars();
        let mut out = "".to_string();
        for s in symbols {
            let n = (self.char_to_val(s) * self.key2 + self.key1) % self.length;
            out.push(self.val_to_char(n))
        }
        out
    }

    fn decrypt(&self, text: &str) -> String {
        let symbols = text.chars();
        let mut out = "".to_string();
        for s in symbols {
            let n = ((self.char_to_val(s) + self.length - self.key1) * self.key2_inv) % self.length;
            out.push(self.val_to_char(n))
        }
        out
    }

    fn encrypt_file(&self, source: &str, target: &str) -> Result<(),Error> {

        let mut target_file = File::create(target.to_string())?;
    
        let mut source_file = File::open(source)?;
        let mut source_text = String::new();
        source_file.read_to_string(&mut source_text)?;
    
        let ciphertext = self.encrypt(&source_text);
    
        target_file.write(ciphertext.as_bytes())?;

        Ok(())
    }

    fn decrypt_file(&self, source: &str, target: &str) -> Result<(),Error> {

        let mut target_file = File::create(target.to_string())?;
    
        let mut source_file = File::open(source)?;
        let mut source_text = String::new();
        source_file.read_to_string(&mut source_text)?;
    
        let ciphertext = self.decrypt(&source_text);
    
        target_file.write(ciphertext.as_bytes())?;

        Ok(())
    }

}

impl fmt::Display for Affine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Affine Cipher\nkey: ({},{})",self.key1,self.key2)
    }
}
