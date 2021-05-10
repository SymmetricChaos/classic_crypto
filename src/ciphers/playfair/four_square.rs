use std::fmt;
use crate::alphabets::keyed_alphabet;

/// The Two Square cipher is a slightly version of the Playfair Cipher that uses two seperate squares to cover for some weaknesses of the Playfair.
pub struct FourSquare {
    alphabet1: String,
    alphabet2: String,
    alphabet: String,
    size: usize,
}

impl FourSquare {
    pub fn new(key1: &str, key2: &str, alphabet: &str, size: usize) -> FourSquare {
        if alphabet.chars().count() > size*size {
            panic!("alphabet does not work, it must have exactly {} charactersto fit in a {}x{} square.",size*size,size,size)
        }
        
        FourSquare{ alphabet1: keyed_alphabet(key1,alphabet), 
                    alphabet2: keyed_alphabet(key2,alphabet), 
                    alphabet: alphabet.to_string(), 
                    size }
    }

    fn symbol_to_pair(&self, symbol: char, alphabet: &str) -> (usize,usize) {
        let num = alphabet.chars().position(|x| x == symbol).unwrap();
        (num % self.size, num / self.size)
    }

    fn pair_to_symbol(&self, pair: (usize,usize), alphabet: &str) -> char {
        let num = pair.1*self.size + pair.0;
        alphabet.chars().nth(num).unwrap()
    }


}

impl crate::auxiliary::Cipher for FourSquare {

    fn encrypt(&self, text: &str) -> String {
        let tlen = text.chars().count();
        if tlen % 2 != 0 {
            panic!("The Two Square Cipher requires an even number of symbols in the text. Please adjust the input.")
        }
        let mut symbols = text.chars();
        let mut out = String::with_capacity(tlen);

        for _ in 0..tlen/2 {
            let a = symbols.next().unwrap();
            let b = symbols.next().unwrap();

            let a_pair = self.symbol_to_pair(a, &self.alphabet);
            let b_pair = self.symbol_to_pair(b, &self.alphabet);

            out.push(self.pair_to_symbol((b_pair.0, a_pair.1), &self.alphabet1 ));
            out.push(self.pair_to_symbol((a_pair.0, b_pair.1), &self.alphabet2 ));
        }
        out
    }

    fn decrypt(&self, text: &str) -> String {
        let tlen = text.chars().count();
        if tlen % 2 != 0 {
            panic!("The Two Square Cipher requires an even number of symbols in the text. Please adjust the input.")
        }
        let mut symbols = text.chars();
        let mut out = String::with_capacity(tlen);

        for _ in 0..tlen/2 {
            let a = symbols.next().unwrap();
            let b = symbols.next().unwrap();

            let a_pair = self.symbol_to_pair(a, &self.alphabet1);
            let b_pair = self.symbol_to_pair(b, &self.alphabet2);

            out.push(self.pair_to_symbol((b_pair.0, a_pair.1), &self.alphabet ));
            out.push(self.pair_to_symbol((a_pair.0, b_pair.1), &self.alphabet ));
        }
        out
    }

}

impl fmt::Display for FourSquare {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "making the display for this is kind of a pain")
    }
}

/* impl fmt::Display for FourSquare {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut square = "Two Square Cipher".to_string();
        for (n, c) in self.alphabet1.chars().enumerate() {
            if n % self.size == 0 {
                square.push_str("\n")
            }
            square.push_str(&format!("{} ",c))
        };
        square.push_str("\n");
        for (n, c) in self.alphabet2.chars().enumerate() {
            if n % self.size == 0 {
                square.push_str("\n")
            }
            square.push_str(&format!("{} ",c))
        };
        write!(f, "{}", square)
    }
} */


