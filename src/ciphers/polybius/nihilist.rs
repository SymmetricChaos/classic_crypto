use std::fmt;

use crate::ciphers::polybius::polybius::{Polybius,polybius_from_keyword};


pub struct Nihilist {
    polybius: Polybius,
    vigenere: Vec<usize>,
}


impl Nihilist {
    pub fn new(polybius_key: &str, vigenere_key: Vec<usize>) -> Nihilist {
        let polybius = polybius_from_keyword(polybius_key,"ABCDFEGHIKLMNOPQRSTUVWXYZ","12345");
        Nihilist{ polybius, vigenere: vigenere_key }
    }

    pub fn encode(&self, text: &str) -> String {
        let poly_pairs = self.polybius.encode(text).chars().collect::<Vec<char>>();
        let mut vkey = self.vigenere.iter().cycle();

        let mut out = "".to_string();

        for p in poly_pairs.chunks(2)
                                  .map(|x| format!("{}{}",x[0],x[1])
                                  .parse::<usize>().unwrap()) {
            out.push_str(&format!("{} ",p+vkey.next().unwrap())) 
        }

        out
    }

    pub fn decode(&self, text: &str) -> String {
        let groups = text.trim_end().split(' ').collect::<Vec<&str>>()
                                   .iter().map(|x| x.parse::<usize>().unwrap())
                                   .collect::<Vec<usize>>();
        let poly_groups = {
            let mut vkey = self.vigenere.iter().cycle();
            let mut s = "".to_string();
            for g in groups {
                s.push_str(&format!("{}",g-vkey.next().unwrap()))
            }
            s
        };
        self.polybius.decode(&poly_groups)
    }
}

impl fmt::Display for Nihilist {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Nihilist Composite Cipher\n{}\nAdditive Key:\n{:?}",self.polybius,self.vigenere)
    }
}


 #[test]
fn nihilist() {

    let nihilist = Nihilist::new("ZEBRAS",vec![20,21,22,23,24]);
    //println!("{}",nihilist);
    let plaintext = "THEQUICKBROWNFOXIUMPSOVERTHELAZYDOG";
    let ciphertext = nihilist.encode(plaintext);
    let cleartext = nihilist.decode(&ciphertext);

    println!("{}\n{}\n{}",plaintext,ciphertext,cleartext);
}