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


#[derive(Debug)]
pub struct UTF8 {}

impl UTF8 {

    pub fn default() -> UTF8 { UTF8{} }

    pub fn encode(&self, text: &str) -> String {
        let mut out = String::new();
        let mut b = text.bytes().peekable();

        // We never need to read in more than 4 bytes to the buffer
        let mut buf = Vec::with_capacity(4);

        // headers are: 0, 110, 1110, 11110
        loop {

            // Stop if needed
            if b.peek().is_none() {
                break
            }

            buf.clear();
            buf.push( b.next().unwrap() );

            if buf[0] < 128 {

            } else if buf[0] < 224 {
                buf.push( b.next().unwrap() );

            } else if buf[0] < 240 {
                buf.push( b.next().unwrap() );
                buf.push( b.next().unwrap() );

            } else { // if n < 248
                buf.push( b.next().unwrap() );
                buf.push( b.next().unwrap() );
                buf.push( b.next().unwrap() );

            }

            for byte in buf.iter() {
                out.push_str( &format!("{:08b}",byte) )
            }
            
        }
        out
    }

    pub fn decode(&self, text: &str) -> String {
        let mut out = String::new();

        out
    }

}


#[test]
fn check_char() {
    let utf8 = UTF8::default();
    println!("{}", utf8.encode("a"));
    println!("{}", utf8.encode("€"));
    println!("{}", utf8.encode("€a"));
}