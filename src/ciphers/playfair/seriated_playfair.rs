use std::fmt;
use std::{fs::File, io::{Error, Read, Write}};
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

    fn encrypt_chars(&self, a: char, b: char) -> (char,char) {
        if a == b {
            panic!("encountered invalid letter pair {}{}",a,b)
        }
        let a_pair = self.symbol_to_pair(a);
        let b_pair = self.symbol_to_pair(b);

        let s = self.size+1;

        if a_pair.0 == b_pair.0 {
            let x = a_pair.0;
            
            let out_a = self.pair_to_symbol((x, (a_pair.1+s)%self.size ));
            let out_b = self.pair_to_symbol((x, (b_pair.1+s)%self.size ));

            return (out_a,out_b)

        } else if a_pair.1 == b_pair.1 {
            let y = a_pair.1;
            
            let out_a = self.pair_to_symbol(( (a_pair.0+s)%self.size , y ));
            let out_b = self.pair_to_symbol(( (b_pair.0+s)%self.size , y ));

            return (out_a,out_b)

        } else {
            let out_a = self.pair_to_symbol((a_pair.0, b_pair.1));
            let out_b =  self.pair_to_symbol((b_pair.0, a_pair.1));

            return (out_a,out_b)
        }
    }

    fn decrypt_chars(&self, a: char, b: char) -> (char,char) {
        let a_pair = self.symbol_to_pair(a);
        let b_pair = self.symbol_to_pair(b);
        
        let s = self.size-1;

        if a_pair.0 == b_pair.0 {
            let x = a_pair.0;
            
            let out_a = self.pair_to_symbol((x, (a_pair.1+s)%self.size ));
            let out_b = self.pair_to_symbol((x, (b_pair.1+s)%self.size ));

            return (out_a,out_b)

        } else if a_pair.1 == b_pair.1 {
            let y = a_pair.1;
            
            let out_a = self.pair_to_symbol(( (a_pair.0+s)%self.size , y ));
            let out_b = self.pair_to_symbol(( (b_pair.0+s)%self.size , y ));

            return (out_a,out_b)

        } else {
            let out_a = self.pair_to_symbol((a_pair.0, b_pair.1));
            let out_b =  self.pair_to_symbol((b_pair.0, a_pair.1));

            return (out_a,out_b)
        }
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

        let mut out = String::with_capacity(tlen);

        for pair in 0..groups.len()/2 {
            println!("{:?}\n{:?}\n",groups[2*pair],groups[2*pair+1]);
            let mut row_a = String::with_capacity(self.period);
            let mut row_b = String::with_capacity(self.period);
            for pos in 0..self.period {
                let a = groups[2*pair][pos];
                let b = groups[2*pair+1][pos];
                let (out_a, out_b) = self.encrypt_chars(a, b);
                row_a.push(out_a);
                row_b.push(out_b);
            }
            out.push_str(&row_a);
            out.push_str(&row_b);
        }

        out

    }

    fn decrypt(&self, text: &str) -> String {
        let tlen = text.chars().count();
        if tlen % (self.period*2) != 0 {
            panic!("number of characters in the text must be a multiple of {}",self.period*2)
        }

        let chunks = text.chars().chunks(self.period);
        let groups = chunks.into_iter().map(|x| x.collect_vec()).collect_vec();

        let mut out = String::with_capacity(tlen);

        for pair in 0..groups.len()/2 {
            println!("{:?}\n{:?}\n",groups[2*pair],groups[2*pair+1]);
            let mut row_a = String::with_capacity(self.period);
            let mut row_b = String::with_capacity(self.period);
            for pos in 0..self.period {
                let a = groups[2*pair][pos];
                let b = groups[2*pair+1][pos];
                let (out_a, out_b) = self.decrypt_chars(a, b);
                row_a.push(out_a);
                row_b.push(out_b);
            }
            out.push_str(&row_a);
            out.push_str(&row_b);
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

impl fmt::Display for SeriatedPlayfair {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", format!("Seriated Playfair Cipher\nPeriod: {}\nSquare: {}\n",self.period,self.alphabet))
    }
}