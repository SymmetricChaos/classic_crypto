use std::fmt;
use std::{ fs::File, io::{Error, Read}};


use crate::alphabets::LATIN26;
use super::auxilliary::PORTA_TABLEAUX;


pub struct RunningKey<'a> {
    tableaux: Vec<&'a str>,
    key_file: &'a str,
}

impl RunningKey<'_> {
    pub fn default<'a>(key_file: &'a str) -> RunningKey<'a> {
        RunningKey{ tableaux: PORTA_TABLEAUX.clone(), key_file }
    }

    pub fn tableaux(&self) -> String {
        let mut out = "  ".to_string();
        for c in LATIN26.chars() {
            out.push(c);
            out.push(' ')
        }
        out.push_str("\n");
        for (row, c) in self.tableaux.iter().zip(LATIN26.chars()) {
            out.push(c);
            out.push(' ');
            for r in row.chars() {
                out.push(r);
                out.push(' ')
            }
            out.push_str("\n")
        }
        out
    }


    // This is potentially very inefficient there are apparently ways to read utf8 characters into a buffer one at a time
    pub fn encrypt(&self, text: &str) -> Result<String,Error> {
        let mut key_text = String::with_capacity(text.chars().count());
        let mut source = File::open(self.key_file)?;
        source.read_to_string(&mut key_text)?;
        let key: Vec<usize> = key_text.chars().map(|c| LATIN26.chars().position(|x| x == c).unwrap() / 2 ).collect();

        let mut out = String::new();

        for (c, k) in text.chars().zip(key) {
            let p = self.tableaux[k].chars().position(|x| x == c).unwrap();
            out.push(LATIN26.chars().nth(p).unwrap())
        }

        Ok(out)
    }

    pub fn decrypt(&self, text: &str) -> Result<String,Error> {
        self.encrypt(text)
    }

}

impl fmt::Display for RunningKey<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Porta Running Key Cipher\nkey file: {:?}",self.key_file)
    }
}




