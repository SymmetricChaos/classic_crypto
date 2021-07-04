use std::fmt;
use std::{fs::File, io::{Error, Read, Write}};

use crate::{Cipher, PolyalphabeticCipher};

pub struct ProgressiveKey<'a> {
    cipher: &'a dyn PolyalphabeticCipher,
    step: usize,
}

impl ProgressiveKey<'_> {
    pub fn new(cipher: &dyn PolyalphabeticCipher, step: usize) -> ProgressiveKey {
        ProgressiveKey{ cipher: cipher.clone(), step }
    }
}

impl Cipher for ProgressiveKey<'_> {
    fn encrypt(&self, text: &str) -> String {
        let nums = self.cipher.text_to_nums(text);
        let mut out = Vec::with_capacity(nums.len());

        let kvals = self.cipher.key_vals().clone();
        let ckey = kvals.iter().cycle();
        
        let mut ctr = 0;
        let mut shift = 0;
        
        let klen = self.cipher.key_vals().len();
        let alen = self.cipher.alphabet_len();
        
        for (n, k) in nums.iter().zip(ckey) {
            out.push( self.cipher.encrypt_char(*n,(k+shift)%alen) );
            ctr = (ctr + 1) % klen;
            if ctr == 0 {
                shift = (shift + self.step) % alen
            }
        }

        self.cipher.nums_to_text(out)
    }

    fn decrypt(&self, text: &str) -> String {
        let nums = self.cipher.text_to_nums(text);
        let mut out = Vec::with_capacity(nums.len());

        let kvals = self.cipher.key_vals().clone();
        let ckey = kvals.iter().cycle();
        
        let mut ctr = 0;
        let mut shift = 0;
        
        let klen = self.cipher.key_vals().len();
        let alen = self.cipher.alphabet_len();
        
        for (n, k) in nums.iter().zip(ckey) {
            out.push( self.cipher.decrypt_char(*n,(k+shift)%alen) );
            ctr = (ctr + 1) % klen;
            if ctr == 0 {
                shift = (shift + self.step) % alen
            }
        }

        self.cipher.nums_to_text(out)
    }

    fn encrypt_file(&self, source: &str, target: &str) -> Result<(),Error> {

        let mut target_file = File::create(target.to_string())?;
    
        let mut source_file = File::open(source)?;
        let mut source_text = String::new();
        source_file.read_to_string(&mut source_text)?;
    
        let ciphertext = self.encrypt(&source_text);
    
        target_file.write(ciphertext.as_bytes())?;

        Ok(())
    }

    fn decrypt_file(&self, source: &str, target: &str) -> Result<(),Error> {

        let mut target_file = File::create(target.to_string())?;
    
        let mut source_file = File::open(source)?;
        let mut source_text = String::new();
        source_file.read_to_string(&mut source_text)?;
    
        let ciphertext = self.decrypt(&source_text);
    
        target_file.write(ciphertext.as_bytes())?;

        Ok(())
    }
}

impl fmt::Display for ProgressiveKey<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Progressive Key {}",self.cipher)
    }
}