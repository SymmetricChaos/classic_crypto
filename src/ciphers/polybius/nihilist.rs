use std::fmt;

use crate::ciphers::polybius::polybius::Polybius;
use crate::ciphers::transposition::columnar::Columnar;


pub struct Nihilist {
    polybius: Polybius,
    vigenere: Vec<usize>,
}


impl Nihilist {
    pub fn new(alphabet: &str, key: Vec<usize>) -> Nihilist {
        let polybius = Polybius::new(alphabet,"12345");
        Nihilist{ polybius, key }
    }

/*     pub fn encode(&self, text: &str) -> String {

        
    } */

/*     pub fn decode(&self, text: &str) -> String {

    } */
}

impl fmt::Display for Nihilist {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Nihilist Composite Cipher\n{}\n{}",self.columnar,self.polybius)
    }
}
