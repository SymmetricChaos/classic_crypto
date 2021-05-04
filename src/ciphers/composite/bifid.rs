use std::fmt;

use crate::ciphers::polybius::polybius::Polybius;

/// The Bifid Cipher combines a Polybius Square with a simple transposition
pub struct Bifid {
    polybius: Polybius,
    block_size: usize,
}


impl Bifid {
    pub fn new(keyword: &str, alphabet: &str, labels: &str, block_size: usize) -> Bifid {
        let polybius = Polybius::new(keyword, alphabet, labels);
        Bifid{ polybius, block_size }
    }

    pub fn encrypt(&self, text: &str) -> String {
        let len = text.chars().count();
        if len % self.block_size != 0 {
            panic!("text length must be a multiple of the block size, add {} characters of padding",self.block_size-(len%self.block_size))
        }
        let n_blocks = len/self.block_size;
        let mut out = String::with_capacity(len*2);
        for block in 0..n_blocks {
            let clip = &text[block*self.block_size..block*self.block_size+self.block_size];
            let poly = self.polybius.encrypt(clip);
            let mut left = String::with_capacity(len);
            let mut right = String::with_capacity(len);
            for (pos, s) in poly.chars().enumerate() {
                if (pos % 2) == 0 {
                    left.push(s)
                } else {
                    right.push(s)
                }
            }
            left.push_str(&right);
            out.push_str(&self.polybius.decrypt(&left))
        }

        out

    }

/*     pub fn decrypt(&self, text: &str) -> String {
        let len = text.chars().count();
        let n_blocks = len/self.block_size;
        let mut out = String::with_capacity(len*2);
        for block in 0..n_blocks {
            let clip = &text[block*self.block_size..block*self.block_size+self.block_size];
            let poly = self.polybius.encrypt(clip);

            // Sort the letters back to their proper positions

            out.push_str(&self.polybius.decrypt(&something))
        }

        out
    } */
}

impl fmt::Display for Bifid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Bifid Composite Cipher\nBlock Size:\n{}\nPolybius Square:\n{}",self.block_size,self.polybius)
    }
}

#[test]
fn bifid() {
    use crate::alphabets::LATIN25_J;

    let bifid = Bifid::new("ZEBRAS", LATIN25_J, "12345", 7);
    println!("{}",bifid);

    let plaintext = "THEQUICKBROWNFOXIUMPSOVERTHELAZYDOG";
    println!("{}",plaintext);

    let ciphertext = bifid.encrypt(plaintext);
    println!("{}",ciphertext);

    //let decrypted = bifid.decrypt(&ciphertext);
    //println!("{}",decrypted);
}