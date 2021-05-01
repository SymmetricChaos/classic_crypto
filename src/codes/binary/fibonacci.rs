use std::{collections::HashMap, fmt};
use crate::codes::binary::code_generators::FibonacciCode;

// Prefix Code


pub struct Fibonacci {
    map: HashMap<char, String>,
    map_inv: HashMap<String, char>,
    alphabet: String,
}

impl Fibonacci {

    pub fn new(alphabet: &str) -> Fibonacci {
        let codes = FibonacciCode::new();
        let mut map = HashMap::new();
        let mut map_inv = HashMap::new();
        for (l,c) in alphabet.chars().zip(codes) {
            map.insert(l,c.clone() );
            map_inv.insert(c, l);
        }
        Fibonacci{ map, map_inv, alphabet: alphabet.to_string() }
        
    }

    pub fn encode(&self, text: &str) -> String {
        let mut out = "".to_string();
        for s in text.chars() {
            out.push_str(&self.map[&s])
        }
        out
    }

    pub fn decode(&self, text: &str) -> String {
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
}

impl fmt::Display for Fibonacci {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = "".to_string();
        for c in self.alphabet.chars() {
            s.push_str(&format!("{}: {}\n",c,self.map[&c]))
        }
        write!(f, "Fibonacci Code\n{}",s)
    }
}

#[test]
fn fibonacci_code() {
    use crate::alphabets::LATIN26_FREQ;
    let fib = Fibonacci::new(LATIN26_FREQ);
    let plaintext = "THEQUICK";
    let coded = fib.encode(plaintext);
    let decoded = fib.decode(&coded);

    println!("{}",fib);

    println!("{}",plaintext);
    println!("{}",coded);
    println!("{}",decoded);
}