use std::fmt;

use crate::ciphers::polybius::polybius::Polybius;
use crate::ciphers::transposition::columnar::Columnar;
use crate::alphabets::{LATIN25_J,LATIN26};

/// The ADFGX Cipher combines a 5x5 Polybius Square (replacing J with I) and Columnar Transposition to achieve surprisingly high security with a very simple cipher. It was later replaced by the ADFGVX Cipher which used a 6x6 Polybius Square.
pub struct ADFGX {
    polybius: Polybius,
    columnar: Columnar,
}


impl ADFGX {
    pub fn new(keyword: &str, columnar_keyword: &str) -> ADFGX {
        let polybius = Polybius::new(keyword, LATIN25_J, "ADFGX");
        let columnar = Columnar::new_keyword(columnar_keyword, LATIN26);
        ADFGX{ polybius, columnar }
    }

    pub fn encrypt(&self, text: &str) -> String {
        let text = text.replace('J',"I");
        let mut intermediate = self.polybius.encrypt(&text);
        intermediate = self.columnar.encrypt(&intermediate);
        self.polybius.decrypt(&intermediate)
    }

    pub fn decrypt(&self, text: &str) -> String {
        let mut intermediate = self.polybius.encrypt(text);
        intermediate = self.columnar.decrypt(&intermediate);
        self.polybius.decrypt(&intermediate)
    }
}

impl fmt::Display for ADFGX {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ADFGX Composite Cipher\n{}\n{}",self.columnar,self.polybius)
    }
}

#[test]
fn adfgx() {

    let adfgx = ADFGX::new("ELPEHANTS", "ZEBRAS");

    println!("{}",adfgx);
    let plaintext = "THEQUICKBROWNFOXJUMPSOVERTHELAZYDOGX";
    let ciphertext = adfgx.encrypt(plaintext);
    let cleartext = adfgx.decrypt(&ciphertext);

    println!("{}\n{}\n{}",plaintext,ciphertext,cleartext);
}