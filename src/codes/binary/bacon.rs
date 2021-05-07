use lazy_static::lazy_static;
use std::{collections::HashMap, fmt};

use crate::auxiliary::log2;
use crate::codes::binary::code_generators::FixedWidthInteger;

lazy_static! {
    pub static ref BACON_MAP: HashMap<char, String> = {
        let mut m = HashMap::new();
        let letters = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        let codes = FixedWidthInteger::new(5);
        for (l,c) in letters.chars().zip(codes) {
            m.insert(l, c);
        }
        m
    };

    pub static ref BACON_MAP_INV: HashMap<String, char> = {
        let mut m = HashMap::new();
        let letters = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        let codes = FixedWidthInteger::new(5);
        for (l,c) in letters.chars().zip(codes) {
            m.insert(c,l);
        }
        m
    };

}

pub struct Bacon {
    map: HashMap<char, String>,
    map_inv: HashMap<String, char>,
    width: usize,
    alphabet: String,
}

impl Bacon {

    pub fn new(alphabet: &str) -> Bacon {
        let length = alphabet.chars().count();
        let width = log2(length);
        let codes = FixedWidthInteger::new(width);
        let mut map = HashMap::new();
        let mut map_inv = HashMap::new();
        for (l,c) in alphabet.chars().zip(codes) {
            map.insert(l,c.clone() );
            map_inv.insert(c, l);
        }
        Bacon{ map, map_inv, width, alphabet: alphabet.to_string() }
        
    }

    pub fn default() -> Bacon {
        Bacon{ map: BACON_MAP.clone(), map_inv: BACON_MAP_INV.clone(), width: 5, alphabet: "ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_string() }
    }

    pub fn encode(&self, text: &str) -> String {
        let mut out = "".to_string();
        for s in text.chars() {
            out.push_str(self.map.get(&s).expect(&format!("The symbol `{}` is not in the alphabet",s)))
        }
        out
    }

    pub fn decode(&self, text: &str) -> String {
        let mut out = "".to_string();
        let w = self.width;
        for p in 0..(text.len()/w) {
            let group = &text[(p*w)..(p*w)+w];
            out.push(*self.map_inv.get(group).expect(&format!("the group `{}` is not valid for this Bacon Code",group)) )
        }
        out
    }
}

impl fmt::Display for Bacon {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = "".to_string();
        for c in self.alphabet.chars() {
            s.push_str(&format!("{}: {}\n",c,self.map[&c]))
        }
        write!(f, "Bacon Code\n{}",s)
    }
}


#[test]
fn bacon_default() {
    let bacon = Bacon::default();
    let plaintext = "THEQUICKBROWNFOXJUMPSOVERTHELAZYDOG";
    let coded = bacon.encode(plaintext);
    let decoded = bacon.decode(&coded);

    //println!("{}",bacon);

    println!("{}",plaintext);
    println!("{}",coded);
    println!("{}",decoded);
}

#[test]
fn bacon_ascii() {
    use crate::alphabets::ASCII95;
    let bacon = Bacon::new(ASCII95);
    let plaintext = "The quick (BROWN) fox jumps over the [LAZY] dog!";
    let coded = bacon.encode(plaintext);
    let decoded = bacon.decode(&coded);

    //println!("{}",bacon);

    println!("{}",plaintext);
    println!("{}",coded);
    println!("{}",decoded);
}