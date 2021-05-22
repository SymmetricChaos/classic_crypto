use std::{char, convert::TryInto};

use itertools::Itertools;


fn bits_to_u32(text: &str) -> u32 {
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

fn utf32_bits_encode(text: &str) -> String {
    let mut out = String::with_capacity(text.len()*32);
    for s in text.chars() {
        out.push_str(&format!("{:032b}", s as u32))
    }
    out
}

fn utf32_bits_decode(text: &str) -> String {
    let mut out = String::new();
    let w = 32;
    for p in 0..(text.len()/w) {
        let group = &text[(p*w)..(p*w)+w];
        let n = bits_to_u32(group);
        out.push( char::from_u32(n).unwrap() )
    }
    out
}





fn hex_to_u32(text: &str) -> u32 {
    let mut out = 0u32;
    for (e, s) in text.chars().rev().enumerate() {
        let n = s.to_digit(16).unwrap() as usize;
        out += (16usize.pow(e.try_into().unwrap()) * n) as u32
    }
    out
}

fn utf32_hex_encode(text: &str) -> String {
    let mut out = String::with_capacity(text.len()*2);
    for s in text.chars() {
        out.push_str(&format!("{:08x}", s as u32))
    }
    out
}

fn utf32_hex_decode(text: &str) -> String {
    let mut out = String::new();
    let w = 8;
    for p in 0..(text.len()/w) {
        let group = &text[(p*w)..(p*w)+w];
        let n = hex_to_u32(group);
        out.push( char::from_u32(n).unwrap() )
    }
    out
}





#[derive(Debug)]
pub struct UTF32 {
    mode: String,
}

impl UTF32 {

    pub fn bits() -> UTF32 { UTF32{ mode: "bits".to_string() } }

    pub fn hex() -> UTF32 { UTF32{ mode: "hex".to_string() } }

    pub fn encode(&self, text: &str) -> String {
        match self.mode.as_str() {
            "bits" => utf32_bits_encode(text),
            "hex" => utf32_hex_encode(text),
            _ => panic!("invlid mode")
        }
    }

    pub fn decode(&self, text: &str) -> String {
        match self.mode.as_str() {
            "bits" => utf32_bits_decode(text),
            "hex" => utf32_hex_decode(text),
            _ => panic!("invlid mode")
        }
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
        if text.len() % 8 != 0 {
            panic!("text must have a multiple of 8 symbols")
        }
        let mut out = String::new();

        let bytes = text.chars().chunks(8).into_iter().map(|x| x.collect::<String>()).collect_vec();
        let mut bytes_iter = bytes.iter();

        loop{
            let byte = {
                match bytes_iter.next() {
                    Some(s) => s,
                    None => break
                }
            };

            let mut bits = byte.chars();
            let s = bits.next().unwrap();

            // Single byte character
            if s == '0' {
                let n = bits_to_u32(&byte);
                out.push( char::from_u32(n).unwrap() )

            // Multi byte characters
            } else {
                let mut buffer = String::with_capacity(22);

                let width = {
                    if &byte[0..4] == "1111"{
                        4
                    } else if &byte[0..3] == "111"{
                        3
                    } else if &byte[0..2] == "11"{
                        2
                    } else {
                        panic!("INVALID CHARACTER")
                    }
                };

                buffer.push_str( &byte[width+1..] );
                for _ in 0..width-1 {
                    let nbyte = &bytes_iter.next().unwrap()[2..];
                    buffer.push_str( nbyte )

                }

                let n = bits_to_u32(&buffer);
                out.push( char::from_u32(n).unwrap() )
            }
        }

        out
    }
}