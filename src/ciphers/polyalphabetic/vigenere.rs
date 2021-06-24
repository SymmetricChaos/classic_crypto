use std::fmt;

use crate::auxiliary::string_to_nums;


pub struct Vigenere<'a> {
    key_vals: Vec<usize>,
    key_name: &'a str,
    alphabet: &'a str,
    length: usize,
}

impl Vigenere<'_> {
    pub fn new<'a>(key: &'a str, alphabet: &'a str) -> Vigenere<'a> {
        let key_name = key;
        let key_vals: Vec<usize> = string_to_nums(key, alphabet);
        Vigenere{ key_vals, key_name, alphabet, length: alphabet.chars().count() }
    }

}

impl crate::PolyalphabeticCipher for Vigenere<'_> {
    fn encrypt_char(&self, t: usize, k: usize) -> usize {
        (t+k) % self.length
    }

    fn decrypt_char(&self, t: usize, k: usize) -> usize {
        (self.length+t-k) % self.length
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
}

impl crate::Cipher for Vigenere<'_> {

    fn encrypt(&self, text: &str) -> String {
        let nums: Vec<usize> = text.chars().map( |x| self.alphabet.chars().position(|c| c == x).unwrap() ).collect();
        let ckey = self.key_vals.iter().cycle();
        let mut out = "".to_string();
        for (n,k) in nums.iter().zip(ckey) {
            out.push(self.alphabet.chars().nth( (n+k)%self.length ).unwrap() )
        }
        out
    }

    fn decrypt(&self, text: &str) -> String {
        let nums: Vec<usize> = text.chars().map( |x| self.alphabet.chars().position(|c| c == x).unwrap() + self.length ).collect();
        let ckey = self.key_vals.iter().cycle();
        let mut out = "".to_string();
        for (n,k) in nums.iter().zip(ckey) {
            out.push(self.alphabet.chars().nth( (n-k)%self.length ).unwrap() )
        }
        out
    }

}

impl fmt::Display for Vigenere<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Vigenere Cipher\nkey: {}",self.key_name)
    }
}