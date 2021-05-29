use std::cell::Cell;
use rand::{Rng, thread_rng};

use crate::alphabets::scramble_alphabet;

pub struct DRYAD {
    cipher_rows: Vec<String>,
    message_key: Cell<usize>,
}

impl DRYAD {

    pub fn random() -> DRYAD {
        let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXY";

        let mut cipher_rows = Vec::with_capacity(26);
        for _ in 0..25 {
            cipher_rows.push( scramble_alphabet(alphabet) )
        }

        DRYAD{ cipher_rows, message_key: Cell::new(0) }
    }

    pub fn set_key(&self, message_key: char) {
        if message_key.is_ascii_uppercase() {
            if message_key == 'Z' {
                panic!("There is no row Z in the DRYAD system")
            }
            let n = message_key as u8 as usize - 65;
            self.message_key.set( n )
        }

    }

    pub fn code_page(&self) -> String {
        let breaks = [0,4,7,10,12,14,17,19,21,23,25];
        let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXY";

        let mut s = "      0    1    2   3   4    5   6   7   8   9".to_string();
        for (i, c) in (0..25).zip(alphabet.chars()) {
            s.push('\n');
            s.push(c);
            s.push_str("  ");
            let r = &self.cipher_rows[i];
            for p in 0..10 {
                s.push_str(&r[breaks[p]..breaks[p+1]]);
                s.push_str("  ");
            }
        }
        s
    }

}

impl crate::Cipher for DRYAD {
    fn encrypt(&self, text: &str) -> String {
        let breaks = [0,4,7,10,12,14,17,19,21,23,25];
        let alphabet = &self.cipher_rows[self.message_key.get()];

        let mut out = String::with_capacity(text.len());
        
        let mut rng = thread_rng();
        for c in text.chars() {
            if !c.is_ascii_digit() {
                panic!("DRYAD only encrypts digits")
            }
            let n = c.to_digit(10).unwrap() as usize;
            let pos = rng.gen_range(breaks[n]..breaks[n+1]);
            out.push( alphabet.chars().nth(pos).unwrap() );
        }

        out
    }

    fn decrypt(&self, text: &str) -> String {
        let digits = "0123456789";
        let alphabet = &self.cipher_rows[self.message_key.get()];
        
        let mut out = String::with_capacity(text.len());
        for c in text.chars() {
            let pos = alphabet.chars().position(|x| x == c).unwrap();
            let d = match pos {
                0..=3 => 0,
                4..=6 => 1,
                5..=9 => 2,
                10..=11 => 3,
                12..=13 => 4,
                14..=16 => 5,
                17..=18 => 6,
                19..=20 => 7,
                21..=22 => 8,
                23..=24 => 9,
                _ => panic!("invalid position encountered")
            };
            out.push(digits.chars().nth(d).unwrap())
        }
        
        out
    }

}