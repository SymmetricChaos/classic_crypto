use std::fmt;


pub struct Porta<'a> {
    key: &'a str,
    alphabet: &'a str,
    length: usize,
}

impl Porta<'_> {
    pub fn new<'a>(key: &'a str, alphabet: &'a str) -> Porta<'a> {

    }
}

impl crate::Cipher for Porta<'_> {

    fn encrypt(&self, text: &str) -> String {

    }

    // The Porta cipher is involutive
    fn decrypt(&self, text: &str) -> String {
        self.encrypt(text)
    }

}

impl fmt::Display for Porta<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Porta Cipher\nkey: {:?}",self.key)
    }
}