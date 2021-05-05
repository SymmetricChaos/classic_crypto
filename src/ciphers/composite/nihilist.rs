use std::fmt;

use crate::ciphers::polybius::polybius::Polybius;

/// The Nihilist ciphers is a combination of the Polybius Square and a variation on the Vigenere Cipher. It proceeds by first using a 5x5 Polybius Square to convert each letter to a two digit number then adds the values of a keyword to each value, repeating as necessary.
pub struct Nihilist {
    polybius: Polybius,
    vigenere: Vec<usize>,
}

impl Nihilist {
    pub fn new(polybius_key: &str, alphabet: &str, labels: &str, vigenere_key: Vec<usize>) -> Nihilist {
        let polybius = Polybius::new(polybius_key, alphabet, labels);
        Nihilist{ polybius, vigenere: vigenere_key }
    }

}

impl crate::auxiliary::Cipher for Nihilist {

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

impl fmt::Display for Nihilist {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Nihilist Composite Cipher\n{}\nAdditive Key:\n{:?}",self.polybius,self.vigenere)
    }
}





#[test]
fn nihilist() {
    use crate::Cipher;
    use crate::alphabets::LATIN25_J;

    let nihilist = Nihilist::new("ZEBRAS",LATIN25_J, "12345", vec![20,21,22,23,24]);
    let plaintext = "THEQUICKBROWNFOXIUMPSOVERTHELAZYDOG";
    let ciphertext = nihilist.encrypt(plaintext);
    let cleartext = nihilist.decrypt(&ciphertext);

    assert_eq!(ciphertext,"65 52 34 67 75 52 43 55 36 38 62 74 63 47 66 74 53 73 58 67 41 63 74 35 38 65 52 34 57 39 31 76 45 65 49");
    assert_eq!(cleartext, "THEQUICKBROWNFOXIUMPSOVERTHELAZYDOG");

/*     let nihilist = NihilistCyrillic::new("РОССИЯ",vec![20,21,22,23,24]);
    let plaintext = "АТАКАНАЗИМНИЙДВОРЕЦ";
    let ciphertext = nihilist.encrypt(plaintext);
    let cleartext = nihilist.decrypt(&ciphertext);

    assert_eq!(ciphertext,"36 64 38 57 40 61 37 54 37 60 61 35 55 47 46 32 32 47 74");
    assert_eq!(cleartext, "АТАКАНАЗИМНИЙДВОРЕЦ"); */
}