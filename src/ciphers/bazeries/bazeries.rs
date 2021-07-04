use std::fmt;
use std::{fs::File, io::{Error, Read, Write}};

pub struct Bazeries<'a> {
    wheels: Vec<&'a str>,
    length: usize,
    offset: usize,
}

impl Bazeries<'_> {
    // It would be more efficient if this were only using the Bazeries rows that are in the key
    pub fn new<'a>(offset: usize, wheels: Vec<&'a str>, alphabet: &'a str) -> Bazeries<'a> {
        let alen = alphabet.chars().count();
        for row in wheels.iter() {
            if row.chars().count() != alen {
                panic!("all of the wheels for the Bazeries cylinder must have the same length as the alphabet")
            }
            for c in row.chars() {
                if !alphabet.contains(c) {
                    panic!("all of the wheels for the Bazeries cylinder must be permutations of the alphabet")
                }
            }
        }
        let length = alphabet.chars().count();
        Bazeries{ wheels, length, offset }
    }

}

impl crate::Cipher for Bazeries<'_> {

    fn encrypt(&self, text: &str) -> String {
        let mut out = String::with_capacity(text.chars().count());
        let ckey = self.wheels.iter().cycle();
        for (k, c) in ckey.zip(text.chars()) {
            let n = (k.chars().position(|x| x == c).unwrap() + self.offset) % self.length;
            out.push(k.chars().nth(n).unwrap())
        }
        out
    }

    fn decrypt(&self, text: &str) -> String {
        let mut out = String::with_capacity(text.chars().count());
        let rev_offset = self.length - self.offset;
        let ckey = self.wheels.iter().cycle();
        for (k, c) in ckey.zip(text.chars()) {
            let n = (k.chars().position(|x| x == c).unwrap() + rev_offset) % self.length;
            out.push(k.chars().nth(n).unwrap())
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

impl fmt::Display for Bazeries<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Bazeries Cipher")
    }
}