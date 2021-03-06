use std::fmt;
use std::{fs::File, io::{Error, Read, Write}};

/// The DecoderRing represents ciphers like those used on Little Orphan Annie and Captain Midnight. Letters are replaced with a number. Captain midnight indicated the key with an alphanumeric pair like A3 to indicate that the ring should be turned so at A lines up with 3. 
pub struct DecoderRing<'a> {
    index: usize,
    alphabet: &'a str,
    length: usize,
}

impl DecoderRing<'_> {
    pub fn new(index: usize, alphabet: &str) -> DecoderRing {
        DecoderRing{ index, alphabet: alphabet, length: alphabet.chars().count() }
    }

    pub fn annie(index: usize) -> DecoderRing<'static> {
        DecoderRing{ index, alphabet: "_ASLWIMVHFKXDPOEJBTNQZGUYRC", length: 27 }
    }

    pub fn midnight(index: usize) -> DecoderRing<'static> {
        DecoderRing{ index, alphabet: "_AEXDTZKNYCJWSGUMBOQHRIVFPL", length: 27 }
    }

}

impl crate::Cipher for DecoderRing<'_> {

    fn encrypt(&self, text: &str) -> String {
        let symbols = text.chars();
        let mut out = Vec::new();
        for s in symbols {
            let n = (self.alphabet.chars().position(|x| x == s).unwrap() + self.index) % self.length;
            out.push( format!("{}",n) )
        }
        out.join(" ")
    }

    fn decrypt(&self, text: &str) -> String {
        let nums = text.split(' ').map(|x| (x.parse::<usize>().unwrap() + self.length - self.index) % self.length );
        let mut out = String::new();
        for n in nums {
            out.push(self.alphabet.chars().nth(n).unwrap() )
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

impl fmt::Display for DecoderRing<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Decoder Ring Cipher\nkey: {}\nAlphabet: {}",self.index,self.alphabet)
    }
}