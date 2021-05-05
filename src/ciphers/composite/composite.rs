use std::fmt;

use crate::Cipher;

/// A composite cipher applies several ciphers in succession. 
pub struct CompositeCipher {
    ciphers: Vec<Box<dyn Cipher>>,
}


impl CompositeCipher {
    pub fn new(ciphers: Vec<Box<dyn Cipher>>) -> CompositeCipher {
        CompositeCipher{ ciphers }
    }

    pub fn encrypt(&self, text: &str) -> String {
        let mut out = text.to_string();
        for cipher in self.ciphers.iter() {
            out = cipher.encrypt(&out)
        }
        out
    }

    pub fn decrypt(&self, text: &str) -> String {
        let mut out = text.to_string();
        for cipher in self.ciphers.iter().rev() {
            out = cipher.decrypt(&out)
        }
        out
    }
}


#[test]
fn composite() {
    use crate::alphabets::LATIN26;
    use crate::ciphers::{Caesar,transposition::Scytale};

    let c1 = Caesar::new(7, LATIN26);
    let c2 = Scytale::new(3);

    let composite = CompositeCipher::new(vec![Box::new(c1), Box::new(c2)]);
    let plaintext = "THEQUICKBROWNFOXJUMPSOVERTHELAZYDOGQ";
    let ciphertext = composite.encrypt(plaintext);
    let decrypt = composite.decrypt(&ciphertext);

    println!("{}",ciphertext);
    println!("{}",decrypt);

/*     assert_eq!(ciphertext,"");
    assert_eq!(cleartext, ""); */


}