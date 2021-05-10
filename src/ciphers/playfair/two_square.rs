use std::fmt;
use crate::auxiliary::keyed_alphabet;


pub struct TwoSquare {
    /// The Two Square cipher is an improvement on the Playfair that uses two alphabetic squares to avoid some of the weaknesses of the Playfair Cipher. This simple implementation is reciprocal, encryption and decryption are identical, however there are many cases in which a digraph may encrypt as itself these transparencies in the text are a potentially serious weakness.
    alphabet1: String,
    alphabet2: String,
    size: usize,
}

impl TwoSquare {
    pub fn new(key1: &str, key2: &str, alphabet: &str, size: usize) -> TwoSquare {
        if alphabet.chars().count() > size*size {
            panic!("alphabet does not work, it must have exactly {} characters to fit in a {}x{} square.",size*size,size,size)
        }
        
        TwoSquare{ alphabet1: keyed_alphabet(key1,alphabet), 
                   alphabet2: keyed_alphabet(key2,alphabet), 
                   size }
    }

    fn symbol_to_pair_1(&self, symbol: char) -> (usize,usize) {
        let num = self.alphabet1.chars().position(|x| x == symbol).unwrap();
        (num % self.size, num / self.size)
    }

    fn symbol_to_pair_2(&self, symbol: char) -> (usize,usize) {
        let num = self.alphabet2.chars().position(|x| x == symbol).unwrap();
        (num % self.size, num / self.size)
    }

    fn pair_to_symbol_1(&self, pair: (usize,usize)) -> char {
        let num = pair.1*self.size + pair.0;
        self.alphabet1.chars().nth(num).unwrap()
    }

    fn pair_to_symbol_2(&self, pair: (usize,usize)) -> char {
        let num = pair.1*self.size + pair.0;
        self.alphabet2.chars().nth(num).unwrap()
    }

}

impl crate::Cipher for TwoSquare {

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

            let a_pair = self.symbol_to_pair_1(a);
            let b_pair = self.symbol_to_pair_2(b);

            out.push(self.pair_to_symbol_1((a_pair.0, b_pair.1)));
            out.push(self.pair_to_symbol_2((b_pair.0, a_pair.1)));
        }
        out
    }

    fn decrypt(&self, text: &str) -> String {
        self.encrypt(text)
    }

}

impl fmt::Display for TwoSquare {
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
}





pub struct TwoSquareInverting {
    alphabet1: String,
    alphabet2: String,
    size: usize,
}

impl TwoSquareInverting {
    // The Two Square Inverting Cipher is a slightly more secure version of the ordinary one. After each digraph is encrtpyed the order of the letters is flipped. This largely removes transparencies but does remove the reciprocal property as during decryption the digraph is flipped and then decoded.
    pub fn new(key1: &str, key2: &str, alphabet: &str, size: usize) -> TwoSquareInverting {
        if alphabet.chars().count() > size*size {
            panic!("alphabet does not work, it must have exactly {} charactersto fit in a {}x{} square.",size*size,size,size)
        }
        
        TwoSquareInverting{ alphabet1: keyed_alphabet(key1,alphabet), 
                            alphabet2: keyed_alphabet(key2,alphabet), 
                            size }
    }

    fn symbol_to_pair_1(&self, symbol: char) -> (usize,usize) {
        let num = self.alphabet1.chars().position(|x| x == symbol).unwrap();
        (num % self.size, num / self.size)
    }

    fn symbol_to_pair_2(&self, symbol: char) -> (usize,usize) {
        let num = self.alphabet2.chars().position(|x| x == symbol).unwrap();
        (num % self.size, num / self.size)
    }

    fn pair_to_symbol_1(&self, pair: (usize,usize)) -> char {
        let num = pair.1*self.size + pair.0;
        self.alphabet1.chars().nth(num).unwrap()
    }

    fn pair_to_symbol_2(&self, pair: (usize,usize)) -> char {
        let num = pair.1*self.size + pair.0;
        self.alphabet2.chars().nth(num).unwrap()
    }

}

impl crate::Cipher for TwoSquareInverting {

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

            let a_pair = self.symbol_to_pair_1(a);
            let b_pair = self.symbol_to_pair_2(b);
            
            out.push(self.pair_to_symbol_2((b_pair.0, a_pair.1)));
            out.push(self.pair_to_symbol_1((a_pair.0, b_pair.1)));
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
            let b = symbols.next().unwrap();
            let a = symbols.next().unwrap();

            let a_pair = self.symbol_to_pair_1(a);
            let b_pair = self.symbol_to_pair_2(b);
            
            out.push(self.pair_to_symbol_1((a_pair.0, b_pair.1)));
            out.push(self.pair_to_symbol_2((b_pair.0, a_pair.1)));
        }
        out
    }

}

impl fmt::Display for TwoSquareInverting {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut square = "Inverting Two Square Cipher".to_string();
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
}