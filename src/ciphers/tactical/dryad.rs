use std::cell::Cell;
use itertools::Itertools;
use rand::{Rng, thread_rng};

use crate::alphabets::scramble_alphabet;

pub struct DRYAD {
    cipher_rows: Vec<String>,
}

impl DRYAD {

    pub fn random() -> DRYAD {
        let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXY";

        let mut cipher_rows = Vec::with_capacity(26);
        for _ in 0..25 {
            cipher_rows.push( scramble_alphabet(alphabet) )
        }

        DRYAD{ cipher_rows }
    }

    pub fn code_page(&self) -> String {
        let breaks = [0,4,7,10,12,14,17,19,21,23,25];
        let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXY";

        let mut s = "      0   1   2  3  4   5  6  7  8  9".to_string();
        for (i, c) in (0..25).zip(alphabet.chars()) {
            s.push('\n');
            s.push(c);
            s.push_str("  ");
            let r = &self.cipher_rows[i];
            for p in 0..10 {
                s.push_str(&r[breaks[p]..breaks[p+1]]);
                s.push(' ')
            }
        }
        s
    }

}

/* impl Cipher for DRYAD {
    fn encrypt(&self, text: &str) -> String {
        let breaks = [0,4,7,10,12,14,17,19,21,23,25];
        let out = String::with_capacity(text.len());

        out
    }

    fn decrypt(&self, text: &str) -> String {
        let breaks = [0,4,7,10,12,14,17,19,21,23,25];
        let out = String::with_capacity(text.len());

        out
    }
} */