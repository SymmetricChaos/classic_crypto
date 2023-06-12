use std::{fmt, fs::File, io::{Error, Read, Write}};
use rand::Rng;

/// The Caesar Cipher is among the oldest of ciphers. It substitutes symbols simply by shifting them some distance along the alphabet
pub struct Caesar {
    key: usize,
    alphabet: String,
    length: usize,
}


impl Caesar {
    pub fn new(key: usize, alphabet: &str) -> Caesar {
        Caesar{ key, alphabet: alphabet.to_string(), length: alphabet.chars().count() }
    }

    pub fn rot13() -> Caesar {
        Caesar{ key: 13, alphabet: crate::alphabets::LATIN26.to_string(), length: 26 }
    }

    pub fn random(alphabet: &str) -> Caesar {
        let mut rng = rand::thread_rng();
        let length = alphabet.chars().count();
        let key = rng.gen_range(0..length);
        Caesar{ key, alphabet: alphabet.to_string(), length }
    }

    fn char_to_val(&self, c: char) -> usize {
        self.alphabet.chars().position(|x| x == c).unwrap()
    }

    fn val_to_char(&self, v: usize) -> char {
        self.alphabet.chars().nth(v).unwrap()
    }

}

impl crate::Cipher for Caesar {

    fn encrypt(&self, text: &str) -> String {
        let symbols = text.chars();
        let mut out = "".to_string();
        for s in symbols {
            let n = (self.char_to_val(s) + self.key) % self.length;
            out.push(self.val_to_char(n))
        }
        out
    }

    fn decrypt(&self, text: &str) -> String {
        let symbols = text.chars();
        let mut out = "".to_string();
        for s in symbols {
            let n = (self.char_to_val(s) + self.length - self.key) % self.length;
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

impl fmt::Display for Caesar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Caesar Cipher\nkey: {}",self.key)
    }
}