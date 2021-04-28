use std::fmt;
use std::iter::Iterator;
use crate::auxiliary::keyed_alphabet;

pub fn playfair_from_keyword(keyword: &str, alphabet: &str, size: usize, filler: char) -> Playfair {

    if !alphabet.contains(filler) {
        panic!("filler characterbe in the alphabet: {}",alphabet)
    }
    let key_alpha = keyed_alphabet(keyword,alphabet);

    Playfair::new(&key_alpha, size, filler)
}

/// The Playfair Cipher, developed by Charles Wheatstone and promoted at his request by Lord Playfair, is a substitution ciphers that operates on digraphs.
pub struct Playfair {
    alphabet: String,
    size: usize,
    filler: char,
}

// Don't need rows, we can extract the indices by finding the position and some aritmetic
// rectangle: same row, opposite corner
// column: down one step
// row: right one step
impl Playfair {
    pub fn new(alphabet: &str, size: usize, filler: char) -> Playfair {
        let alen = alphabet.chars().count();
        if alen != size*size {
            panic!("an alphabet with {} characters does exactly fit in a {}x{} square.",alen,size,size)
        }

        Playfair{ alphabet: alphabet.to_string(), size, filler }
    }

    fn symbol_to_pair(&self, symbol: char) -> (usize,usize) {
        let num = self.alphabet.chars().position(|x| x == symbol).unwrap();
        (num / self.size, num % self.size)
    }

    fn pair_to_symbol(&self, pair: (usize,usize)) -> char {
        let num = pair.0*self.size + pair.1;
        self.alphabet.chars().nth(num).unwrap()
        
    }

    pub fn encode(&self, text: &str) -> String {
        let mut symbols = text.chars().peekable();
        let mut out = "".to_string();
        loop {
            if symbols.peek().is_none() {
                break
            } else {
                let a = symbols.next().unwrap();
                // If the next symbol would match a then return the filler symbol
                // Otherwise take the next symbol
                let b = {
                    let n = symbols.peek();
                    if n.is_some() && *n.unwrap() != a {
                        symbols.next().unwrap()
                    } else {
                        self.filler
                    }
                };
                let a_pair = self.symbol_to_pair(a);
                let b_pair = self.symbol_to_pair(b);

                let s = self.size+1;

                if a_pair.0 == b_pair.0 {
                    let x = a_pair.0;
                    
                    out.push(self.pair_to_symbol((x, (a_pair.1+s)%self.size )));
                    out.push(self.pair_to_symbol((x, (b_pair.1+s)%self.size )));

                } else if a_pair.1 == b_pair.1 {
                    let y = a_pair.1;
                    
                    out.push(self.pair_to_symbol(( (a_pair.0+s)%self.size , y )));
                    out.push(self.pair_to_symbol(( (b_pair.0+s)%self.size , y )));

                } else {
                    out.push(self.pair_to_symbol((a_pair.0, b_pair.1)));
                    out.push(self.pair_to_symbol((b_pair.0, a_pair.1)));
                }
            }
        }
        out
    }

    pub fn decode(&self, text: &str) -> String {
        if text.chars().count() % 2 == 1 {
            panic!("Valid Playfair ciphertext cannot have and odd number of symbols")
        }
        let mut symbols = text.chars().peekable();
        let mut out = "".to_string();
        loop {
            if symbols.peek().is_none() {
                break
            } else {
                let a = symbols.next().unwrap();
                // If the next symbol would match a then return the filler symbol
                // Otherwise take the next symbol
                let b = {
                    let n = symbols.peek();
                    if n.is_some() && *n.unwrap() != a {
                        symbols.next().unwrap()
                    } else {
                        self.filler
                    }
                };
                let a_pair = self.symbol_to_pair(a);
                let b_pair = self.symbol_to_pair(b);
                let s = self.size-1;

                if a_pair.0 == b_pair.0 {
                    let x = a_pair.0;
                    
                    out.push(self.pair_to_symbol(( x , (a_pair.1+s)%self.size ) ));
                    out.push(self.pair_to_symbol(( x , (b_pair.1+s)%self.size ) ));

                } else if a_pair.1 == b_pair.1 {
                    let y = a_pair.1;
                    
                    out.push(self.pair_to_symbol(( (a_pair.0+s)%self.size , y ) ));
                    out.push(self.pair_to_symbol(( (b_pair.0+s)%self.size , y ) ));

                } else {
                    out.push(self.pair_to_symbol((a_pair.0, b_pair.1)));
                    out.push(self.pair_to_symbol((b_pair.0, a_pair.1)));
                }
            }
        }
        out
    }
}

impl fmt::Display for Playfair {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut square = "Playfair Cipher".to_string();
        for (n, c) in self.alphabet.chars().enumerate() {
            if n % self.size == 0 {
                square.push_str("\n")
            }
            square.push_str(&format!("{} ",c))
        };
        write!(f, "{}", square)
    }
}

#[test]
fn playfair() {
    use crate::auxiliary::LATIN25_J;
    let playfair = playfair_from_keyword("PLAYFAIREXAMPLE",LATIN25_J, 5, 'X');
    println!("{}",playfair);

    let plaintext = "HIDETHEGOLDINTHETREESTUMP";
    let ciphertext = playfair.encode(plaintext);
    let decoded = playfair.decode(&ciphertext);

    println!("{}",plaintext);
    println!("{}",ciphertext);
    println!("{}",decoded);
    
}