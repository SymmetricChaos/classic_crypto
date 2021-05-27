use itertools::Itertools;

use crate::alphabets::{LATIN26,scramble_alphabet};

#[derive(Clone,Debug)]
pub struct BATCO {
    cipher_rows: Vec<String>,
    key_cols: Vec<String>,
}



//["2", "3", "4", "5", "6", "7", "0", "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "CH", "."];
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

        BATCO{ cipher_rows, key_cols }
    }

    pub fn key_section(&self) -> String {
        let mut s = String::new();
        s.push_str("2 3 4 5 6 7");
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
        let mut s = String::new();
        s.push_str(" 0  0  1  2  3  4  5  6  7  8  9 CH  .");
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

    pub fn key_to_row(&self, key: &str) -> usize {
        let c = key.chars().collect_vec();
        let x = c[0].to_digit(10).unwrap() as usize;
        let alpha = &self.key_cols[x-2];
        alpha.chars().position(|x| x == c[1]).unwrap()
    }

/*     pub fn encrypt(&self, text: &str, key: &str) -> String {
        if text.chars().count() > 22 {
            panic!("BATCO messages are limited to 22 characters per key for security reasons")
        }
        
    }

    pub fn decrypt(&self, text: &str, key: &str) -> String {
        
    } */
}