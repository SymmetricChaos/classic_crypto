use std::fmt;
use std::collections::VecDeque;

use crate::auxiliary::string_to_nums;


pub struct Chaocipher<'a> {
    alpha_l: VecDeque<char>,
    alpha_r: VecDeque<char>,
    alphabet: &'a str,
}

impl Chaocipher<'_> {
    pub fn new<'a>(key: &'a str, alphabet: &'a str) -> Chaocipher<'a> {
        let alpha_l: VecDeque = alphabet.chars().collect();
        let alpha_r: VecDeque = alphabet.chars().collect();
        Chaocipher{ alpha_l, alpha_r, alphabet }
    }

    fn permute_l(&mut self, n: usize) {
        self.alpha_l.rotate_left(n)
    }

    fn permute_r(&mut self, n: usize) {
        self.alpha_r.rotate_left(n)
    }
}

impl crate::Cipher for Chaocipher<'_> {

    fn encrypt(&self, text: &str) -> String {

        let mut out = String::new();
        out

    }

    fn decrypt(&self, text: &str) -> String {

        let mut out = String::new();
        out
        
    }

}

/* impl fmt::Display for Chaocipher<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Chaocipher Cipher\nkey: {}",self.key_name)
    }
} */