use std::cell::Cell;
use itertools::Itertools;
use rand::{Rng, thread_rng};

use crate::alphabets::{LATIN26,scramble_alphabet};

#[derive(Clone,Debug)]
pub struct BATCO {
    cipher_rows: Vec<String>,
    key_cols: Vec<String>,
    key: Cell<usize>,
}

/*
As it only encodes digits BATCO is designed to be used with various sets of vocabulary cards.
Assuming the enemy knows the contents of the vocabulary cards the security of BATCO comes from 
the messages being extremely short, no more than 22 digits before a key change, and the BATCO
page being changed regularly. Furthermore tactical information should not be relevant for long
enough for serious cryptanalysis to be applied.
*/


//["0", "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "CH", "."];
impl BATCO {
    pub fn random() -> BATCO {
        let mut cipher_rows = Vec::with_capacity(26);
        for _ in 0..26 {
            cipher_rows.push( scramble_alphabet(LATIN26) )
        }
        let mut key_cols = Vec::with_capacity(6);
        for _ in 0..6 {
            key_cols.push( scramble_alphabet(LATIN26) )
        }

        BATCO{ cipher_rows, key_cols, key: Cell::new(0) }
    }

    pub fn key_section(&self) -> String {
        let mut s = "2 3 4 5 6 7".to_string();
        for i in 0..26 {
            s.push('\n');
            for j in 0..6 {
                s.push( self.key_cols[j].chars().nth(i).unwrap() );
                s.push(' ')
            }
        }
        s
    }

    pub fn cipher_section(&self) -> String {
        let mut s = " 0  0  1  2  3  4  5  6  7  8  9 CH  .\n".to_string();
        for r in self.cipher_rows.iter() {
            s.push('\n');
            let v = r.chars().collect_vec();
            let ch = v.chunks(2).map(|x| format!("{}{} ",x[0],x[1])).collect_vec();
            for pair in ch {
                s.push_str(&pair)
            }
        }
        s
    }

    pub fn key_row(&self) -> String {
        let mut s = " 0  0  1  2  3  4  5  6  7  8  9 CH  .\n".to_string();
        let v = self.cipher_rows[self.key.get()].chars().collect_vec();
        let ch = v.chunks(2).map(|x| format!("{}{} ",x[0],x[1])).collect_vec();
        for pair in ch {
            s.push_str(&pair)
        }
        s
    }

    // The key is usize but its defined by a digit from 2 to 7 (to select a column) and a letter (to select a row in that column)
    pub fn set_key(&self, key: &str) {
        self.key.set( self.key_to_row(key) )
    }

    fn key_to_row(&self, key: &str) -> usize {
        let c = key.chars().collect_vec();
        let x = c[0].to_digit(10).unwrap() as usize;
        let alpha = &self.key_cols[x-2];
        alpha.chars().position(|x| x == c[1]).unwrap()
    }

    pub fn encrypt(&self, text: &str) -> String {
        if text.chars().count() > 22 {
            panic!("BATCO messages are limited to 22 characters per key for security reasons")
        }
        let mut rng = thread_rng();
        let alphabet = &self.cipher_rows[self.key.get()];
        let mut symbols = text.chars();
        let breaks = [0,4,6,8,10,12,14,16,18,20,22,24,26];

        let mut out = String::with_capacity(text.len());
        // loop while c is Some(char)
        while let Some(c) = symbols.next() {
            // H is ignored since it always follows C
            if c == 'H' { continue }
            // Convert the symbol to a numbers
            let v = match c {
                '0' => 0,
                '1' => 1,
                '2' => 2,
                '3' => 3,
                '4' => 4,
                '5' => 5,
                '6' => 6,
                '7' => 7,
                '8' => 8,
                '9' => 9,
                'C' => 10,
                '.' => 11,
                _ => panic!("the only valid symbols are digits, CH, and the period")
            };

            // Select a random symbol from the allowed range for that number
            let pos = rng.gen_range(breaks[v]..breaks[v+1]);
            out.push( alphabet.chars().nth(pos).unwrap() );
        }
        out
    }

    pub fn decrypt(&self, text: &str) -> String {
        let alphabet = &self.cipher_rows[self.key.get()];
        let digits = ["0", "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "CH", "."];
        let symbols = text.chars();

        let mut out = String::with_capacity(text.len());
        for c in symbols {
            let pos = alphabet.chars().position(|x| x == c).unwrap()/2;
            out.push_str(digits[pos])
        }

        out
    }
}