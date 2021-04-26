use std::fmt;
use std::iter::Iterator;


pub struct RailFence {
    key: usize,
}

impl RailFence {
    pub fn new(key: usize) -> RailFence {
        if key < 3 {
            panic!("RailFence key must be greater than 2")
        }
        RailFence{ key }
    }

    pub fn encode(&self, text: &str) -> String {
        let symbols = text.chars();
        let mut rows = Vec::new();
        for _ in 0..self.key {
            rows.push(Vec::<char>::new());
        }

        let mut positions: Vec<usize> = (0..self.key).collect();
        for p in 2..self.key {
            positions.push(self.key-p)
        }

        for (c, n) in symbols.zip(positions.iter().cycle()) {
            rows[*n].push(c)
        }

        let mut out = "".to_string();
        for row in rows {
            for c in row {
                out.push(c)
            }
        }

        out
    }


/*     pub fn decode(&self, text: &str) -> String {

    } */
}

impl fmt::Display for RailFence {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RailFence\nkey: {}",self.key)
    }
}

#[test]
fn fail_fence() {

    let railfence = RailFence::new(3);
    println!("{}",railfence);
    let plaintext = "WEAREDISCOVEREDFLEEATONCE";
    let ciphertext = railfence.encode(plaintext);
    //let cleartext = railfence.decode(&ciphertext);

    println!("{}",ciphertext);
    //println!("{}",cleartext);

}