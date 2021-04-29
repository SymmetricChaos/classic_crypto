use std::fmt;

use crate::ciphers::polybius::polybius::Polybius;
use crate::ciphers::transposition::columnar::Columnar;

/// The ADFGX Cipher combines a 5x5 Polybius Square with Columnar Transposition to achieve surprisingly high security with a very simple cipher. It was later replaced by the ADFGVX Cipher which used a 6x6 Polybius Square.
pub struct ADFGX {
    polybius: Polybius,
    columnar: Columnar,
}

// Automatically pads with the letter X during the columnar step meaning the message is padded with the last letter of the alphabet, this should be adjustable
impl ADFGX {
    pub fn new(keyword: &str, columnar_key: Vec<usize>) -> ADFGX {
        let polybius = Polybius::new(keyword, "ABCDEFGHIKLMNOPQRSTUVWXYZ", "ADFGX");
        let columnar = Columnar::new(columnar_key);
        ADFGX{ polybius, columnar }
    }

    pub fn encode(&self, text: &str) -> String {
        let text = text.replace('J',"I");
        let mut intermediate = self.polybius.encode(&text);
        intermediate = self.columnar.encode(&intermediate);
        self.polybius.decode(&intermediate)
    }

    pub fn decode(&self, text: &str) -> String {
        let mut intermediate = self.polybius.encode(text);
        intermediate = self.columnar.decode(&intermediate);
        self.polybius.decode(&intermediate)
    }
}

impl fmt::Display for ADFGX {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ADFGX Composite Cipher\n{}\n{}",self.columnar,self.polybius)
    }
}

#[test]
fn adfgx() {

    let adfgx = ADFGX::new("ABCDEFGHIKLMNOPQRSTUVWXYZ", vec![5,2,1,3,0,4]);

    println!("{}",adfgx);
    let plaintext = "THEQUICKBROWNFOXJUMPSOVERTHELAZYDOGX";
    let ciphertext = adfgx.encode(plaintext);
    let cleartext = adfgx.decode(&ciphertext);

    println!("{}\n{}\n{}",plaintext,ciphertext,cleartext);
}