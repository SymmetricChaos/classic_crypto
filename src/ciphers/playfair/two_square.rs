use std::fmt;

/// The Two Square cipher is a slightly version of the Playfair Cipher that uses two seperate squares to cover for some weaknesses of the Playfair.
pub struct TwoSquare {
    alphabet1: String,
    alphabet2: String,
    size: usize,
}
impl TwoSquare {
    pub fn new(alphabet1: &str, alphabet2: &str, size: usize) -> TwoSquare {
        if alphabet1.chars().count() > size*size {
            panic!("alphabet1 does not work, it must have {} charactersto fit in a {}x{} square.",size*size,size,size)
        }

        if alphabet2.chars().count() > size*size {
            panic!("alphabet2 does not work, it must have {} charactersto fit in a {}x{} square.",size*size,size,size)
        }

        TwoSquare{ alphabet1: alphabet1.to_string(), alphabet2: alphabet2.to_string(), size }
    }

    fn symbol_to_pair_1(&self, symbol: char) -> (usize,usize) {
        let num = self.alphabet1.chars().position(|x| x == symbol).unwrap();
        (num / self.size, num % self.size)
    }

    fn symbol_to_pair_2(&self, symbol: char) -> (usize,usize) {
        let num = self.alphabet2.chars().position(|x| x == symbol).unwrap();
        (num / self.size, num % self.size)
    }

    fn pair_to_symbol_1(&self, pair: (usize,usize)) -> char {
        let num = pair.0*self.size + pair.1;
        self.alphabet1.chars().nth(num).unwrap()
    }

    fn pair_to_symbol_2(&self, pair: (usize,usize)) -> char {
        let num = pair.0*self.size + pair.1;
        self.alphabet2.chars().nth(num).unwrap()
    }

    pub fn encode(&self, text: &str) -> String {
        let mut symbols = text.chars();
        let mut out = "".to_string();
        loop {

            let a = match symbols.next() {
                Some(s) => s,
                None => break,
            };
            
            let b = match symbols.next() {
                Some(s) => s,
                None => break,
            };


            let a_pair = self.symbol_to_pair_1(a);
            let b_pair = self.symbol_to_pair_2(b);

            if a_pair.1 == b_pair.1 {
                out.push(b);
                out.push(a);
            } else {
                out.push(self.pair_to_symbol_1((a_pair.0, b_pair.1)));
                out.push(self.pair_to_symbol_2((b_pair.0, a_pair.1)));
            }
        }
        out
    }

/*     pub fn decode(&self, text: &str) -> String {
    } */
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


// This does not decrypt properly
#[test]
fn two_square() {
    use crate::auxiliary::{LATIN25_Q,keyed_alphabet};
    let alphabet1 = &keyed_alphabet("EXAMPLE",LATIN25_Q);
    let alphabet2 = &keyed_alphabet("KEYWORD",LATIN25_Q);

    let two_square = TwoSquare::new(alphabet1, alphabet2, 5);

    println!("{}",two_square);

    let plaintext = "HELPMEOBIWANKENOBI";
    let ciphertext = &two_square.encode(plaintext);
    //let decoded = two_square.decode(ciphertext);
    println!("\n\n{}",plaintext);
    println!("{}",ciphertext);
    //println!("{}",decoded);
    
}