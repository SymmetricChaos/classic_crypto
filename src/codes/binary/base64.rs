use lazy_static::lazy_static;
use std::collections::HashMap;

use crate::alphabets::BASE64;
use super::code_generators::FixedWidthInteger;

lazy_static! {

    pub static ref B64_MAP: HashMap<char, String> = {
        let mut m = HashMap::new();
        let codes = FixedWidthInteger::new(6);
        for (l,c) in BASE64.chars().zip(codes) {
            m.insert(l, c);
        }
        m
    };

    pub static ref B64_MAP_INV: HashMap<String, char> = {
        let mut m = HashMap::new();
        let codes = FixedWidthInteger::new(6);
        for (l,c) in BASE64.chars().zip(codes) {
            m.insert(c, l);
        }
        m
    };

}


pub struct Base64<'a> {
    map: HashMap<char, String>,
    map_inv: HashMap<String, char>,
    alphabet: &'a str,
}

impl Base64<'_> {

    pub fn default() -> Base64<'static> {
        Base64{ map: B64_MAP.clone(), map_inv: B64_MAP_INV.clone(), alphabet: BASE64 }
    }


}

impl crate::Code for Base64<'_> {
    fn encode(&self, text: &str) -> String {
        let mut out = "".to_string();
        for s in text.chars() {
            out.push_str(self.map.get(&s).expect(&format!("The symbol `{}` is not in the MIME Base64 alphabet",s)))
        }
        out
    }

    fn decode(&self, text: &str) -> String {
        let mut out = "".to_string();
        let w = 6;
        for p in 0..(text.len()/w) {
            let group = &text[(p*w)..(p*w)+w];
            out.push(*self.map_inv.get(group).expect(&format!("the group `{}` is not valid for Base64",group)) )
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
