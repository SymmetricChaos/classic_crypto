use lazy_static::lazy_static;
use std::collections::HashMap;
use num::Integer;

use crate::auxiliary::log2;
use crate::codes::binary::code_generators::BaconCode;

lazy_static! {
    pub static ref BACON_MAP: HashMap<char, String> = {
        let mut m = HashMap::new();
        let letters = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        let codes = BaconCode::new(5);
        for (l,c) in letters.chars().zip(codes) {
            m.insert(l, c);
        }
        m
    };

    pub static ref BACON_MAP_INV: HashMap<String, char> = {
        let mut m = HashMap::new();
        let letters = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        let codes = BaconCode::new(5);
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
}

impl Bacon {

    pub fn new(alphabet: &str) -> Bacon {
        let length = alphabet.chars().count();
        let width = log2(length);
        let codes = BaconCode::new(width);
        let mut map = HashMap::new();
        let mut map_inv = HashMap::new();
        for (l,c) in alphabet.chars().zip(codes) {
            println!("{}",c.clone());
            map.insert(l,c.clone() );
            map_inv.insert(c, l);
        }
        Bacon{ map, map_inv, width }
        
    }

    pub fn default() -> Bacon {
        Bacon{ map: BACON_MAP.clone(), map_inv: BACON_MAP_INV.clone(), width: 5 }
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

#[test]
fn bacon_default() {
    let bacon = Bacon::default();
    let plaintext = "THEQUICKBROWNFOXJUMPSOVERTHELAZYDOG";
    let coded = bacon.encode(plaintext);
    let decoded = bacon.decode(&coded);

    println!("{}",plaintext);
    println!("{}",coded);
    println!("{}",decoded);
}

#[test]
fn bacon_ascii() {
    use crate::alphabets::ASCII94;
    let bacon = Bacon::new(ASCII94);
    let plaintext = "THEQUICK(BROWN)FOXJUMPS!OVERTHELAZYDOG?";
    let coded = bacon.encode(plaintext);
    let decoded = bacon.decode(&coded);

    println!("{}",plaintext);
    println!("{}",coded);
    println!("{}",decoded);
}