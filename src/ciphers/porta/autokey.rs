use std::{collections::VecDeque, fmt};
use std::{fs::File, io::{Error, Read, Write}};

//use crate::alphabets::LATIN26;
//use super::auxilliary::PORTA_TABLEAUX;




pub struct Autokey<'a> {
    tableaux: Vec<&'a str>,
    key: &'a str,
    key_vals: Vec<usize>,
    alphabet: &'a str,
}

impl Autokey<'_> {
    pub fn new<'a>(key: &'a str, tableaux: Vec<&'a str>, alphabet: &'a str) -> Autokey<'a> {
        let alen = alphabet.chars().count();
        if tableaux.len() != alen {
            panic!("the tableaux must have exactly one row for each character in the alphabet")
        }
        for row in tableaux.iter() {
            if row.chars().count() != alen {
                panic!("all rows of the tableaux must have the same length as the alphabet")
            }
            for c in row.chars() {
                if !alphabet.contains(c) {
                    panic!("all rows of the tableaux must be permutations of the alphabet")
                }
            }
        }
        let key_vals = key.chars().map(|c| alphabet.chars().position(|x| x == c).unwrap() ).collect();
        Autokey{ tableaux: tableaux.clone(), key, key_vals, alphabet }
    }

    pub fn tableaux(&self) -> String {
        let mut out = "  ".to_string();
        for c in self.alphabet.chars() {
            out.push(c);
            out.push(' ')
        }
        out.push_str("\n");
        for (row, c) in self.tableaux.iter().zip(self.alphabet.chars()) {
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

    fn encrypt_pair(&self, c: char, k: usize) -> char {
        let n = self.tableaux[k].chars().position(|x| x == c).unwrap();
        self.alphabet.chars().nth(n).unwrap()
    }

    fn decrypt_pair(&self, c: char, k: usize) -> char {
        let n = self.alphabet.chars().position(|x| x == c).unwrap();
        self.tableaux[k].chars().nth(n).unwrap()
    }

}

impl crate::Cipher for Autokey<'_> {

    fn encrypt(&self, text: &str) -> String {
        let mut out = String::new();
        let mut akey: VecDeque<usize> = self.key_vals.clone().into_iter().collect();
        for c in text.chars() {
            let k = akey.pop_front().unwrap();
            akey.push_back( self.alphabet.chars().position(|x| x == c).unwrap() );
            out.push( self.encrypt_pair(c,k) )
        }

        out
    }

    fn decrypt(&self, text: &str) -> String {
        let mut out = String::new();
        let mut akey: VecDeque<usize> = self.key_vals.clone().into_iter().collect();
        for c in text.chars() {
            let k = akey.pop_front().unwrap();
            let d = self.decrypt_pair(c,k);
            akey.push_back( self.alphabet.chars().position(|x| x == d).unwrap() );
            out.push( d )
        }
        out
    }

    fn encrypt_file(&self, source: &str, target: &str) -> Result<(),Error> {

        let mut target_file = File::create(target.to_string())?;
    
        let mut source_file = File::open(source)?;
        let mut source_text = String::new();
        source_file.read_to_string(&mut source_text)?;
    
        let ciphertext = self.encrypt(&source_text);
    
        target_file.write(ciphertext.as_bytes())?;

        Ok(())
    }

    fn decrypt_file(&self, source: &str, target: &str) -> Result<(),Error> {

        let mut target_file = File::create(target.to_string())?;
    
        let mut source_file = File::open(source)?;
        let mut source_text = String::new();
        source_file.read_to_string(&mut source_text)?;
    
        let ciphertext = self.decrypt(&source_text);
    
        target_file.write(ciphertext.as_bytes())?;

        Ok(())
    }

}

impl fmt::Display for Autokey<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Tableaux Autokey Cipher\nkey: {:?}",self.key)
    }
}