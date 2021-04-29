use std::fmt;
use std::collections::VecDeque;


pub struct Autokey {
    key: Vec<usize>,
    key_name: String,
    alphabet: String,
    length: usize,
}

impl Autokey {
    pub fn new(key: &str, alpha: &str) -> Autokey {
        let key_name = key.to_string();
        let key: Vec<usize> = key.chars().map(|x| alpha.chars().position(|c| c == x).unwrap()).collect();

        Autokey{ key, key_name, alphabet: alpha.to_string(), length: alpha.chars().count() }
    }

    pub fn encrypt(&self, text: &str) -> String {
        let mut out = "".to_string();
        let mut akey: VecDeque<usize> = self.key.clone().into_iter().collect();
        let nums: Vec<usize> = text.chars().map( |x| self.alphabet.chars().position(|c| c == x).unwrap() ).collect();
        for n in nums {
            akey.push_back(n);
            let k = akey.pop_front().unwrap();
            out.push(self.alphabet.chars().nth( (n+k)%self.length ).unwrap() )
        }
        out
    }

    pub fn decrypt(&self, text: &str) -> String {
        let mut out = "".to_string();
        let mut akey: VecDeque<usize> = self.key.clone().into_iter().collect();
        let nums: Vec<usize> = text.chars().map( |x| self.alphabet.chars().position(|c| c == x).unwrap() ).collect();
        for n in nums {
            let k = akey.pop_front().unwrap();
            let v = (n + self.length - k ) % self.length;
            let c = self.alphabet.chars().nth( v ).unwrap();
            out.push(c);
            akey.push_back(v);
        }
        out
    }
}

impl fmt::Display for Autokey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Autokey Cipher\nkey: {:?}",self.key_name)
    }
}

#[test]
fn autokey() {
    use crate::auxiliary::LATIN26;
let auto = Autokey::new("SECRET", LATIN26);
    let plaintext = "THEQUICKBROWNFOXJUMPSOVERTHELAZYDOG";
    let ciphertext = auto.encrypt(plaintext);
    let cleartext = auto.decrypt(&ciphertext);

    println!("{}\n{}\n{}",plaintext,ciphertext,cleartext);
    
}