pub trait Key {  }

pub trait Cipher {
    fn encode(&self, text: &str) -> String;

    fn decode(&self, text: &str) -> String;

    pub fn set_punctuation(&mut self, boolean: bool);

    pub fn set_whitespace(&mut self, boolean: bool);
    
}