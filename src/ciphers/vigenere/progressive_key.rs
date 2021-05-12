use std::fmt;

use crate::auxiliary::string_to_nums;



pub struct ProgressiveKey<'a> {
    /// The Progressive Key version of the Vigenere Cipher stretches the key by shifting it along the alphabet by some increment each time it would repeat. It is bet for the increment to be coprime to the length of the alphabet as this ensures the maximum period before repetition.
    key_vals: Vec<usize>,
    increment: usize,
    key_name: &'a str,
    alphabet: &'a str,
    length: usize,
}

impl ProgressiveKey<'_> {
    pub fn new<'a>(key: &'a str, increment: usize, alphabet: &'a str) -> ProgressiveKey<'a> {
        let key_name = key;
        let key_vals: Vec<usize> = string_to_nums(key, alphabet);
        ProgressiveKey{ key_vals, increment, key_name, alphabet, length: alphabet.chars().count() }
    }
}

impl crate::auxiliary::Cipher for ProgressiveKey<'_> {

    fn encrypt(&self, text: &str) -> String {
        let mut shift = 0;
        let nums: Vec<usize> = text.chars().map( |x| self.alphabet.chars().position(|c| c == x).unwrap() ).collect();
        let ckey = self.key_vals.iter().cycle();
        let mut out = String::new();
        let mut ctr = 0;
        let klen = self.key_vals.len();
        let alen = self.length;
        for (n,k) in nums.iter().zip(ckey) {
            out.push(self.alphabet.chars().nth( (n+k+shift)%alen ).unwrap() );
            ctr = (ctr + 1) % klen;
            if ctr == 0 {
                shift = (shift + self.increment) % alen
            }
        }
        out
    }

    fn decrypt(&self, text: &str) -> String {
        let mut shift = 0;
        let nums: Vec<usize> = text.chars().map( |x| self.alphabet.chars().position(|c| c == x).unwrap() ).collect();
        let ckey = self.key_vals.iter().cycle();
        let mut out = String::new();
        let mut ctr = 0;
        let klen = self.key_vals.len();
        let alen = self.length;
        for (n,k) in nums.iter().zip(ckey) {
            out.push(self.alphabet.chars().nth( (alen+n-(k+shift))%alen ).unwrap() );
            ctr = (ctr + 1) % klen;
            if ctr == 0 {
                shift = (shift + self.increment) % alen
            }
        }
        out
    }

}

impl fmt::Display for ProgressiveKey<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Vigenere Progressive Key Cipher\nkey: {}\nincrement: {}",self.key_name,self.increment)
    }
}