use lazy_static::lazy_static;
use std::{collections::HashMap, fmt};

use crate::auxiliary::log2;
use crate::codes::binary::code_generators::FixedWidthInteger;
use crate::alphabets::LATIN26;

lazy_static! {
    pub static ref BACON_MAP: HashMap<char, String> = {
        let mut m = HashMap::new();
        let codes = FixedWidthInteger::new(5);
        for (l,c) in LATIN26.chars().zip(codes) {
            m.insert(l, c);
        }
        m
    };

    pub static ref BACON_MAP_INV: HashMap<String, char> = {
        let mut m = HashMap::new();
        let codes = FixedWidthInteger::new(5);
        for (l,c) in LATIN26.chars().zip(codes) {
            m.insert(c,l);
        }
        m
    };

}

pub struct Bacon<'a> {
    map: HashMap<char, String>,
    map_inv: HashMap<String, char>,
    width: usize,
    alphabet: &'a str,
}

impl Bacon<'_> {

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
        Bacon{ map, map_inv, width, alphabet }
        
    }

    pub fn default() -> Bacon<'static> {
        Bacon{ map: BACON_MAP.clone(), map_inv: BACON_MAP_INV.clone(), width: 5, alphabet: LATIN26 }
    }


}

impl fmt::Display for Bacon<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Bacon Code")
    }
}

impl crate::Code for Bacon<'_> {
    fn encode(&self, text: &str) -> String {
        let mut out = "".to_string();
        for s in text.chars() {
            out.push_str(self.map.get(&s).expect(&format!("The symbol `{}` is not in the alphabet",s)))
        }
        out
    }

    fn decode(&self, text: &str) -> String {
        let mut out = "".to_string();
        let w = self.width;
        for p in 0..(text.len()/w) {
            let group = &text[(p*w)..(p*w)+w];
            out.push(*self.map_inv.get(group).expect(&format!("the group `{}` is not valid for this Bacon Code",group)) )
        }
        out
    }

    fn char_map(&self) -> String {
        let mut out = String::new();
        for c in self.alphabet.chars() {
            out.push_str(&format!("{}  {}\n",c,self.map[&c]))
        }
        out
    }
}

