use std::{collections::HashMap, fmt};



pub struct Slidefair<'a> {
    /// The Slidefair Cipher, invented by Helen Fouché Gaines, is a cyclic key variation on the Playfair Cipher. Rather than using a single scrambled alphabet for each digraph it uses a different shifted alphabet for each digraph, cycling as needed. It is a reciprocal cipher.
    alphabet: &'a str,
    square: HashMap<char,String>,
    key: &'a str,
}


impl Slidefair<'_> {
    pub fn new<'a>(key: &'a str, alphabet: &'a str) -> Slidefair<'a> {
        let mut square = HashMap::new();
        for (index, symbol) in alphabet.chars().enumerate() {
            if key.contains(symbol) {
                let mut row = alphabet[index..].to_string();
                row.push_str(&alphabet[..index]);
                square.insert(symbol, row);
            }
        }

        Slidefair{ alphabet, square, key }
    }

}

impl crate::auxiliary::Cipher for Slidefair<'_> {
    
    fn encrypt(&self, text: &str) -> String {
        let tlen = text.chars().count();
        if tlen % 2 != 0 {
            panic!("The Slidefair Cipher requires an even number of symbols in the text. Please adjust the input.")
        }
        let mut symbols = text.chars();
        let mut out = String::with_capacity(tlen);
        let mut ckey = self.key.chars().cycle();

        for _ in 0..tlen/2 {
            let a = symbols.next().unwrap();
            let b = symbols.next().unwrap();
            let i = ckey.next().unwrap();
            
            let p = self.alphabet.chars().position(|x| x == a).unwrap();
            let q = self.square[&i].chars().position(|x| x == b).unwrap();

            out.push(self.alphabet.chars().nth(q).unwrap());
            out.push(self.square[&i].chars().nth(p).unwrap());

        }

        out
    }

    fn decrypt(&self, text: &str) -> String {
        self.encrypt(text)
    }

}

impl fmt::Display for Slidefair<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", format!("Slidefair Cipher\nKey:{}\nAlphabet: {}\n",self.key,self.alphabet))
    }
}