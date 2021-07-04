use std::fmt;
use std::{fs::File, io::{Error, Read, Write}};

use crate::alphabets::LATIN26;
use super::auxilliary::PORTA_TABLEAUX;


/// Porta Cipher uses a set of 13 alphabets (called a tableaux) to encrypt characters. The alphabets were chosen by Porta to make the cipher reciprocal.
pub struct Porta<'a> {
    tableaux: Vec<&'a str>,
    key: &'a str,
    key_vals: Vec<usize>,
    alphabet: &'a str,
}

impl Porta<'_> {
    pub fn default<'a>(key: &'a str) -> Porta<'a> {
        let key_vals = key.chars().map(|c| LATIN26.chars().position(|x| x == c).unwrap() / 2 ).collect();
        Porta{ tableaux: PORTA_TABLEAUX.clone(), key, key_vals, alphabet: LATIN26 }
    }
}

impl crate::Cipher for Porta<'_> {

    fn encrypt(&self, text: &str) -> String {
        let mut out = String::new();
        let ckey = self.key_vals.iter().cycle();
        for (c, k) in text.chars().zip(ckey) {
            let p = self.tableaux[*k].chars().position(|x| x == c).unwrap();
            out.push(self.alphabet.chars().nth(p).unwrap())
        }
        out
    }

    // The Porta cipher is reciprocal
    fn decrypt(&self, text: &str) -> String {
        self.encrypt(text)
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

impl fmt::Display for Porta<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Porta Cipher\nkey: {:?}",self.key)
    }
}