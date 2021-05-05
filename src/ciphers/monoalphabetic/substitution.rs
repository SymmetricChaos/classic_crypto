use std::fmt;
use std::collections::HashMap;


/// The most general substitution cipher operates by replacing each symbol in the first alphabet with a corresponding symbol in another alphabet
pub struct Substitution {
    alphabet1: String,
    alphabet2: String,
    map: HashMap<char,char>,
    map_inv: HashMap<char,char>,
}

impl Substitution {
    pub fn new(alphabet1: &str, alphabet2: &str) -> Substitution {
        if alphabet1.chars().count() != alphabet2.chars().count() {
            panic!("alphabet1 is of length {} but alphabet2 is of length {}",alphabet1.chars().count(),alphabet2.chars().count())
        }
        let mut map = HashMap::new();
        let mut map_inv = HashMap::new();
        for (a, b) in alphabet1.chars().zip(alphabet2.chars()) {
            if a.is_whitespace() || b.is_whitespace() {
                panic!("whitespace is not allowed as a substitution symbol")
            }
            map.insert(a, b);
            map_inv.insert(b, a);
        }
        Substitution{ alphabet1: alphabet1.to_string(), alphabet2: alphabet2.to_string(), map, map_inv }
    }

    pub fn atbash(alphabet: &str) -> Substitution {
        let alphabet1 = alphabet.to_string();
        let alphabet2: String = alphabet.chars().rev().collect();
        let mut map = HashMap::new();
        let mut map_inv = HashMap::new();
        for (a, b) in alphabet1.chars().zip(alphabet2.chars()) {
            if a.is_whitespace() || b.is_whitespace() {
                panic!("whitespace is not allowed as a substitution symbol")
            }
            map.insert(a, b);
            map_inv.insert(b, a);
        }
        Substitution{ alphabet1, alphabet2, map, map_inv }
    }

}

impl crate::auxiliary::Cipher for Substitution {

    fn encrypt(&self, text: &str) -> String {
        let mut out = "".to_string();
        for c in text.chars() {
            out.push(self.map[&c])
        }
        out
    }

    fn decrypt(&self, text: &str) -> String {
        let mut out = "".to_string();
        for c in text.chars() {
            out.push(self.map_inv[&c])
        }
        out
    }

}

impl fmt::Display for Substitution {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Simple Substitution\n{}\n{}",self.alphabet1,self.alphabet2)
    }
}

#[test]
fn substitution() {
    use crate::Cipher;
    let substitution = Substitution::new("abcdefghijklmnopqrstuvwxyz","QWERTYUIOPASDFGHJKLZXCVBNM");
    
    println!("{}\n",substitution);

    let plaintext = "thequickbrownfoxjumpsoverthelazydog";
    let ciphertext = substitution.encrypt(plaintext);
    let decrypt = substitution.decrypt(&ciphertext);

    println!("{}\n{}\n{}\n",plaintext,ciphertext,decrypt)

}

#[test]
fn substitution_unicode() {
    use crate::Cipher;
    use crate::alphabets::LATIN26;
    let substitution = Substitution::new(LATIN26,"🌰🌱🌲🌳🌴🌵🌶️🌷🌸🌹🌺🌻🌼🌽🌾🌿🍀🍁🍂🍃🍄🍅🍆🍇🍈");
    
    let plaintext = "THEQUICKBROWNFOXJUMPSOVERTHELAZYDOG";
    let ciphertext = substitution.encrypt(plaintext);
    let decrypted = substitution.decrypt(&ciphertext);

    println!("{}\n{}\n{}\n",plaintext,ciphertext,decrypted)

    
}