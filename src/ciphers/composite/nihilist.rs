use std::fmt;
use num::integer::Roots;

use crate::{Cipher, ciphers::polybius::Polybius};

/// The Nihilist ciphers is a combination of the Polybius Square and a variation on the Vigenere Cipher. It proceeds by first using a 5x5 Polybius Square to convert each letter to a two digit number then adds the values of a keyword to each value, repeating as necessary.
pub struct Nihilist<'a> {
    polybius: Polybius<'a>,
    vigenere: Vec<usize>,
    vigenere_key_name: &'a str,
}

impl Nihilist<'_> {
    pub fn new<'a>(polybius_key: &'a str, vigenere_key: &'a str, alphabet: &'a str) -> Nihilist<'a> {
        let alpha_len = alphabet.chars().count();
        let side_len = alpha_len.sqrt();
        if side_len*side_len != alpha_len {
            panic!("Alphabets must have a square number of characters")
        }
        if side_len > 9 {
            panic!("The standard Nihilist Cipher only accepts alphabets of up to 81 symbols")
        }
        let labels = &"123456789"[..side_len];
        let polybius = Polybius::new(polybius_key, alphabet, labels);

        let inter: Vec<char> = polybius.encrypt(vigenere_key).chars().collect();
        let vigenere: Vec<usize> = inter.chunks(2)
                                        .map(|x| format!("{}{}",x[0],x[1])
                                        .parse::<usize>().unwrap()).collect();
        Nihilist{ polybius, vigenere, vigenere_key_name: vigenere_key }
    }

}

impl crate::Cipher for Nihilist<'_> {

    fn encrypt(&self, text: &str) -> String {
        let poly_pairs = self.polybius.encrypt(text).chars().collect::<Vec<char>>();
        let mut vkey = self.vigenere.iter().cycle();

        let mut out = Vec::<String>::new();

        for p in poly_pairs.chunks(2)
                                  .map(|x| format!("{}{}",x[0],x[1])
                                  .parse::<usize>().unwrap()) {
            out.push(format!("{}",p+vkey.next().unwrap()))
        }

        out.join(" ")
    }

    fn decrypt(&self, text: &str) -> String {
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
        self.polybius.decrypt(&poly_groups)
    }

}

impl fmt::Display for Nihilist<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Nihilist Composite Cipher\n{}\nAdditive Key:\n{:?}",self.polybius,self.vigenere_key_name)
    }
}