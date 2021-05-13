use std::{collections::HashMap};
use lazy_static::lazy_static;

use crate::codes::binary::code_generators::FixedWidthInteger;
use crate::alphabets::ASCII128;

lazy_static! {
    pub static ref ASCII_MAP8: HashMap<char, String> = {
        let mut m = HashMap::new();
        let codes = FixedWidthInteger::new(8);
        for (l,c) in ASCII128.chars().zip(codes) {
            m.insert(l, c);
        }
        m
    };

    pub static ref ASCII_MAP_INV8: HashMap<String, char> = {
        let mut m = HashMap::new();
        let codes = FixedWidthInteger::new(8);
        for (l,c) in ASCII128.chars().zip(codes) {
            m.insert(c,l);
        }
        m
    };

    pub static ref ASCII_MAP7: HashMap<char, String> = {
        let mut m = HashMap::new();
        let codes = FixedWidthInteger::new(7);
        for (l,c) in ASCII128.chars().zip(codes) {
            m.insert(l, c);
        }
        m
    };

    pub static ref ASCII_MAP_INV7: HashMap<String, char> = {
        let mut m = HashMap::new();
        let codes = FixedWidthInteger::new(7);
        for (l,c) in ASCII128.chars().zip(codes) {
            m.insert(c,l);
        }
        m
    };

}


pub struct ASCII {
    map: HashMap<char,String>,
    map_inv: HashMap<String,char>,
    width: usize,
    alphabet: &'static str,
}

impl ASCII {

    pub fn default8() -> ASCII {
        ASCII{ map: ASCII_MAP8.clone(), map_inv: ASCII_MAP_INV8.clone(), width: 8, alphabet: ASCII128 }
    }

    pub fn default7() -> ASCII {
        ASCII{ map: ASCII_MAP7.clone(), map_inv: ASCII_MAP_INV7.clone(), width: 7, alphabet: ASCII128 }
    }

}


impl crate::Code for ASCII {

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
            out.push(*self.map_inv.get(group).expect(&format!("the group `{}` is not valid for this version of ASCII",group)) )
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