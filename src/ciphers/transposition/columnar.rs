use std::fmt;
use num::Integer;

pub fn pad_with_char(text: &str, length: usize, symbol: char) -> String {
    let mut text = text.to_string();
    while text.chars().count() % length != 0 {
        text.push(symbol)
    }
    text
}

/* pub fn word_to_ranks(text: &str) -> Vec<usize> {

} */


// Given 5,2,1,3,0,4 we want to get 4,2,1,3,5,0
fn inverse_ranks(v: Vec<usize>) -> Vec<usize> {
    let mut out = v.clone();
    for (pos,val) in v.iter().enumerate() {
        out[*val] = pos
    }
    out
}


pub struct Columnar {
    key: Vec<usize>,
}

impl Columnar {
    pub fn new(key: Vec<usize>) -> Columnar {
        Columnar{ key }
    }

    pub fn encode(&self, text: &str) -> String {
        let mut columns = Vec::new();
        for _ in 0..self.key.len() {
            columns.push(Vec::<char>::new());
        }
        let mut symbols = text.chars();
        let n_rows = text.len().div_ceil(&self.key.len());
        for _row in 0..n_rows {
            for col in columns.iter_mut() {
                col.push(symbols.next().unwrap_or('X'))
            }
        }
        let mut out = "".to_string();
        for rank in inverse_ranks(self.key.clone()).iter() {
            let s: String = columns[*rank].iter().collect();
            out.push_str(&s);
        }
        out
    }

    // Decoding is very different
    pub fn decode(&self, text: &str) -> String {
        let symbols: Vec<char> = text.chars().collect();
        let n_rows = text.len().div_ceil(&self.key.len());
        let rows: Vec<&[char]> = symbols.chunks(n_rows).collect();
        let mut out = "".to_string();
        for col in 0..n_rows {
            for rank in self.key.iter() {
                out.push(rows[*rank][col])
            }
        }
        out
    }
}

impl fmt::Display for Columnar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Columnar Cipher\nkey: {:?}",self.key)
    }
}

#[test]
fn columnar() {

    let col = Columnar::new(vec![5,2,1,3,0,4]);
    println!("{}",col);
    let plaintext = "WEAREDISCOVEREDFLEEATONCEQKJEU";
    let ciphertext = col.encode(plaintext);
    let decoded = col.decode(&ciphertext);

    println!("{}\n{}\n{}",plaintext,ciphertext,decoded)

}