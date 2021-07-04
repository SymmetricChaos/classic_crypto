use std::fmt;
use std::{fs::File, io::{Error, Read, Write}};

use crate::auxiliary::string_to_nums;

/// Polyalphabetic substitution ciphers (like the Beaufort, Porta, and Vigenere) are all special cases of the Tableaux Cipher with vastly simpler keys.
pub struct Tableaux<'a> {
    tableaux: Vec<&'a str>,
    tableaux_vals: Vec<Vec<usize>>,
    key: &'a str,
    key_vals: Vec<usize>,
    alphabet: &'a str,
    length: usize
}

impl Tableaux<'_> {
    pub fn new<'a>(key: &'a str, tableaux: Vec<&'a str>, alphabet: &'a str) -> Tableaux<'a> {
        let mut tableaux_vals = Vec::with_capacity(tableaux.len());
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
            tableaux_vals.push(string_to_nums(row,alphabet))
        }
        let key_vals = key.chars().map(|c| alphabet.chars().position(|x| x == c).unwrap() ).collect();
        Tableaux{ tableaux: tableaux.clone(), tableaux_vals, key, key_vals, alphabet, length: alen }
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

impl crate::PolyalphabeticCipher for Tableaux<'_> {
    fn encrypt_char(&self, t: usize, k: usize) -> usize {
        self.tableaux_vals[k].iter().position(|x| *x == t).unwrap()
    }

    fn decrypt_char(&self, t: usize, k: usize) -> usize {
        self.tableaux_vals[k][t]
    }

    fn text_to_nums(&self, text: &str) -> Vec<usize> {
        let nums: Vec<usize> = text.chars().map( |x| self.alphabet.chars().position(|c| c == x).unwrap() ).collect();
        nums
    }

    fn nums_to_text(&self, nums: Vec<usize>) -> String {
        nums.iter().map(|n| self.alphabet.chars().nth(*n).unwrap()).collect::<String>()
    }

    fn key_vals(&self) -> Vec<usize> {
        self.key_vals.clone()
    }

    fn alphabet_len(&self) -> usize {
        self.length
    }
}

impl crate::Cipher for Tableaux<'_> {

    fn encrypt(&self, text: &str) -> String {
        let mut out = String::new();
        let ckey = self.key_vals.iter().cycle();
        for (c, k) in text.chars().zip(ckey) {
            out.push( self.encrypt_pair(c,*k) )
        }
        out
    }

    fn decrypt(&self, text: &str) -> String {
        let mut out = String::new();
        let ckey = self.key_vals.iter().cycle();
        for (c, k) in text.chars().zip(ckey) {
            out.push( self.decrypt_pair(c,*k) )
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

impl fmt::Display for Tableaux<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Tableaux Cipher\nkey: {:?}",self.key)
    }
}