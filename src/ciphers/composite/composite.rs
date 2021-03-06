use std::{fs::File, io::{Error, Read, Write}};

use crate::Cipher;

/// A composite cipher applies several ciphers in succession.
pub struct CompositeCipher<'a> {
    ciphers: Vec<Box<&'a dyn Cipher>>,
}


impl CompositeCipher<'_> {
    pub fn new(ciphers: Vec<&dyn Cipher>) -> CompositeCipher {
        let mut cipher_vec = Vec::new();
        for c in ciphers {
            cipher_vec.push(Box::new(c))
        }
        CompositeCipher{ ciphers: cipher_vec }
    }

    pub fn encrypt_steps(&self, text: &str) -> Vec<String> {
        let mut out = vec![text.to_string()];
        let mut partial = text.to_string();
        for cipher in self.ciphers.iter() {
            partial = cipher.encrypt(&partial);
            out.push(partial.clone());
        }
        out
    }
}

impl crate::Cipher for CompositeCipher<'_> {

    fn encrypt(&self, text: &str) -> String {
        let mut out = text.to_string();
        for cipher in self.ciphers.iter() {
            out = cipher.encrypt(&out)
        }
        out
    }

    fn decrypt(&self, text: &str) -> String {
        let mut out = text.to_string();
        for cipher in self.ciphers.iter().rev() {
            out = cipher.decrypt(&out)
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