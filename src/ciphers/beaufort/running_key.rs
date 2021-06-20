use std::{fmt, fs::File, io::{Error, Read}};

fn beaufort_encrypt(n: usize, k: usize, l: usize) -> usize {
    (l + k - n) % l
}



pub struct RunningKey<'a> {
    key_file: &'a str,
    alphabet: &'a str,
    length: usize,
}


impl RunningKey<'_> {
    pub fn new<'a>(key_file: &'a str, alphabet: &'a str) -> RunningKey<'a> {
        RunningKey{ key_file, alphabet, length: alphabet.chars().count() }
    }

    // This is very inefficient 
    pub fn encrypt(&self, text: &str) -> Result<String,Error> {
        let mut key_text = String::new();
        let mut source = File::open(self.key_file)?;
        source.read_to_string(&mut key_text)?;
        let key: Vec<usize> = key_text.chars().map( |x| self.alphabet.chars().position(|c| c == x).unwrap() ).collect();
        let nums: Vec<usize> = text.chars().map( |x| self.alphabet.chars().position(|c| c == x).unwrap() ).collect();

        let mut out = String::new();

        for (n,k) in nums.iter().zip(key) {
            out.push(self.alphabet.chars().nth( beaufort_encrypt(*n,k,self.length) ).unwrap() )
        }
        Ok(out)
    }

    // The running key is an involution
    pub fn decrypt(&self, text: &str) -> Result<String,Error> {
        self.encrypt(text)
    }

}

impl fmt::Display for RunningKey<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Beaufort Running Key Cipher\nkey file: {:?}",self.key_file)
    }
}