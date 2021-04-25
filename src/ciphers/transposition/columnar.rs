use std::fmt;
use std::collections::HashMap;
use num::Integer;

use crate::errors::CipherError;

pub fn pad_with_char(text: &str, length: usize, symbol: char) -> String {
    let mut text = text.to_string();
    while text.chars().count() % length != 0 {
        text.push(symbol)
    }
    text
}


// Need a less memory intensive method
pub struct Columnar {
    key: Vec<usize>,
}

impl Columnar {
    pub fn new(key: Vec<usize>) -> Columnar {
        Columnar{ key }
    }

    pub fn encode(&self, text: &str) -> Result<String,CipherError> {
        let mut columns = Vec::new();
        for _ in 0..self.key.len() {
            columns.push(Vec::<char>::new());
        }
        let mut symbols = text.chars();
        let n_rows = text.len().div_ceil(&self.key.len());
        for _row in 0..n_rows {
            for col in columns.iter_mut() {
                col.push(symbols.next().unwrap_or('X'))
            }
        }
        println!("{:?}",columns);

        let out = "".to_string();
        Ok(out)
    }

/*     pub fn decode(&self, text: &str) -> Result<String,CipherError> {

    } */
}

impl fmt::Display for Columnar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}",self.key)
    }
}

#[test]
fn columnar() {

    let col = Columnar::new(vec![6,3,2,4,1,5]);
    println!("Columnar Cipher: {}",col);
    let plaintext = "WEAREDISCOVEREDFLEEATONCE";
    let ciphertext = col.encode(plaintext).unwrap();

}