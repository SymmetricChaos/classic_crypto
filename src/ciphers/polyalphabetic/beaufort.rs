use std::fmt;
use std::{fs::File, io::{Error, Read, Write}};

use crate::auxiliary::string_to_nums;


pub struct Beaufort<'a> {
    key_vals: Vec<usize>,
    key_name: &'a str,
    alphabet: &'a str,
    length: usize,
}

impl Beaufort<'_> {
    pub fn new<'a>(key: &'a str, alphabet: &'a str) -> Beaufort<'a> {
        let key_name = key;
        let key_vals: Vec<usize> = string_to_nums(key, alphabet);
        Beaufort{ key_vals, key_name, alphabet, length: alphabet.chars().count() }
    }

}

impl crate::PolyalphabeticCipher for Beaufort<'_> {
    fn encrypt_char(&self, t: usize, k: usize) -> usize {
        (self.length+k-t) % self.length
    }

    fn decrypt_char(&self, t: usize, k: usize) -> usize {
        (self.length+k-t) % self.length
    }

    fn text_to_nums(&self, text: &str) -> Vec<usize> {
        let nums: Vec<usize> = text.chars().map( |x| self.alphabet.chars().position(|c| c == x).unwrap() ).collect();
        nums
    }

    fn nums_to_text(&self, nums: Vec<usize>) -> String {
        nums.iter().map(|n| self.alphabet.chars().nth(*n).unwrap()).collect::<String>()
    }

    fn key_vals(&self) -> Vec<usize> {
        self.key_vals.clone()
    }

    fn alphabet_len(&self) -> usize {
        self.length
    }
}

impl crate::Cipher for Beaufort<'_> {

    fn encrypt(&self, text: &str) -> String {
        let nums: Vec<usize> = text.chars().map( |x| self.alphabet.chars().position(|c| c == x).unwrap() ).collect();
        let ckey = self.key_vals.iter().cycle();
        let mut out = "".to_string();
        for (n, k) in nums.iter().zip(ckey) {
            let m = (self.length + *k - *n) % self.length;
            out.push(self.alphabet.chars().nth( m ).unwrap() )
        }
        out
    }

    // The Beaufort cipher is reciprocal
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

impl fmt::Display for Beaufort<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Beaufort Cipher\nkey: {}",self.key_name)
    }
}