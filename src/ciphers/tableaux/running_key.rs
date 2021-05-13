use std::fmt;
use std::{ fs::File, io::{Error, Read}};





pub struct RunningKey<'a> {
    tableaux: Vec<&'a str>,
    key_file: &'a str,
    alphabet: &'a str,
}

impl RunningKey<'_> {
    pub fn new<'a>(key_file: &'a str, tableaux: Vec<&'a str>, alphabet: &'a str) -> RunningKey<'a> {
        RunningKey{ tableaux, key_file, alphabet }
    }

    pub fn tableaux(&self) -> String {
        let mut out = "  ".to_string();
        for c in self.alphabet.chars() {
            out.push(c);
            out.push(' ')
        }
        out.push_str("\n");
        for (row, c) in self.tableaux.iter().zip(self.alphabet.chars()) {
            out.push(c);
            out.push(' ');
            for r in row.chars() {
                out.push(r);
                out.push(' ')
            }
            out.push_str("\n")
        }
        out
    }

    fn encrypt_pair(&self, c: char, k: usize) -> char {
        let n = self.tableaux[k].chars().position(|x| x == c).unwrap();
        self.alphabet.chars().nth(n).unwrap()
    }

    fn decrypt_pair(&self, c: char, k: usize) -> char {
        let n = self.alphabet.chars().position(|x| x == c).unwrap();
        self.tableaux[k].chars().nth(n).unwrap()
    }

    // This is potentially very inefficient there are apparently ways to read utf8 characters into a buffer one at a time
    pub fn encrypt(&self, text: &str) -> Result<String,Error> {
        let mut key_text = String::new();
        let mut source = File::open(self.key_file)?;
        source.read_to_string(&mut key_text)?;
        let key: Vec<usize> = key_text.chars().map( |x| self.alphabet.chars().position(|c| c == x).unwrap() ).collect();

        let mut out = String::new();

        for (c,k) in text.chars().zip(key) {
            out.push( self.encrypt_pair(c, k))
        }
        Ok(out)
    }

    pub fn decrypt(&self, text: &str) -> Result<String,Error> {
        let mut key_text = String::new();
        let mut source = File::open(self.key_file)?;
        source.read_to_string(&mut key_text)?;
        let key: Vec<usize> = key_text.chars().map( |x| self.alphabet.chars().position(|c| c == x).unwrap() ).collect();

        let mut out = String::new();

        for (c,k) in text.chars().zip(key) {
            out.push( self.decrypt_pair(c, k))
        }
        Ok(out)
    }

}

impl fmt::Display for RunningKey<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Tableaux Running Key Cipher\nkey file: {:?}",self.key_file)
    }
}




