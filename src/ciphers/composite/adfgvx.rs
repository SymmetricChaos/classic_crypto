use std::fmt;

use crate::ciphers::polybius::polybius::Polybius;
use crate::ciphers::transposition::columnar::Columnar;
use crate::alphabets::{LATIN36,LATIN26};

/// The ADFGVX Cipher combines a 6x6 Polybius Square with Columnar Transposition to achieve surprisingly high security with a very simple cipher. It is an updated version of the ADFGX Cipher which used a 5x5 Polybius Square.
pub struct ADFGVX {
    polybius: Polybius,
    columnar: Columnar,
}


impl ADFGVX {
    pub fn new(keyword: &str, columnar_keyword: &str) -> ADFGVX {
        let polybius = Polybius::new(keyword, LATIN36, "ADFGVX");
        let columnar = Columnar::new_keyword(columnar_keyword, LATIN26);
        ADFGVX{ polybius, columnar }
    }

    pub fn encrypt(&self, text: &str) -> String {
        let mut intermediate = self.polybius.encrypt(text);
        intermediate = self.columnar.encrypt(&intermediate);
        self.polybius.decrypt(&intermediate)
    }

    pub fn decrypt(&self, text: &str) -> String {
        let mut intermediate = self.polybius.encrypt(text);
        intermediate = self.columnar.decrypt(&intermediate);
        self.polybius.decrypt(&intermediate)
    }
}

impl fmt::Display for ADFGVX {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ADFGVX Composite Cipher\n{}\n{}",self.columnar,self.polybius)
    }
}

#[test]
fn adfgvx() {

    let adfgvx = ADFGVX::new("17ZEBRAS42", "ELEPHANTS");

    println!("{}",adfgvx);
    let plaintext = "THEQUICKBROWNFOXJUMPSOVERTHELAZYDOGX";
    let ciphertext = adfgvx.encrypt(plaintext);
    let cleartext = adfgvx.decrypt(&ciphertext);

    println!("{}\n{}\n{}",plaintext,ciphertext,cleartext);
}