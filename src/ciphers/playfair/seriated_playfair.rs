use std::{collections::HashMap, fmt};

use itertools::Itertools;

use crate::alphabets::keyed_alphabet;

// http://www.cryptoprograms.com/othercreate/seriatedplayfair

pub struct SeriatedPlayfair {
    alphabet: String,
    size: usize,
    period: usize,
}


impl SeriatedPlayfair {
    pub fn new(key: &str, alphabet: &str, size: usize, period: usize) -> SeriatedPlayfair {
        let alen = alphabet.chars().count();
        
        if alen != size*size {
            panic!("an alphabet with {} characters does not exactly fit in a {}x{} square.",alen,size,size)
        }
        
        SeriatedPlayfair{ alphabet: keyed_alphabet(key, alphabet), size, period }
    }

    
    fn symbol_to_pair(&self, symbol: char) -> (usize,usize) {
        let num = self.alphabet.chars().position(|x| x == symbol).unwrap();
        (num / self.size, num % self.size)
    }

    fn pair_to_symbol(&self, pair: (usize,usize)) -> char {
        let num = pair.0*self.size + pair.1;
        self.alphabet.chars().nth(num).unwrap()
    }

}

impl crate::Cipher for SeriatedPlayfair {
    
    fn encrypt(&self, text: &str) -> String {
        let tlen = text.chars().count();
        if tlen % (self.period*2) != 0 {
            panic!("number of characters in the text must be a multiple of {}",self.period*2)
        }

        let chunks = text.chars().chunks(self.period);
        let groups = chunks.into_iter().map(|x| x.collect_vec()).collect_vec();

        for g in groups {
            println!("{:?}",g);
        }

        

        let mut out = String::with_capacity(tlen);

        out

    }

    fn decrypt(&self, text: &str) -> String {
        let tlen = text.chars().count();
        if tlen % (self.period*2) != 0 {
            panic!("number of characters in the text must be a multiple of {}",self.period*2)
        }

        let mut out = String::with_capacity(tlen);

        out
    }

}

impl fmt::Display for SeriatedPlayfair {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", format!("Seriated Playfair Cipher\nPeriod: {}\nSquare: {}\n",self.period,self.alphabet))
    }
}