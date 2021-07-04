use std::fmt;
use num::Integer;
use std::{fs::File, io::{Error, Read, Write}};

pub fn pad_with_char(text: &str, length: usize, symbol: char) -> String {
    let mut text = text.to_string();
    while text.chars().count() % length != 0 {
        text.push(symbol)
    }
    text
}


pub struct Scytale {
    key: usize,
}

impl Scytale {
    pub fn new(key: usize) -> Scytale {
        Scytale{ key }
    }


}

impl crate::Cipher for Scytale {

    fn encrypt(&self, text: &str) -> String {
        let n_cols = text.len().div_ceil(&self.key);
        let mut symbols = text.chars();
        let mut rows = Vec::new();

        for _ in 0..self.key {
            let mut v = Vec::new();
            for _ in 0..n_cols {
                v.push(symbols.next().unwrap_or('X'))
            }
            rows.push(v)
        }

        let mut out = "".to_string();

        for x in 0..n_cols {
            for y in 0..self.key {
                out.push(rows[y][x])
            }
        }

        out
    }


    fn decrypt(&self, text: &str) -> String {
        let n_cols = text.len().div_ceil(&self.key);
        let mut symbols = text.chars();
        let mut rows = Vec::new();

        for _ in 0..n_cols {
            let mut v = Vec::new();
            for _ in 0..self.key {
                v.push(symbols.next().unwrap_or('X'))
            }
            rows.push(v)
        }

        let mut out = "".to_string();

        for x in 0..self.key {
            for y in 0..n_cols {
                out.push(rows[y][x])
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

impl fmt::Display for Scytale {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Scytale\nkey: {}",self.key)
    }
}