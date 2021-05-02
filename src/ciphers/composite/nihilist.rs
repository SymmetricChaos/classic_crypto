use std::fmt;

use crate::ciphers::polybius::polybius::Polybius;

/// The Nihilist ciphers is a combination of the Polybius Square and a variation on the Vigenere Cipher. It proceeds by first using a 5x5 Polybius Square to convert each letter to a two digit number then adds the values of a keyword to each value, repeating as necessary.
pub struct Nihilist {
    polybius: Polybius,
    vigenere: Vec<usize>,
}

impl Nihilist {
    pub fn new(polybius_key: &str, vigenere_key: Vec<usize>) -> Nihilist {
        let polybius = Polybius::new(polybius_key, "ABCDFEGHIKLMNOPQRSTUVWXYZ", "12345");
        Nihilist{ polybius, vigenere: vigenere_key }
    }

    pub fn encrypt(&self, text: &str) -> String {
        let poly_pairs = self.polybius.encrypt(text).chars().collect::<Vec<char>>();
        let mut vkey = self.vigenere.iter().cycle();

        let mut out = "".to_string();

        for p in poly_pairs.chunks(2)
                                  .map(|x| format!("{}{}",x[0],x[1])
                                  .parse::<usize>().unwrap()) {
            out.push_str(&format!("{} ",p+vkey.next().unwrap())) 
        }

        out
    }

    pub fn decrypt(&self, text: &str) -> String {
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

impl fmt::Display for Nihilist {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Nihilist Composite Cipher\n{}\nAdditive Key:\n{:?}",self.polybius,self.vigenere)
    }
}


pub struct NihilistCyrillic {
    polybius: Polybius,
    vigenere: Vec<usize>,
}



impl NihilistCyrillic {
    pub fn new(polybius_key: &str, vigenere_key: Vec<usize>) -> NihilistCyrillic {
        let polybius = Polybius::new(polybius_key, "АБВГДЕЁЖЗИЙКЛМНОПРСТУФХЦЧШЩЪЫЬЭЮЯ", "123456");
        NihilistCyrillic{ polybius, vigenere: vigenere_key }
    }

    pub fn encrypt(&self, text: &str) -> String {
        let poly_pairs = self.polybius.encrypt(text).chars().collect::<Vec<char>>();
        let mut vkey = self.vigenere.iter().cycle();

        let mut out = "".to_string();

        for p in poly_pairs.chunks(2)
                                  .map(|x| format!("{}{}",x[0],x[1])
                                  .parse::<usize>().unwrap()) {
            out.push_str(&format!("{} ",p+vkey.next().unwrap())) 
        }

        out
    }

    pub fn decrypt(&self, text: &str) -> String {
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

impl fmt::Display for NihilistCyrillic {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Nihilist Composite Cipher (Cyrillic Version)\n{}\nAdditive Key:\n{:?}",self.polybius,self.vigenere)
    }
}



#[test]
fn nihilist() {

    let nihilist = Nihilist::new("ZEBRAS",vec![20,21,22,23,24]);
    let plaintext = "THEQUICKBROWNFOXIUMPSOVERTHELAZYDOG";
    let ciphertext = nihilist.encrypt(plaintext);
    let cleartext = nihilist.decrypt(&ciphertext);

    println!("{}\n{}\n{}",plaintext,ciphertext,cleartext);

    let nihilist = NihilistCyrillic::new("РОССИЯ",vec![20,21,22,23,24]);
    let plaintext = "АТАКАНАЗИМНИЙДВОРЕЦ";
    let ciphertext = nihilist.encrypt(plaintext);
    let cleartext = nihilist.decrypt(&ciphertext);

    println!("{}\n{}\n{}",plaintext,ciphertext,cleartext);
}