use std::fmt;

use itertools::Itertools;

use crate::ciphers::Polybius;

/// The Trifid Cipher uses a Polybius "cube" to convert each character to a three digit string then applies a simple transposition
pub struct Trifid {
    polybius: Polybius,
    block_size: usize,
}


impl Trifid {
    pub fn new(keyword: &str, labels: &str, block_size: usize, alphabet: &str) -> Trifid {
        let polybius = Polybius::new(keyword, alphabet, labels, 3);
        Trifid{ polybius, block_size }
    }

}

impl crate::Cipher for Trifid {

    fn encrypt(&self, text: &str) -> String {
        let tlen = text.chars().count();
        let vector: Vec<char> = text.chars().collect();
        let mut out = String::with_capacity(tlen);


        for clip in vector.chunks(self.block_size).map(|x| x.to_vec().iter().collect::<String>()) {
            let polystring = self.polybius.encrypt(&clip);

            let mut block_a = String::with_capacity(self.block_size);
            let mut block_b = String::with_capacity(self.block_size);
            let mut block_c = String::with_capacity(self.block_size);

            for (pos, s) in polystring.chars().enumerate() {
                match pos % 3 {
                    0 => block_a.push(s),
                    1 => block_b.push(s),
                    2 => block_c.push(s),
                    _ => unreachable!("math is broken")
                }
            }
            block_a.push_str(&block_b);
            block_a.push_str(&block_c);
            out.push_str(&self.polybius.decrypt(&block_a))
        }

        out

    }

    fn decrypt(&self, text: &str) -> String {
        let tlen = text.chars().count();

        let vector: Vec<char> = text.chars().collect();
        let mut out = String::with_capacity(tlen);

        for clip in vector.chunks(self.block_size).map(|x| x.iter().collect::<String>()) {

            let polystring = self.polybius.encrypt(&clip);
            let raw_chunks = polystring.chars().chunks(clip.chars().count()).into_iter().map(|x| x.into_iter().collect::<String>()).collect_vec();

            let block_a = &raw_chunks[0];
            let block_b = &raw_chunks[1];
            let block_c = &raw_chunks[2];

            let mut sorted = String::with_capacity(self.block_size);
            for (a, (b, c)) in block_a.chars().zip(block_b.chars().zip(block_c.chars())) {
                sorted.push(a);
                sorted.push(b);
                sorted.push(c);
            }

            out.push_str(&self.polybius.decrypt(&sorted))
        }

        out
    }

}

impl fmt::Display for Trifid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Trifid Composite Cipher\nBlock Size:\n{}\nPolybius Square:\n{}",self.block_size,self.polybius)
    }
}
