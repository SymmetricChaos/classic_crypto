use std::collections::VecDeque;

use crate::{Cipher, PolyalphabeticCipher};

pub struct Autokey<'a> {
    cipher: Box<&'a dyn PolyalphabeticCipher>
}

impl Autokey<'_> {
    pub fn new(cipher: &dyn PolyalphabeticCipher) -> Autokey {
        Autokey{ cipher: Box::new(cipher.clone()) }
    }
}

impl Cipher for Autokey<'_> {
    fn encrypt(&self, text: &str) -> String {
        let mut out = Vec::with_capacity(text.chars().count());

        let mut akey: VecDeque<usize> = self.cipher.key_vals().into_iter().collect();
        let nums = self.cipher.text_to_nums(text);

        for n in nums {
            akey.push_back(n);
            let k = akey.pop_front().unwrap();
            out.push( self.cipher.encrypt_char(n,k) )
        }

        self.cipher.nums_to_text(out)
    }

    fn decrypt(&self, text: &str) -> String {
        let mut out = Vec::with_capacity(text.chars().count());

        let mut akey: VecDeque<usize> = self.cipher.key_vals().into_iter().collect();
        let nums = self.cipher.text_to_nums(text);

        for n in nums {
            let k = akey.pop_front().unwrap();
            let v = self.cipher.decrypt_char(n,k);
            out.push(v);
            akey.push_back(v);
        }
        
        self.cipher.nums_to_text(out)
    }
}