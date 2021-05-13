use std::{char, convert::TryInto};


pub fn bits_to_u32(text: &str) -> u32 {
    let mut out = 0u32;
    for (e, s) in text.chars().rev().enumerate() {
        match s {
            '0' => (),
            '1' => out += (2usize.pow(e.try_into().unwrap())) as u32,
            _ => panic!("bits can only be 0 or 1"),
        }
    }
    out
}



#[derive(Debug)]
pub struct UTF32 {}

impl UTF32 {

    pub fn default() -> UTF32 { UTF32{} }

    pub fn encode(&self, text: &str) -> String {
        let mut out = "".to_string();
        for s in text.chars() {
            out.push_str(&format!("{:032b}", s as u32))
        }
        out
    }

    pub fn decode(&self, text: &str) -> String {
        let mut out = String::new();
        let w = 32;
        for p in 0..(text.len()/w) {
            let group = &text[(p*w)..(p*w)+w];
            let n = bits_to_u32(group);
            out.push( char::from_u32(n).unwrap() )
        }
        out
    }

}


