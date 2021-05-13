use std::collections::HashMap;
use crate::codes::binary::code_generators::FibonacciCode;

// Prefix Code


pub struct Fibonacci<'a> {
    map: HashMap<char, String>,
    map_inv: HashMap<String, char>,
    alphabet: &'a str,
}

impl Fibonacci<'_> {

    pub fn new<'a>(alphabet: &'a str) -> Fibonacci {
        let codes = FibonacciCode::new();
        let mut map = HashMap::new();
        let mut map_inv = HashMap::new();
        for (l,c) in alphabet.chars().zip(codes) {
            map.insert(l,c.clone() );
            map_inv.insert(c, l);
        }
        Fibonacci{ map, map_inv, alphabet }
    }

}

impl crate::Code for Fibonacci<'_> {

    fn encode(&self, text: &str) -> String {
        let mut out = "".to_string();
        for s in text.chars() {
            out.push_str(&self.map[&s])
        }
        out
    }

    fn decode(&self, text: &str) -> String {
        let mut output = "".to_string();
        let mut buffer = "".to_string();
        for b in text.chars() {
            buffer.push(b);
            if self.map_inv.contains_key(&buffer) {
                output.push(self.map_inv[&buffer]);
                buffer = "".to_string();
            }
        }
        output
    }

    fn char_map(&self) -> String {
        let mut out = String::new();
        for c in self.alphabet.chars() {
            out.push_str(&format!("{}  {}\n",c,self.map[&c]))
        }
        out
    }
}