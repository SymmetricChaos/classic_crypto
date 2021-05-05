use std::fmt;
use num::Integer;

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

impl crate::auxiliary::Cipher for Scytale {

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

}

impl fmt::Display for Scytale {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Scytale\nkey: {}",self.key)
    }
}

#[test]
fn scytale() {
    use crate::Cipher;
    let scytale = Scytale::new(3);
    println!("{}",scytale);
    let plaintext = "WEAREDISCOVEREDFLEEATONCE";
    let ciphertext = scytale.encrypt(plaintext);
    let cleartext = scytale.decrypt(&ciphertext);

    println!("{}",ciphertext);
    println!("{}",cleartext);

}