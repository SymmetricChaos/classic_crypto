use std::fmt;
use lazy_static::lazy_static;


lazy_static! {

}

/// Polyalphabetic substitution ciphers (like the Beaufort, Porta, and Vigenere) are all special cases of the Tableaux Cipher with vastly simpler keys.
pub struct Tableaux<'a> {
    tableaux: Vec<&'a str>,
    key: &'a str,
    key_vals: Vec<usize>,
    alphabet: &'a str,
}

impl Tableaux<'_> {
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
        let key_vals = key.chars().map(|c| LATIN26.chars().position(|x| x == c).unwrap() ).collect();
        Tableaux{ tableaux: tableaux.clone(), key, key_vals, alphabet }
    }

    pub fn tableaux(&self) -> String {
        let mut out = String::new();
        for row in self.tableaux.iter() {
            out.push_str(row);
            out.push_str("\n")
        }
        out
    }

}

impl crate::Cipher for Tableaux<'_> {

    fn encrypt(&self, text: &str) -> String {
        let mut out = String::new();
        let ckey = self.key_vals.iter().cycle();
        for (c, k) in text.chars().zip(ckey) {
            let p = self.tableaux[*k].chars().position(|x| x == c).unwrap();
            out.push(self.alphabet.chars().nth(p).unwrap())
        }
        out
    }

    fn decrypt(&self, text: &str) -> String {
        
    }

}

impl fmt::Display for Tableaux<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Tableaux Cipher\nkey: {:?}",self.key)
    }
}