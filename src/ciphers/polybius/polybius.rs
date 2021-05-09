use std::fmt;
use std::collections::HashMap;
use crate::auxiliary::keyed_alphabet;


// Less memory intensive method?
pub struct Polybius<'a> {
    map: HashMap<char,(char,char)>,
    map_inv: HashMap<(char,char),char>,
    alphabet: &'a str,
    labels: &'a str,
}

impl Polybius<'_> {
    pub fn new<'a>(keyword: &'a str, alphabet: &'a str, labels: &'a str) -> Polybius<'a> {

        let alpha = keyed_alphabet(keyword,alphabet);

        let alen = alpha.chars().count();
        let llen = labels.chars().count();
        if alen > llen*llen {
            panic!("an alphabet with {} characters does not fit in a {}x{} square.",alen,llen,llen)
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

        Polybius{ map, map_inv, alphabet, labels }
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
            panic!("Polybius Square Error: cannot decrypt a string with an odd number of characters")
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

impl crate::auxiliary::Cipher for Polybius<'_> {

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
            panic!("Polybius Square Error: cannot decrypt a string with an odd number of characters")
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

impl fmt::Display for Polybius<'_> {
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
        write!(f, "Polybius Square\n{}",square)
    }
}





#[test]
fn polybius() {
    use crate::alphabets::LATIN36;
    use crate::Cipher;

    let poly = Polybius::new("17ZEBRAS42",LATIN36,"123456");
    println!("{}",poly);
    let plaintext = "THEQUICKBROWNFOXJUMPSOVERTHELAZYDOG";
    let ciphertext = poly.encrypt(plaintext);
    let cleartext = poly.decrypt(&ciphertext);

    println!("{}\n{}\n{}",plaintext,ciphertext,cleartext);
}

#[test]
fn polybius_vec() {
    use crate::alphabets::LATIN36;

    let poly = Polybius::new("17ZEBRAS42",LATIN36,"123456");
    println!("{}",poly);
    let plaintext = "THEQUICKBROWNFOXJUMPSOVERTHELAZYDOG";
    let ciphertext = poly.encrypt_to_vec(plaintext);
    let cleartext = poly.decrypt_from_vec(&ciphertext);

    println!("{}\n{:?}\n{}",plaintext,ciphertext,cleartext);
}