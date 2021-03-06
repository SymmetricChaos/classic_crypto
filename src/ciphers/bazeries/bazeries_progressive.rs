use std::fmt;
use std::{fs::File, io::{Error, Read, Write}};

pub struct BazeriesProgressive<'a> {
    wheels: Vec<&'a str>,
    increment: usize,
    length: usize,
    offset: usize,
}

impl BazeriesProgressive<'_> {
    // It would be more efficient if this were only using the BazeriesProgressive rows that are in the key
    pub fn new<'a>(offset: usize, increment: usize, wheels: Vec<&'a str>, alphabet: &'a str) -> BazeriesProgressive<'a> {
        let alen = alphabet.chars().count();
        for row in wheels.iter() {
            if row.chars().count() != alen {
                panic!("all of the wheels for the BazeriesProgressive cylinder must have the same length as the alphabet")
            }
            for c in row.chars() {
                if !alphabet.contains(c) {
                    panic!("all of the wheels for the BazeriesProgressive cylinder must be permutations of the alphabet")
                }
            }
        }
        let length = alphabet.chars().count();
        BazeriesProgressive{ wheels, increment, length, offset }
    }

}

impl crate::Cipher for BazeriesProgressive<'_> {

    fn encrypt(&self, text: &str) -> String {
        let mut out = String::with_capacity(text.chars().count());
        let ckey = self.wheels.iter().cycle();
        let mut shift = 0;
        let mut ctr = 0;
        let klen = self.wheels.len();
        for (k, c) in ckey.zip(text.chars()) {
            let n = (k.chars().position(|x| x == c).unwrap() + self.offset + shift) % self.length;
            out.push(k.chars().nth(n).unwrap());
            ctr = (ctr+1) % klen;
            if ctr == 0 {
                shift += self.increment
            }
        }
        out
    }

    fn decrypt(&self, text: &str) -> String {
        let mut out = String::with_capacity(text.chars().count());
        let ckey = self.wheels.iter().cycle();
        let mut shift = 0;
        let mut ctr = 0;
        let klen = self.wheels.len();
        for (k, c) in ckey.zip(text.chars()) {
            let n = (k.chars().position(|x| x == c).unwrap() + self.length - self.offset - shift) % self.length;
            out.push(k.chars().nth(n).unwrap());
            ctr = (ctr+1) % klen;
            if ctr == 0 {
                shift += self.increment
            }
        }
        out
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

impl fmt::Display for BazeriesProgressive<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Bazeries Progressive Cipher")
    }
}