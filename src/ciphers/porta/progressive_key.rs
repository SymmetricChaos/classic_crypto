use std::fmt;

use crate::alphabets::LATIN26;
use super::auxilliary::PORTA_TABLEAUX;


pub struct ProgressiveKey<'a> {
    tableaux: Vec<&'a str>,
    key: &'a str,
    key_vals: Vec<usize>,
    increment: usize,
    alphabet: &'a str,
}

impl ProgressiveKey<'_> {
    pub fn default<'a>(key: &'a str, increment: usize) -> ProgressiveKey<'a> {
        let key_vals = key.chars().map(|c| LATIN26.chars().position(|x| x == c).unwrap() / 2 ).collect();
        ProgressiveKey{ tableaux: PORTA_TABLEAUX.clone(), key, key_vals, increment, alphabet: LATIN26 }
    }
}

impl crate::Cipher for ProgressiveKey<'_> {

    fn encrypt(&self, text: &str) -> String {
        let mut out = String::new();
        let ckey = self.key_vals.iter().cycle();
        let mut ctr = 0;
        let mut shift = 0;
        let klen = self.key_vals.len();
        for (c, k) in text.chars().zip(ckey) {
            let p = self.tableaux[(*k + shift) % 13].chars().position(|x| x == c).unwrap();
            out.push(self.alphabet.chars().nth(p).unwrap());
            ctr = (ctr + 1) % klen;
            if ctr == 0 {
                shift = (shift + self.increment) % 13
            }
        }
        out
    }

    // The Porta ProgressiveKey cipher is reciprocal
    fn decrypt(&self, text: &str) -> String {
        self.encrypt(text)
    }

}

impl fmt::Display for ProgressiveKey<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Porta Progressive Key Cipher\nkey: {:?}",self.key)
    }
}