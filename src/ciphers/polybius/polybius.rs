use std::fmt;
use std::collections::HashMap;
use crate::auxiliary::keyed_alphabet;


// Less memory intensive method?
pub struct Polybius {
    map: HashMap<char,(char,char)>,
    map_inv: HashMap<(char,char),char>,
    square: String,
}

impl Polybius {
    pub fn new(keyword: &str, alphabet: &str, labels: &str) -> Polybius {

        let alpha = keyed_alphabet(keyword,alphabet);

        let alen = alpha.chars().count();
        let llen = labels.chars().count();
        if alen > llen*llen {
            panic!("an alphabet with {} characters does not fit in a {}x{} square.",alen,llen,llen)
        }

        let mut symbols = alpha.chars();
        let mut map = HashMap::new();
        let mut map_inv = HashMap::new();
        for a in labels.chars() {
            for b in labels.chars() {
                let t = (a,b);
                let c = symbols.next();
                if c.is_some() {
                    map.insert(c.unwrap(), t);
                    map_inv.insert(t, c.unwrap());
                }
            }
        }

        let mut square = " ".to_string();
        for c in labels.chars() {
            square.push(' ');
            square.push(c)
        }
        square.push_str("\n");
        let mut symbols = alpha.chars();
        for l in labels.chars() {
            square.push(l);
            for _ in 0..llen {
                square.push(' ');
                square.push(symbols.next().unwrap_or(' '))
            }
            square.push_str("\n");
        }

        Polybius{ map, map_inv, square }
    }

    pub fn encode(&self, text: &str) -> String {
        let mut out = "".to_string();
        for c in text.chars() {
            let (a,b) = self.map[&c];
            let s = format!("{}{}",a,b);
            out.push_str(&s);
        }
        out
    }

    pub fn decode(&self, text: &str) -> String {
        let mut out = "".to_string();
        let tlen = text.chars().count();
        if tlen % 2 == 1 {
            panic!("Polybius Square Error: cannot decode a string with an odd number of characters")
        }
        let n_groups = tlen / 2;
        let mut symbols = text.chars();
        for _ in 0..n_groups {
            let t = (symbols.next().unwrap(),symbols.next().unwrap());
            out.push(self.map_inv[&t]);
        }
        out
    }
}

impl fmt::Display for Polybius {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Polybius Square\n{}",self.square)
    }
}


#[test]
fn polybius() {
    use crate::auxiliary::LATIN36;

    let poly = Polybius::new("17ZEBRAS42",LATIN36,"123456");
    println!("{}",poly);
    let plaintext = "THEQUICKBROWNFOXJUMPSOVERTHELAZYDOG";
    let ciphertext = poly.encode(plaintext);
    let cleartext = poly.decode(&ciphertext);

    println!("{}\n{}\n{}",plaintext,ciphertext,cleartext);
}