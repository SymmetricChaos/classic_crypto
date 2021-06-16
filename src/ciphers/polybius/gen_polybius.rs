use std::fmt;
use std::collections::HashMap;
use crate::alphabets::keyed_alphabet;
use itertools::Itertools;
use itertools::Permutations;

// Generalized polybius hypercube
pub struct GenPolybius<'a> {
    map: HashMap<char,String>,
    map_inv: HashMap<String,char>,
    alphabet: &'a str,
    labels: &'a str,
}

impl GenPolybius<'_> {
    pub fn new<'a>(keyword: &'a str, alphabet: &'a str, labels: &'a str, dimension: usize) -> GenPolybius<'a> {

        let alpha = keyed_alphabet(keyword,alphabet);

        //alpha.chars().permutations(dimension);

        let alen = alpha.chars().count();
        let llen = labels.chars().count();
        if alen > llen*llen*dimension {
            panic!("an alphabet with {} characters does not fit in an {}-cube with edges of length {}.",alen,dimension,llen)
        }

        let mut symbols = alpha.chars();
        let mut map = HashMap::new();
        let mut map_inv = HashMap::new();
        for a in labels.chars() {
            for b in labels.chars() {
                let t = (a,b);
                let c = symbols.next();
                if c.is_some() {
                    map.insert(c.unwrap(), t);
                    map_inv.insert(t, c.unwrap());
                }
            }
        }

        GenPolybius{ map, map_inv, alphabet, labels }
    }

    pub fn encrypt_to_vec(&self, text: &str) -> Vec<char> {
        let mut out = Vec::new();
        for c in text.chars() {
            let (a,b) = self.map[&c];
            out.push(a);
            out.push(b);
        }
        out
    }

    pub fn decrypt_from_vec(&self, text: &Vec<char>) -> String {
        let mut out = "".to_string();
        if  text.len() % 2 == 1 {
            panic!("GenPolybius Square Error: cannot decrypt a string with an odd number of characters")
        }
        let n_groups = text.len() / 2;
        let mut symbols = text.iter();
        for _ in 0..n_groups {
            let t = (*symbols.next().unwrap(),*symbols.next().unwrap());
            out.push(self.map_inv[&t]);
        }
        out
    }
}

impl crate::Cipher for GenPolybius<'_> {

    fn encrypt(&self, text: &str) -> String {
        let mut out = "".to_string();
        for c in text.chars() {
            let (a,b) = self.map[&c];
            let s = format!("{}{}",a,b);
            out.push_str(&s);
        }
        out
    }

    fn decrypt(&self, text: &str) -> String {
        let mut out = "".to_string();
        let tlen = text.chars().count();
        if tlen % 2 == 1 {
            panic!("GenPolybius Square Error: cannot decrypt a string with an odd number of characters")
        }
        let n_groups = tlen / 2;
        let mut symbols = text.chars();
        for _ in 0..n_groups {
            let t = (symbols.next().unwrap(),symbols.next().unwrap());
            out.push(self.map_inv[&t]);
        }
        out
    }

}

impl fmt::Display for GenPolybius<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut square = " ".to_string();
        for c in self.labels.chars() {
            square.push(' ');
            square.push(c)
        }
        square.push_str("\n");
        let mut symbols = self.alphabet.chars();
        for l in self.labels.chars() {
            square.push(l);
            for _ in 0..self.labels.chars().count() {
                square.push(' ');
                square.push(symbols.next().unwrap_or(' '))
            }
            square.push_str("\n");
        }
        write!(f, "GenPolybius Square\n{}",square)
    }
}




/* 
#[test]
fn GenPolybius() {
    use crate::alphabets::LATIN36;
    use crate::Cipher;

    let poly = GenPolybius::new("17ZEBRAS42",LATIN36,"123456");
    println!("{}",poly);
    let plaintext = "THEQUICKBROWNFOXJUMPSOVERTHELAZYDOG";
    let ciphertext = poly.encrypt(plaintext);
    let cleartext = poly.decrypt(&ciphertext);

    println!("{}\n{}\n{}",plaintext,ciphertext,cleartext);
}

#[test]
fn GenPolybius_vec() {
    use crate::alphabets::LATIN36;

    let poly = GenPolybius::new("17ZEBRAS42",LATIN36,"123456");
    println!("{}",poly);
    let plaintext = "THEQUICKBROWNFOXJUMPSOVERTHELAZYDOG";
    let ciphertext = poly.encrypt_to_vec(plaintext);
    let cleartext = poly.decrypt_from_vec(&ciphertext);

    println!("{}\n{:?}\n{}",plaintext,ciphertext,cleartext);
} */