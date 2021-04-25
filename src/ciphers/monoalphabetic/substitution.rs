use std::fmt;
use std::collections::HashMap;



pub struct Substitution {
    key1: String,
    key2: String,
    map: HashMap<char,char>,
    map_inv: HashMap<char,char>,
    whitespace: bool,
}

impl Substitution {
    pub fn new(key1: &str, key2: &str) -> Substitution {
        if key1.chars().count() != key2.chars().count() {
            panic!("key1 is of length {} but key2 is of length {}",key1.chars().count(),key2.chars().count())
        }
        let mut map = HashMap::new();
        let mut map_inv = HashMap::new();
        for (a, b) in key1.chars().zip(key2.chars()) {
            if a.is_whitespace() || b.is_whitespace() {
                panic!("whitespace is not allowed as a substitution symbol")
            }
            map.insert(a, b);
            map_inv.insert(b, a);
        }
        Substitution{ key1: key1.to_string(), key2: key2.to_string(), map, map_inv, whitespace: false }
    }

    pub fn set_whitespace(&mut self, boolean: bool) {
        self.whitespace = boolean
    }

    pub fn set_key(&mut self, key1: &str, key2: &str) {
        if key1.len() != key2.len() {
            panic!("Must provide two equal length strings")
        }
        let mut map = HashMap::new();
        let mut map_inv = HashMap::new();
        for (a, b) in key1.chars().zip(key2.chars()) {
            if a.is_whitespace() || b.is_whitespace() {
                panic!("whitespace is not allowed as a substitution symbol")
            }
            map.insert(a, b);
            map_inv.insert(b, a);
        }
        self.key1 = key1.to_string();
        self.key2 = key2.to_string();
        self.map = map;
        self.map_inv = map_inv;
    }

    pub fn encode(&self, text: &str) -> String {
        let mut out = "".to_string();
        for c in text.chars() {
            if self.whitespace && c.is_whitespace() {
                out.push(c)
            } else {
                out.push(self.map[&c])
            }
        }
        out
    }

    pub fn decode(&self, text: &str) -> String {
        let mut out = "".to_string();
        for c in text.chars() {
            if self.whitespace && c.is_whitespace() {
                out.push(c)
            } else {
                out.push(self.map_inv[&c])
            }
        }
        out
    }
}

impl fmt::Display for Substitution {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Simple Substitution\n{}\n{}",self.key1,self.key2)
    }
}

#[test]
fn substitution() {
    let substitution = Substitution::new("abcdefghijklmnopqrstuvwxyz","QWERTYUIOPASDFGHJKLZXCVBNM");
    
    println!("{}\n",substitution);

    let plaintext = "thequickbrownfoxjumpsoverthelazydog";
    let ciphertext = substitution.encode(plaintext);
    let decrypt = substitution.decode(&ciphertext);

    println!("{}\n{}\n{}\n",plaintext,ciphertext,decrypt)

    
}

#[test]
fn substitution_unicode() {
    let substitution = Substitution::new("abcdefghijklmnopqrstuvwxyz","🌰🌱🌲🌳🌴🌵🌶️🌷🌸🌹🌺🌻🌼🌽🌾🌿🍀🍁🍂🍃🍄🍅🍆🍇🍈");
    
    let plaintext = "thequickbrownfoxjumpsoverthelazydog";
    let ciphertext = substitution.encode(plaintext);
    let decrypt = substitution.decode(&ciphertext);

    println!("{}\n{}\n{}\n",plaintext,ciphertext,decrypt)

    
}