use std::fmt;
use std::iter::Iterator;
use std::{fs::File, io::{Error, Read, Write}};

use crate::alphabets::keyed_alphabet;

/// The Playfair Cipher, developed by Charles Wheatstone and promoted at his request by Lord Playfair, is a substitution ciphers that operates on digraphs.
pub struct Playfair {
    alphabet: String,
    size: usize,
}

// Don't need rows, we can extract the indices by finding the position and some aritmetic
// rectangle: same row, opposite corner
// column: down one step
// row: right one step
impl Playfair {
    pub fn new(key: &str, alphabet: &str, size: usize) -> Playfair {
        let alen = alphabet.chars().count();
        
        if alen != size*size {
            panic!("an alphabet with {} characters does not exactly fit in a {}x{} square.",alen,size,size)
        }
        
        Playfair{ alphabet: keyed_alphabet(key, alphabet), size }
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

impl crate::Cipher for Playfair {
    
    fn encrypt(&self, text: &str) -> String {
        if text.chars().count() % 2 != 0 {
            panic!("The Playfair Cipher requires an even number of symbols in the text. Please adjust the input.")
        }
        let mut symbols = text.chars().peekable();
        let mut out = String::with_capacity(text.chars().count());
        loop {
            if symbols.peek().is_none() {
                break
            } else {
                let a = symbols.next().unwrap();
                let b = symbols.next().unwrap();
                let a_pair = self.symbol_to_pair(a);
                let b_pair = self.symbol_to_pair(b);

                let s = self.size+1;

                if a_pair.0 == b_pair.0 {
                    let x = a_pair.0;
                    
                    out.push(self.pair_to_symbol((x, (a_pair.1+s)%self.size )));
                    out.push(self.pair_to_symbol((x, (b_pair.1+s)%self.size )));

                } else if a_pair.1 == b_pair.1 {
                    let y = a_pair.1;
                    
                    out.push(self.pair_to_symbol(( (a_pair.0+s)%self.size , y )));
                    out.push(self.pair_to_symbol(( (b_pair.0+s)%self.size , y )));

                } else {
                    out.push(self.pair_to_symbol((a_pair.0, b_pair.1)));
                    out.push(self.pair_to_symbol((b_pair.0, a_pair.1)));
                }
            }
        }
        out
    }

    fn decrypt(&self, text: &str) -> String {
        if text.chars().count() % 2 != 0 {
            panic!("The Playfair Cipher requires an even number of symbols in the text. Please adjust the input.")
        }
        let mut symbols = text.chars().peekable();
        let mut out = String::with_capacity(text.chars().count());
        loop {
            if symbols.peek().is_none() {
                break
            } else {
                let a = symbols.next().unwrap();
                let b = symbols.next().unwrap();
                let a_pair = self.symbol_to_pair(a);
                let b_pair = self.symbol_to_pair(b);
                
                let s = self.size-1;

                if a_pair.0 == b_pair.0 {
                    let x = a_pair.0;
                    
                    out.push(self.pair_to_symbol(( x , (a_pair.1+s)%self.size ) ));
                    out.push(self.pair_to_symbol(( x , (b_pair.1+s)%self.size ) ));

                } else if a_pair.1 == b_pair.1 {
                    let y = a_pair.1;
                    
                    out.push(self.pair_to_symbol(( (a_pair.0+s)%self.size , y ) ));
                    out.push(self.pair_to_symbol(( (b_pair.0+s)%self.size , y ) ));

                } else {
                    out.push(self.pair_to_symbol((a_pair.0, b_pair.1)));
                    out.push(self.pair_to_symbol((b_pair.0, a_pair.1)));
                }
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

impl fmt::Display for Playfair {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut square = "Playfair Cipher".to_string();
        for (n, c) in self.alphabet.chars().enumerate() {
            if n % self.size == 0 {
                square.push_str("\n")
            }
            square.push_str(&format!("{} ",c))
        };
        write!(f, "{}", square)
    }
}
