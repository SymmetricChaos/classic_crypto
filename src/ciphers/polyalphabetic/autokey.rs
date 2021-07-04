use std::{collections::VecDeque, fmt};
use std::{fs::File, io::{Error, Read, Write}};

use crate::{Cipher, PolyalphabeticCipher};

pub struct Autokey<'a> {
    cipher: &'a dyn PolyalphabeticCipher
}

impl Autokey<'_> {
    pub fn new(cipher: &dyn PolyalphabeticCipher) -> Autokey {
        Autokey{ cipher: cipher.clone() }
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

impl fmt::Display for Autokey<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Autokey {}",self.cipher)
    }
}