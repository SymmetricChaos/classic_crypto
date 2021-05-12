use std::fmt;

use crate::alphabets::LATIN26;
use super::auxilliary::PORTA_TABLEAUX;


/// Polyalphabetic substitution ciphers (like the Beaufort, Porta, and Vigenere) are all special cases of the Tableaux Cipher with vastly simpler keys.
pub struct Tableaux<'a> {
    tableaux: Vec<&'a str>,
    key: &'a str,
    key_vals: Vec<usize>,
    alphabet: &'a str,
}

impl Tableaux<'_> {
    // It would be more efficient if this were only using the tableaux rows that are in the key
    pub fn new<'a>(key: &'a str, tableaux: Vec<&'a str>, alphabet: &'a str) -> Tableaux<'a> {
        let alen = alphabet.chars().count();
        if tableaux.len() != alen {
            panic!("the tableaux must have exactly one row for each character in the alphabet")
        }
        for row in tableaux.iter() {
            if row.chars().count() != alen {
                panic!("all rows of the tableaux must have the same length as the alphabet")
            }
            for c in row.chars() {
                if !alphabet.contains(c) {
                    panic!("all rows of the tableaux must be permutations of the alphabet")
                }
            }
        }
        let key_vals = key.chars().map(|c| alphabet.chars().position(|x| x == c).unwrap() ).collect();
        Tableaux{ tableaux: tableaux.clone(), key, key_vals, alphabet }
    }

    pub fn tableaux(&self) -> String {
        let mut out = "  ".to_string();
        for c in self.alphabet.chars() {
            out.push(c);
            out.push(' ')
        }
        out.push_str("\n");
        for (row, c) in self.tableaux.iter().zip(self.alphabet.chars()) {
            out.push(c);
            out.push(' ');
            for r in row.chars() {
                out.push(r);
                out.push(' ')
            }
            out.push_str("\n")
        }
        out
    }

    fn encrypt_pair(&self, c: char, k: usize) -> char {
        let n = self.tableaux[k].chars().position(|x| x == c).unwrap();
        self.alphabet.chars().nth(n).unwrap()
    }

    fn decrypt_pair(&self, c: char, k: usize) -> char {
        let n = self.alphabet.chars().position(|x| x == c).unwrap();
        self.tableaux[k].chars().nth(n).unwrap()
    }

}

impl crate::Cipher for Tableaux<'_> {

    fn encrypt(&self, text: &str) -> String {
        let mut out = String::new();
        let ckey = self.key_vals.iter().cycle();
        for (c, k) in text.chars().zip(ckey) {
            out.push( self.encrypt_pair(c,*k) )
        }
        out
    }

    fn decrypt(&self, text: &str) -> String {
        let mut out = String::new();
        let ckey = self.key_vals.iter().cycle();
        for (c, k) in text.chars().zip(ckey) {
            out.push( self.decrypt_pair(c,*k) )
        }
        out
    }

}

impl fmt::Display for Tableaux<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Tableaux Cipher\nkey: {:?}",self.key)
    }
}








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

}

impl fmt::Display for Porta<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Porta Cipher\nkey: {:?}",self.key)
    }
}