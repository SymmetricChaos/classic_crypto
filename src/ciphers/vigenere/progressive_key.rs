use std::fmt;





pub struct ProgressiveKey<'a> {
    key_vals: Vec<usize>,
    increment: usize,
    key_name: &'a str,
    alphabet: &'a str,
    length: usize,
}

impl ProgressiveKey<'_> {
    pub fn new<'a>(key: &'a str, increment: usize, alphabet: &'a str) -> ProgressiveKey<'a> {
        let key_name = key;
        let key_vals: Vec<usize> = key.chars().map(|x| alphabet.chars().position(|c| c == x).unwrap()).collect();
        ProgressiveKey{ key_vals, increment, key_name, alphabet, length: alphabet.chars().count() }
    }
}

impl crate::auxiliary::Cipher for ProgressiveKey<'_> {

    fn encrypt(&self, text: &str) -> String {
        let mut shift = 0;
        let nums: Vec<usize> = text.chars().map( |x| self.alphabet.chars().position(|c| c == x).unwrap() ).collect();
        let ckey = self.key_vals.iter().cycle();
        let mut out = String::new();
        let mut ctr = 0;
        let key_len = self.key_vals.len();
        for (n,k) in nums.iter().zip(ckey) {
            out.push(self.alphabet.chars().nth( (n+k+shift)%self.length ).unwrap() );
            ctr = (ctr+1)%key_len;
            if ctr == 0 {
                shift += self.increment
            }
        }
        out
    }

    fn decrypt(&self, text: &str) -> String {
        let mut shift = 0;
        let nums: Vec<usize> = text.chars().map( |x| self.alphabet.chars().position(|c| c == x).unwrap() ).collect();
        let ckey = self.key_vals.iter().cycle();
        let mut out = String::new();
        let mut ctr = 0;
        let key_len = self.key_vals.len();
        for (n,k) in nums.iter().zip(ckey) {
            out.push(self.alphabet.chars().nth( (self.length+n-(k+shift))%self.length ).unwrap() );
            ctr = (ctr+1)%key_len;
            if ctr == 0 {
                shift += self.increment
            }
        }
        out
    }

}

impl fmt::Display for ProgressiveKey<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Vigenere Progressive Key Cipher\nkey: {:?}",self.key_name)
    }
}