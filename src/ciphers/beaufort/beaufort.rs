use std::fmt;

fn beaufort_encrypt(n: usize, k: usize, l: usize) -> usize {
    (l + k - n) % l
}

pub struct Beaufort<'a> {
    key_vals: Vec<usize>,
    key: &'a str,
    alphabet: &'a str,
    length: usize,
}

impl Beaufort<'_> {
    pub fn new<'a>(key: &'a str, alphabet: &'a str) -> Beaufort<'a> {
        let key_vals: Vec<usize> = key.chars().map(|x| alphabet.chars().position(|c| c == x).unwrap()).collect();
        Beaufort{ key, key_vals, alphabet, length: alphabet.chars().count() }
    }
}

impl crate::Cipher for Beaufort<'_> {

    fn encrypt(&self, text: &str) -> String {
        let nums: Vec<usize> = text.chars().map( |x| self.alphabet.chars().position(|c| c == x).unwrap() ).collect();
        let ckey = self.key_vals.iter().cycle();
        let mut out = "".to_string();
        for (n,k) in nums.iter().zip(ckey) {
            let m = beaufort_encrypt(*n,*k,self.length);
            out.push(self.alphabet.chars().nth( m ).unwrap() )
        }
        out
    }

    // The Beaufort cipher is involutive
    fn decrypt(&self, text: &str) -> String {
        self.encrypt(text)
    }

}

impl fmt::Display for Beaufort<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Beaufort Cipher\nkey: {:?}",self.key)
    }
}