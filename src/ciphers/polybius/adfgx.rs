use std::fmt;

use crate::ciphers::polybius::polybius::Polybius;
use crate::ciphers::transposition::columnar::Columnar;

//const ALPHANUM: &str = "ABCDEFGHIKLMNOPQRSTUVWXYZ";

pub struct ADFGX {
    polybius: Polybius,
    columnar: Columnar,
}

// Automatically pads with the letter X during the columnar step meaning the message is padded with the last letter of the alphabet, this should be adjustable
// Need to enforce the specific alphabet of the ADFGX cipher
impl ADFGX {
    pub fn new(alphabet: &str, columnar_key: Vec<usize>) -> ADFGX {
        let polybius = Polybius::new(alphabet,"ADFGX");
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