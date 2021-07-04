use std::fmt;
use std::{fs::File, io::{Error, Read, Write}};

use crate::ciphers::polybius::Polybius;
use crate::ciphers::transposition::columnar::Columnar;
use crate::alphabets::{LATIN25_J,LATIN26};

/// The ADFGX Cipher combines a 5x5 Polybius Square (replacing J with I) and Columnar Transposition to achieve surprisingly high security with a very simple cipher. It was later replaced by the ADFGVX Cipher which used a 6x6 Polybius Square.
pub struct ADFGX<'a> {
    polybius: Polybius,
    columnar: Columnar<'a>,
}


impl ADFGX<'_> {
    pub fn new<'a>(keyword: &'a str, columnar_keyword: &'a str) -> ADFGX<'a> {
        let polybius = Polybius::new(keyword, LATIN25_J, "ADFGX", 2);
        let columnar = Columnar::new(columnar_keyword, LATIN26);
        ADFGX{ polybius, columnar }
    }

}

impl crate::Cipher for ADFGX<'_> {

    fn encrypt(&self, text: &str) -> String {
        let text = text.replace('J',"I");
        let mut intermediate = self.polybius.encrypt(&text);
        intermediate = self.columnar.encrypt(&intermediate);
        self.polybius.decrypt(&intermediate)
    }

    fn decrypt(&self, text: &str) -> String {
        let mut intermediate = self.polybius.encrypt(text);
        intermediate = self.columnar.decrypt(&intermediate);
        self.polybius.decrypt(&intermediate)
    }

    fn encrypt_file(&self, source: &str, target: &str) -> Result<(),Error> {

        let mut target_file = File::create(target.to_string())?;
    
        let mut source_file = File::open(source)?;
        let mut source_text = String::new();
        source_file.read_to_string(&mut source_text)?;
    
        let ciphertext = self.encrypt(&source_text);
    
        target_file.write(ciphertext.as_bytes())?;

        Ok(())
    }

    fn decrypt_file(&self, source: &str, target: &str) -> Result<(),Error> {

        let mut target_file = File::create(target.to_string())?;
    
        let mut source_file = File::open(source)?;
        let mut source_text = String::new();
        source_file.read_to_string(&mut source_text)?;
    
        let ciphertext = self.decrypt(&source_text);
    
        target_file.write(ciphertext.as_bytes())?;

        Ok(())
    }

}

impl fmt::Display for ADFGX<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ADFGX Composite Cipher\n{}\n{}",self.columnar,self.polybius)
    }
}
