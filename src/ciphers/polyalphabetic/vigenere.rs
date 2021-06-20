use std::fmt;

use crate::auxiliary::string_to_nums;


pub struct Vigenere<'a> {
    key_vals: Vec<usize>,
    key_name: &'a str,
    alphabet: &'a str,
    length: usize,
    mode: &'a str,
}

#[inline(always)]
pub fn vigenere_enc(c: usize, k: usize, length: usize) -> usize {
    (c+k)%length
}

#[inline(always)]
pub fn vigenere_dec(c: usize, k: usize, length: usize) -> usize {
    (length+c-k)%length
}


impl Vigenere<'_> {

    pub fn new<'a>(key: &'a str, mode: &'a str, alphabet: &'a str) -> Vigenere<'a> {
        let key_name = key;
        let key_vals: Vec<usize> = string_to_nums(key, alphabet);
        Vigenere{ key_vals, key_name, alphabet, length: alphabet.chars().count(), mode }
    }



    fn text_to_nums(&self, text: &str) -> Vec<usize> {
        text.chars().map( |x| self.alphabet.chars().position(|c| c == x).unwrap() ).collect_vec()
    }



    fn encrypt_norm(&self, text: &str) -> String {
        let mut out = String::with_capacity(text.chars().count());
        let nums = text_to_nums(text);
        let ckey = self.key_vals.iter().cycle();
        for (n,k) in nums.iter().zip(ckey) {
            out.push(self.alphabet.chars().nth( (n+k)%self.length ).unwrap() )
        }
        out
    }

    fn decrypt_norm(&self, text: &str) -> String {
        let mut out = String::with_capacity(text.chars().count());
        let nums = text_to_nums(text);
        let ckey = self.key_vals.iter().cycle();
        for (n,k) in nums.iter().zip(ckey) {
            out.push(self.alphabet.chars().nth( (n-k)%self.length ).unwrap() )
        }
        out
    }



    fn encrypt_auto(&self, text: &str) -> String {
        let mut out = String::with_capacity(text.chars().count());
        let nums = text_to_nums(text);
        
        let mut akey: VecDeque<usize> = self.key_vals.clone().into_iter().collect();
        for n in nums {
            akey.push_back(n);
            let k = akey.pop_front().unwrap();
            out.push(self.alphabet.chars().nth( (n+k)%self.length ).unwrap() )
        }
        out
    }

    fn decrypt_auto(&self, text: &str) -> String {
        let mut out = String::with_capacity(text.chars().count());
        let nums = text_to_nums(text);

        let mut akey: VecDeque<usize> = self.key_vals.clone().into_iter().collect();
        for n in nums {
            let k = akey.pop_front().unwrap();
            let v = (n + self.length - k ) % self.length;
            let c = self.alphabet.chars().nth( v ).unwrap();
            out.push(c);
            akey.push_back(v);
        }
        out
    }



    fn encrypt_prog(&self, text: &str) -> String {
        let mut out = String::with_capacity(text.chars().count());
        let nums = text_to_nums(text);

        let ckey = self.key_vals.iter().cycle();

        let mut shift = 0;
        let mut ctr = 0;
        let klen = self.key_vals.len();
        let alen = self.length;
        for (n, k) in nums.iter().zip(ckey) {
            out.push(self.alphabet.chars().nth( (n+k+shift)%alen ).unwrap() );
            ctr = (ctr + 1) % klen;
            if ctr == 0 {
                shift = (shift + self.increment) % alen
            }
        }
        out
    }

    fn decrypt_prog(&self, text: &str) -> String {
        let mut out = String::with_capacity(text.chars().count());
        let nums = text_to_nums(text);

        let ckey = self.key_vals.iter().cycle();

        let mut shift = 0;
        let mut ctr = 0;
        let klen = self.key_vals.len();
        let alen = self.length;
        for (n, k) in nums.iter().zip(ckey) {
            out.push(self.alphabet.chars().nth( (alen+n-(k+shift))%alen ).unwrap() );
            ctr = (ctr + 1) % klen;
            if ctr == 0 {
                shift = (shift + self.increment) % alen
            }
        }
        out
    }

}

impl crate::Cipher for Vigenere<'_> {

    fn encrypt(&self, text: &str) -> String {
        match self.mode {
            "normal" => self.encrypt_norm(text),
            "auotkey" => self.encrypt_auto(text),
            "progressive" => self.encrypt_prog(text),
        }
    }

    fn decrypt(&self, text: &str) -> String {
        match self.mode {
            "normal" => self.decrypt_norm(text),
            "auotkey" => self.decrypt_auto(text),
            "progressive" => self.decrypt_prog(text),
        }
    }

}

impl fmt::Display for Vigenere<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Vigenere Cipher\nkey: {}",self.key_name)
    }
}