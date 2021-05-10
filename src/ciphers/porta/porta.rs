use std::fmt;
use lazy_static::lazy_static;

use crate::alphabets::LATIN26;

lazy_static! {
    pub static ref PORTA_TABLEAUX: Vec<&'static str> = vec![
        "NOPQRSTUVWXYZABCDEFGHIJKLM",
        "OPQRSTUVWXYZNMABCDEFGHIJKL",
        "PQRSTUVWXYZNOLMABCDEFGHIJK",
        "QRSTUVWXYZNOPKLMABCDEFGHIJ",
        "RSTUVWXYZNOPQJKLMABCDEFGHI",
        "STUVWXYZNOPQRIJKLMABCDEFGH",
        "TUVWXYZNOPQRSHIJKLMABCDEFG",
        "UVWXYZNOPQRSTGHIJKLMABCDEF",
        "VWXYZNOPQRSTUFGHIJKLMABCDE",
        "WXYZNOPQRSTUVEFGHIJKLMABCD",
        "XYZNOPQRSTUVWDEFGHIJKLMABC",
        "YZNOPQRSTUVWXCDEFGHIJKLMAB",
        "ZNOPQRSTUVWXYBCDEFGHIJKLMA"];
    
}


pub struct Porta<'a> {
    tableaux: Vec<&'a str>,
    key: &'a str,
    key_vals: Vec<usize>,
    alphabet: &'a str,
}

impl Porta<'_> {
    // need a better version of this
    pub fn new<'a>(key: &'a str, tableaux: Vec<&'a str>, alphabet: &'a str) -> Porta<'a> {
        let key_vals = key.chars().map(|c| LATIN26.chars().position(|x| x == c).unwrap() / 2 ).collect();
        Porta{ tableaux: tableaux.clone(), key, key_vals, alphabet }
    }

    pub fn default<'a>(key: &'a str) -> Porta<'a> {
        let key_vals = key.chars().map(|c| LATIN26.chars().position(|x| x == c).unwrap() / 2 ).collect();
        Porta{ tableaux: PORTA_TABLEAUX.clone(), key, key_vals, alphabet: LATIN26 }
    }
}

impl crate::Cipher for Porta<'_> {

    fn encrypt(&self, text: &str) -> String {
        let mut out = String::new();
        let ckey = self.key_vals.iter().cycle();
        let nums: Vec<usize> = text.chars().map(|c| self.alphabet.chars().position(|x| x == c).unwrap() / 2 ).collect();
        for (n, k) in nums.iter().zip(ckey) {
            out.push(self.tableaux[*k].chars().nth(*n).unwrap())
        }
        out
    }

    // The Porta cipher is reciprocal
    fn decrypt(&self, text: &str) -> String {
        self.encrypt(text)
    }

}

impl fmt::Display for Porta<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Porta Cipher\nkey: {:?}",self.key)
    }
}