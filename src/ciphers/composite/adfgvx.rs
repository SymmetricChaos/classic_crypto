use std::fmt;

use crate::ciphers::polybius::Polybius;
use crate::ciphers::transposition::columnar::Columnar;
use crate::alphabets::LATIN36;

/// The ADFGVX Cipher combines a 6x6 Polybius Square with Columnar Transposition to achieve surprisingly high security with a very simple cipher. It is an updated version of the ADFGX Cipher which used a 5x5 Polybius Square.
pub struct ADFGVX<'a> {
    polybius: Polybius,
    columnar: Columnar<'a>,
}


impl ADFGVX<'_> {
    pub fn new<'a>(keyword: &'a str, columnar_keyword: &'a str) -> ADFGVX<'a> {
        let polybius = Polybius::new(keyword, LATIN36, "ADFGVX", 2);
        let columnar = Columnar::new(columnar_keyword, LATIN36);
        ADFGVX{ polybius, columnar }
    }
}

impl crate::Cipher for ADFGVX<'_> {
    
    fn encrypt(&self, text: &str) -> String {
        let mut intermediate = self.polybius.encrypt(text);
        intermediate = self.columnar.encrypt(&intermediate);
        self.polybius.decrypt(&intermediate)
    }

    fn decrypt(&self, text: &str) -> String {
        let mut intermediate = self.polybius.encrypt(text);
        intermediate = self.columnar.decrypt(&intermediate);
        self.polybius.decrypt(&intermediate)
    }

}

impl fmt::Display for ADFGVX<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ADFGVX Composite Cipher\n{}\n{}",self.columnar,self.polybius)
    }
}
