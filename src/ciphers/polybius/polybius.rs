use std::fmt;
use std::collections::HashMap;
use std::{fs::File, io::{Error, Read, Write}};
use crate::alphabets::keyed_alphabet;
use itertools::Itertools;



struct CartesianPower {
    alphabet: Vec<char>,
    dimension: usize,
    counters: Vec<usize>,
    counter: usize,
}

impl Iterator for CartesianPower {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {

        if self.counter >= self.alphabet.len().pow(self.dimension as u32) {
            return None
        }

        self.counter += 1;

        let mut s = String::with_capacity(self.dimension);
        for c in self.counters.iter().rev() {
            s.push(self.alphabet[*c])
        }
        
        let mut carry = true;
        for x in self.counters.iter_mut() {
            if carry {
                *x += 1
            }
            if x == &self.alphabet.len() {
                carry = true;
                *x = *x % self.alphabet.len()
            } else {
                carry = false
            }
        }

        Some(s)
    }

}

impl CartesianPower {

    pub fn new(alpha: &str, dimension: usize) -> CartesianPower {
        let alphabet = alpha.chars().collect_vec();
        let mut counters = Vec::with_capacity(dimension);
        for _ in 0..dimension {
            counters.push(0)
        }
        CartesianPower{ alphabet, dimension, counters, counter: 0 }
    }

}



// Generalized Polybius hypercube, usually used with dimension 2 to produce a square or sometimes with dimension 3 to produce a cube
pub struct Polybius {
    map: HashMap<char,String>,
    map_inv: HashMap<String,char>,
    alphabet: String,
    dimension: usize,
}

impl Polybius {
    pub fn new(keyword: &str, alphabet: &str, labels: &str, dimension: usize) -> Polybius {

        if dimension < 2 {
            panic!("Dimensions must be greater than 1")
        }

        let alphabet = keyed_alphabet(keyword,alphabet);        

        let alen = alphabet.chars().count();
        let llen = labels.chars().count();
        if alen > llen*llen*dimension {
            panic!("an alphabet with {} characters does not fit in an {}-cube with edges of length {}.",alen,dimension,llen)
        }

        let symbols = alphabet.chars();
        let codes = CartesianPower::new(labels, dimension);

        let mut map = HashMap::with_capacity(alen);
        let mut map_inv = HashMap::with_capacity(alen);
        for (sy,code) in symbols.zip(codes) {
            map.insert(sy, code.clone());
            map_inv.insert(code, sy);
        }

        Polybius{ map, map_inv, alphabet, dimension }
    }

}

impl crate::Cipher for Polybius {

    fn encrypt(&self, text: &str) -> String {
        let tlen = text.chars().count();
        let mut out = String::with_capacity(tlen*self.dimension);

        for c in text.chars() {
            let s = &self.map[&c];
            out.push_str(s);
        }

        out
    }

    fn decrypt(&self, text: &str) -> String {
        let tlen = text.chars().count();
        let mut out = String::with_capacity(tlen/self.dimension);

        let raw_chunks = text.chars().chunks(self.dimension);
        let chunks = raw_chunks.into_iter().map(|x| x.collect::<String>()).into_iter();

        for c in chunks {
            out.push(self.map_inv[&c]);
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

impl fmt::Display for Polybius {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut square = format!("Generalized Polybius Square with Dimension {}\n",self.dimension);
        
        for a in self.alphabet.chars() {
            let s = format!("{}  {}\n",a,self.map[&a]);
            square.push_str(&s)
        }

        write!(f, "{}",square)
    }
}