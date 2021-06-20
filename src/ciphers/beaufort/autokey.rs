use std::{collections::VecDeque, fmt};

fn beaufort_encrypt(n: usize, k: usize, l: usize) -> usize {
    (l + k - n) % l
}



pub struct Autokey<'a> {
    key_vals: Vec<usize>,
    key: &'a str,
    alphabet: &'a str,
    length: usize,
}

impl Autokey<'_> {
    pub fn new<'a>(key: &'a str, alphabet: &'a str) -> Autokey<'a> {
        let key_vals: Vec<usize> = key.chars().map( |x| alphabet.chars().position(|c| c == x).unwrap() ).collect();
        Autokey{ key, key_vals, alphabet, length: alphabet.chars().count() }
    }
}

impl crate::Cipher for Autokey<'_> {

    fn encrypt(&self, text: &str) -> String {
        let mut out = "".to_string();
        let mut akey: VecDeque<usize> = self.key_vals.clone().into_iter().collect();
        let nums: Vec<usize> = text.chars().map( |x| self.alphabet.chars().position(|c| c == x).unwrap() ).collect();
        for n in nums {
            akey.push_back(n);
            let k = akey.pop_front().unwrap();
            let m = beaufort_encrypt(n,k,self.length);
            out.push(self.alphabet.chars().nth( m ).unwrap() )
        }
        out
    }

    // The Autokey is not involutive like the Beaufort
    fn decrypt(&self, text: &str) -> String {
        let mut out = "".to_string();
        let mut akey: VecDeque<usize> = self.key_vals.clone().into_iter().collect();
        let nums: Vec<usize> = text.chars().map( |x| self.alphabet.chars().position(|c| c == x).unwrap() ).collect();
        for n in nums {
            let k = akey.pop_front().unwrap();
            let m = beaufort_encrypt(n,k,self.length);
            akey.push_back(m);
            out.push(self.alphabet.chars().nth( m ).unwrap() )
        }
        out
    }

}

impl fmt::Display for Autokey<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Beaufort Autokey Cipher\nkey: {:?}",self.key)
    }
}