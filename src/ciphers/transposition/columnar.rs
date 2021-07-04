use std::fmt;
use num::Integer;
use std::{fs::File, io::{Error, Read, Write}};

use crate::grid::Grid;
use crate::auxiliary::rank_str;

pub struct Columnar<'a> {
    key: Vec<usize>,
    key_name: &'a str,
}

impl Columnar<'_> {

    pub fn new<'a>(key: &'a str, alphabet: &'a str) -> Columnar<'a> {
        let key_name = key;
        let key = rank_str(key,alphabet);
        Columnar{ key, key_name }
    }

}

impl crate::Cipher for Columnar<'_> {

    fn encrypt(&self, text: &str) -> String {
        let tlen = text.chars().count();
        let n_rows = tlen.div_ceil(&self.key.len());
        let g = Grid::new(text, n_rows, self.key.len());

        let mut out = String::with_capacity(text.len());
        for k in self.key.iter() {
            let mut s: String = g.read_col_n(*k).iter().collect();
            s = s.replace('\0', "");
            out.push_str(&s);
        }
        out
    }

    // Decoding is very different
    fn decrypt(&self, text: &str) -> String {
        let tlen = text.chars().count();
        let filled = match tlen % self.key.len() {
            0 => self.key.len(),
            a => a
        };
        let n_rows = tlen.div_ceil(&self.key.len());

        let mut g = Grid::new_empty(n_rows, self.key.len());
        let mut symbols = text.chars();

        for k in self.key.iter() {
            let mut s = String::new();
            if *k < filled {
                for _ in 0..self.key.len() {
                    s.push(symbols.next().unwrap())
                }
            } else {
                for _ in 0..self.key.len()-1 {
                    s.push(symbols.next().unwrap())
                }
            }
            
            g.write_col_n(*k,&s);
        }

        let mut out = String::with_capacity(text.len());
        for i in 0..n_rows {
            let s: String = g.read_row_n(i).iter().collect();
            out.push_str(&s)
        }
        out = out.replace('\0', "");
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

impl fmt::Display for Columnar<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Columnar Cipher\nkey: {}",self.key_name)
    }
}