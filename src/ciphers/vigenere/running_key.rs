use std::fmt;
use std::{ fs::File, io::{Error, Read}};





pub struct RunningKey<'a> {
    /// The Running Key version uses a key that is the same length as the entire plaintext, never repeating. This can be done using the ordinary Vigenere implementation but this makes it easier by reading the key information from a file.
    key_file: &'a str,
    alphabet: &'a str,
    length: usize,
}

impl RunningKey<'_> {
    pub fn new<'a>(key_file: &'a str, alphabet: &'a str) -> RunningKey<'a> {
        RunningKey{ key_file, alphabet, length: alphabet.chars().count() }
    }

    // This is potentially very inefficient there are apparently ways to read utf8 characters into a buffer one at a time
    pub fn encrypt(&self, text: &str) -> Result<String,Error> {
        let mut key_text = String::new();
        let mut source = File::open(self.key_file)?;
        source.read_to_string(&mut key_text)?;
        let key: Vec<usize> = key_text.chars().map( |x| self.alphabet.chars().position(|c| c == x).unwrap() ).collect();
        let nums: Vec<usize> = text.chars().map( |x| self.alphabet.chars().position(|c| c == x).unwrap() ).collect();

        let mut out = String::new();

        for (n,k) in nums.iter().zip(key) {
            out.push(self.alphabet.chars().nth( (n+k)%self.length ).unwrap() )
        }
        Ok(out)
    }

    pub fn decrypt(&self, text: &str) -> Result<String,Error> {
        let mut key_text = String::new();
        let mut source = File::open(self.key_file)?;
        source.read_to_string(&mut key_text)?;
        let key: Vec<usize> = key_text.chars().map( |x| self.alphabet.chars().position(|c| c == x).unwrap() ).collect();
        let nums: Vec<usize> = text.chars().map( |x| self.alphabet.chars().position(|c| c == x).unwrap() ).collect();

        let mut out = String::new();

        for (n,k) in nums.iter().zip(key) {
            out.push(self.alphabet.chars().nth( (self.length+n-k)%self.length ).unwrap() )
        }
        Ok(out)
    }

}

impl fmt::Display for RunningKey<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Vigenere Running Key Cipher\nkey file: {:?}",self.key_file)
    }
}




