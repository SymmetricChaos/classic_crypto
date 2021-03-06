use std::{fmt, io::Error};


pub trait Cipher { 
    fn encrypt(&self, text: &str) -> String;
    fn decrypt(&self, text: &str) -> String;
    fn encrypt_file(&self, source: &str, target: &str) -> Result<(),Error>;
    fn decrypt_file(&self, source: &str, target: &str) -> Result<(),Error>;
}

// For the Vigenere like family of ciphers
pub trait PolyalphabeticCipher: fmt::Display {
    fn encrypt_char(&self, t: usize, k: usize) -> usize;
    fn decrypt_char(&self, t: usize, k: usize) -> usize;
    fn text_to_nums(&self, text: &str) -> Vec<usize>;
    fn nums_to_text(&self, nums: Vec<usize>) -> String;
    fn key_vals(&self) -> Vec<usize>;
    fn alphabet_len(&self) -> usize;
}

pub trait Code { 
    fn encode(&self, text: &str) -> String;
    fn decode(&self, text: &str) -> String;
    fn char_map(&self) -> String;
}