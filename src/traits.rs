
pub trait Cipher { 
    fn encrypt(&self, text: &str) -> String;
    fn decrypt(&self, text: &str) -> String;
}

pub trait Code { 
    fn encode(&self, text: &str) -> String;
    fn decode(&self, text: &str) -> String;
    fn char_map(&self) -> String;
}