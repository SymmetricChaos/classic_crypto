use std::fmt;
use num::Integer;

use crate::auxiliary::rank_str;

/// The Incomplete Columnar Transposition Cipher is the same as the normal Columnar Transposition but relaxes the requirement that the text be a multiple of the key length.

// Given 5,2,1,3,0,4 we want to get 4,2,1,3,5,0
fn inverse_ranks(v: Vec<usize>) -> Vec<usize> {
    let mut out = v.clone();
    for (pos,val) in v.iter().enumerate() {
        out[*val] = pos
    }
    out
}



pub struct IncompleteColumnar<'a> {
    key: Vec<usize>,
    key_name: &'a str,
    key_len: usize,
}

impl IncompleteColumnar<'_> {

    pub fn new<'a>(key: &'a str, alphabet: &'a str) -> IncompleteColumnar<'a> {
        let key_name = key;
        let key = rank_str(key,alphabet);
        let key_len = key.len();
        IncompleteColumnar{ key, key_name, key_len }
    }


}

impl crate::Cipher for IncompleteColumnar<'_> {

    fn encrypt(&self, text: &str) -> String {

        // Use ceiling division to calculate number of rows
        let tlen = text.chars().count();
        let n_rows = tlen.div_ceil(&self.key_len);

        // Create columns
        let mut columns = Vec::new();
        for _ in 0..self.key_len {
            columns.push(Vec::<char>::new());
        }

        // Read the symbols into the columns (if printed out these are rotated and reversed)
        let mut symbols = text.chars();
        for _row in 0..n_rows {
            for col in columns.iter_mut() {
                match symbols.next() {
                    Some(c) => col.push(c),
                    None => break,
                }
            }
        }

        // Read off the columns in the order specified by the key
        let mut out = "".to_string();
        for rank in inverse_ranks(self.key.clone()).iter() {
            let s: String = columns[*rank].iter().collect();
            out.push_str(&s);
        }
        out
    }

    // Decoding is very different
    fn decrypt(&self, text: &str) -> String {

        // Calculate how many spaces in the final row are filled
        let tlen = text.chars().count();
        let filled = tlen % self.key_len;

        // Use ceiling division to calculate number of rows
        let n_rows = tlen.div_ceil(&self.key_len);

        //println!("filled: {}",filled);

        // Create columns
        let mut columns = Vec::new();

        // If the inverse rank is geq the filled spots read one less than the number of rows else read the number of rows
        let mut symbols = text.chars();
        for inv_rank in inverse_ranks(self.key.clone()).iter() {
            
            //println!("{}",inv_rank);
            if inv_rank < &filled {
                
                let mut v = Vec::with_capacity(n_rows);
                for _ in 0..n_rows {
                    v.push(symbols.next().unwrap())
                }
                //println!("{:?}",&v);
                columns.push(v);
            } else {
                let mut v = Vec::with_capacity(n_rows-1);
                for _ in 0..n_rows-1 {
                    v.push(symbols.next().unwrap())
                }
                v.push('\0'); // push a null here to tell us later the space is blank
                //println!("{:?}",&v);
                columns.push(v);
            }
        }


        let mut out = String::new();

        
        for r in 0..n_rows {
            for k in self.key.iter() {
                let c = columns[*k][r];
                if c != '\0' {
                    out.push(c)
                }                
            }
        }

        out
    }

}

impl fmt::Display for IncompleteColumnar<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Incomplete Columnar Cipher\nkey: {:?}",self.key_name)
    }
}

#[test]
fn columnar() {
    use crate::Cipher;
    use crate::alphabets::LATIN26;
    let col = IncompleteColumnar::new("FCBDAE", LATIN26);
    let plaintext = "THEQUICKBROWNFOXJUMPSOVERTHELAZY";
    let ciphertext = col.encrypt(plaintext);
    let decrypted = col.decrypt(&ciphertext);

    assert_eq!(ciphertext,"UOJVLEBOSHHKFPTYQRXOEIWUEATCNMRZ");
    assert_eq!(decrypted,plaintext)
}