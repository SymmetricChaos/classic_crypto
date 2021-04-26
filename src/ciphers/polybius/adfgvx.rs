use std::fmt;

use crate::ciphers::polybius::polybius::Polybius;
use crate::ciphers::transposition::columnar::Columnar;

//const ALPHANUM: &str = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";

pub struct ADFGVX {
    polybius: Polybius,
    columnar: Columnar,
}

// Automatically pads with the letter X during the columnar step meaning the message is padded with the last letter of the alphabet, this should be adjustable
// Need to enforce the specific alphabet of the ADFGVX cipher
impl ADFGVX {
    pub fn new(alphabet: &str, columnar_key: Vec<usize>) -> ADFGVX {
        let polybius = Polybius::new(alphabet,"ADFGVX");
        let columnar = Columnar::new(columnar_key);
        ADFGVX{ polybius, columnar }
    }

    pub fn encode(&self, text: &str) -> String {
        let mut intermediate = self.polybius.encode(text);
        intermediate = self.columnar.encode(&intermediate);
        self.polybius.decode(&intermediate)
    }

    pub fn decode(&self, text: &str) -> String {
        let mut intermediate = self.polybius.encode(text);
        intermediate = self.columnar.decode(&intermediate);
        self.polybius.decode(&intermediate)
    }
}

impl fmt::Display for ADFGVX {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ADFGVX Composite Cipher\n{}\n{}",self.columnar,self.polybius)
    }
}

#[test]
fn adfgvx() {

    let adfgvx = ADFGVX::new("0AB1CD2EF3GH4IJ5KL6MN7OP8QR9STUVWXYZ", vec![5,2,1,3,0,4]);

    println!("{}",adfgvx);
    let plaintext = "THEQUICKBROWNFOXJUMPSOVERTHELAZYDOGX";
    let ciphertext = adfgvx.encode(plaintext);
    let cleartext = adfgvx.decode(&ciphertext);

    println!("{}\n{}\n{}",plaintext,ciphertext,cleartext);
}