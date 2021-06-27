use itertools::Itertools;
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
    // Base64 is unusual in that is is for encoding binary data as text data, not the other way around
    // Here we just assume the text for encoding is binary
    fn encode(&self, text: &str) -> String {
        let mut out = String::new();

        let mut prep_text = text.to_string();
        let mut padding = 0;
        if prep_text.len() % 6 == 2 {
            prep_text.push_str("0000");
            padding = 2;
        }
        if prep_text.len() % 6 == 4 {
            prep_text.push_str("00");
            padding = 1;
        }


        let raw_chunks = prep_text.chars().chunks(6);
        let groups = raw_chunks.into_iter().map(|x| x.collect::<String>());

        for g in groups {
            out.push(*self.map_inv.get(&g).expect(&format!("the group `{}` is not valid for Base64",g)) )
        }
        if padding == 2 {
            out.push_str("==")
        } else {
            out.push('=')
        }
        out
    }

    fn decode(&self, text: &str) -> String {
        let mut out = String::new();
        for s in text.chars() {
            if s == '=' {
                continue
            }
            out.push_str(self.map.get(&s).expect(&format!("The symbol `{}` is not in the MIME Base64 alphabet",s)))
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
