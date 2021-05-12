use std::fmt;

use crate::alphabets::LATIN26;
use super::auxilliary::PORTA_TABLEAUX;




pub struct TableauxProgressiveKey<'a> {
    tableaux: Vec<&'a str>,
    key: &'a str,
    increment: usize,
    key_vals: Vec<usize>,
    alphabet: &'a str,
    length: usize,
}

impl TableauxProgressiveKey<'_> {
    pub fn new<'a>(key: &'a str, increment: usize, tableaux: Vec<&'a str>, alphabet: &'a str) -> TableauxProgressiveKey<'a> {
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
        TableauxProgressiveKey{ tableaux: tableaux.clone(), key, increment, key_vals, alphabet, length: alen }
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

    fn encrypt_pair(&self, c: char, k: usize, shift: usize) -> char {
        let n = self.tableaux[(k+shift)%self.length].chars().position(|x| x == c).unwrap();
        self.alphabet.chars().nth(n).unwrap()
    }

    fn decrypt_pair(&self, c: char, k: usize, shift: usize) -> char {
        let n = self.alphabet.chars().position(|x| x == c).unwrap();
        self.tableaux[(k+shift)%self.length].chars().nth(n).unwrap()
    }

}

impl crate::Cipher for TableauxProgressiveKey<'_> {

    fn encrypt(&self, text: &str) -> String {
        let mut out = String::new();
        let ckey = self.key_vals.iter().cycle();
        let mut ctr = 0;
        let mut shift = 0;
        for (c, k) in text.chars().zip(ckey) {
            out.push( self.encrypt_pair(c,*k,shift) );
            ctr = (ctr + 1) % self.key_vals.len();
            if ctr == 0 {
                shift = (shift + self.increment) % self.length
            }
        }
        out
    }

    fn decrypt(&self, text: &str) -> String {
        let mut out = String::new();
        let ckey = self.key_vals.iter().cycle();
        let mut ctr = 0;
        let mut shift = 0;
        for (c, k) in text.chars().zip(ckey) {
            out.push( self.decrypt_pair(c,*k,shift) );
            ctr = (ctr + 1) % self.key_vals.len();
            if ctr == 0 {
                shift = (shift + self.increment) % self.length
            }
        }
        out
    }

}

impl fmt::Display for TableauxProgressiveKey<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Tableaux Progressive Key Cipher\nkey: {:?}",self.key)
    }
}





pub struct PortaProgressiveKey<'a> {
    tableaux: Vec<&'a str>,
    key: &'a str,
    key_vals: Vec<usize>,
    increment: usize,
    alphabet: &'a str,
}

impl PortaProgressiveKey<'_> {
    pub fn default<'a>(key: &'a str, increment: usize) -> PortaProgressiveKey<'a> {
        let key_vals = key.chars().map(|c| LATIN26.chars().position(|x| x == c).unwrap() / 2 ).collect();
        PortaProgressiveKey{ tableaux: PORTA_TABLEAUX.clone(), key, key_vals, increment, alphabet: LATIN26 }
    }
}

impl crate::Cipher for PortaProgressiveKey<'_> {

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

    // The PortaProgressiveKey cipher is reciprocal
    fn decrypt(&self, text: &str) -> String {
        self.encrypt(text)
    }

}

impl fmt::Display for PortaProgressiveKey<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Porta Progressive Key Cipher\nkey: {:?}",self.key)
    }
}