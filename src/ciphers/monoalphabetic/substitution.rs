use std::fmt;
use std::collections::HashMap;





pub struct Substitution<'a> {
    /// The most general substitution cipher operates by replacing each symbol in the first alphabet with a corresponding symbol in another alphabet
    alphabet1: &'a str,
    alphabet2: &'a str,
    map: HashMap<char,char>,
    map_inv: HashMap<char,char>,
}

impl Substitution<'_> {
    pub fn new<'a>(alphabet1: &'a str, alphabet2: &'a str) -> Substitution<'a> {
        if alphabet1.chars().count() != alphabet2.chars().count() {
            panic!("alphabet1 is of length {} but alphabet2 is of length {}, they must be the same length",alphabet1.chars().count(),alphabet2.chars().count())
        }
        let mut map = HashMap::new();
        let mut map_inv = HashMap::new();
        for (a, b) in alphabet1.chars().zip(alphabet2.chars()) {
            map.insert(a, b);
            map_inv.insert(b, a);
        }
        Substitution{ alphabet1, alphabet2, map, map_inv }
    }

}

impl crate::Cipher for Substitution<'_> {

    fn encrypt(&self, text: &str) -> String {
        let mut out = "".to_string();
        for c in text.chars() {
            out.push(self.map[&c])
        }
        out
    }

    fn decrypt(&self, text: &str) -> String {
        let mut out = String::new();
        for c in text.chars() {
            out.push(self.map_inv[&c])
        }
        out
    }

}

impl fmt::Display for Substitution<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Simple Substitution\n{}\n{}",self.alphabet1,self.alphabet2)
    }
}





pub struct Atbash<'a> {
    /// The Atbash Cipher is a special case of the general Substitution Cipher in which a single alphabet is used and characters are tranformed into their opposite. This is a reciprocal cipher.
    alphabet: &'a str,
    map: HashMap<char,char>,
}

impl Atbash<'_> {
    pub fn new<'a>(alphabet: &'a str) -> Atbash<'a> {
        let mut map = HashMap::new();
        for (a, b) in alphabet.chars().zip(alphabet.chars().rev()) {
            map.insert(a, b);
        }
        Atbash{ alphabet, map }
    }

}

impl crate::Cipher for Atbash<'_> {

    fn encrypt(&self, text: &str) -> String {
        let mut out = "".to_string();
        for c in text.chars() {
            out.push(self.map[&c])
        }
        out
    }

    fn decrypt(&self, text: &str) -> String {
        self.encrypt(text)
    }

}

impl fmt::Display for Atbash<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Atbash\n{}\n{}",self.alphabet,self.alphabet.chars().rev().collect::<String>())
    }
}