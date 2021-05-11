use std::fmt;
//use num::Integer;


// This will have to be a whole family of ciphers

pub struct Route<'a> {
    dimensions: (usize,usize),
    text_length: usize,
    key: &'a str,
}

impl Route<'_> {
    pub fn new<'a>(dimensions: (usize,usize), key: &'a str) -> Route {
        let text_length = dimensions.0 * dimensions.1;
        Route{ dimensions, text_length, key }
    }

    fn create_rows<'b>(&self, text: &'b str) -> Vec<&'b str> {
        let mut v = Vec::new();
        let l = self.dimensions.0;
        for i in 0..self.dimensions.0 {
            v.push(&text[l*i..l*i+l])
        }
        v
    }

    fn encrypt_snake(&self, text: &str) -> String {
        let tlen = text.chars().count();
        let mut out = String::with_capacity(tlen);
        out
    }

    fn encrypt_stripe(&self, text: &str) -> String {
        let tlen = text.chars().count();
        let mut out = String::with_capacity(tlen);
        out
    }

    fn encrypt_spiral(&self, text: &str) -> String {
        let tlen = text.chars().count();
        let mut out = String::with_capacity(tlen);
        out
    }


}

impl crate::Cipher for Route<'_> {

    fn encrypt(&self, text: &str) -> String {
        let tlen = text.chars().count();
        if tlen != self.text_length {
            panic!("For a route cipher using a {} by {} grid the text must have exactly {} symbols, the text provided has {} symbols",
                   self.dimensions.0, self.dimensions.1, self.text_length, tlen)
        }

        match self.key {
            "snake" => self.encrypt_snake(text),
            "stripe" => self.encrypt_stripe(text),
            _ => panic!("invalid key"),
        }
    }

    fn decrypt(&self, text: &str) -> String {
        String::new()
    }

}

impl fmt::Display for Route<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Route\nkey: {}",self.key)
    }
}