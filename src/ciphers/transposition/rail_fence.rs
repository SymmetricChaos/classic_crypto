use std::fmt;
use std::iter::Iterator;
use std::{fs::File, io::{Error, Read, Write}};


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


}

impl crate::Cipher for RailFence {

    fn encrypt(&self, text: &str) -> String {
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

    // There's probably an easier way to do this.
    fn decrypt(&self, text: &str) -> String {
        
        // Count how many letters must be on each rail
        let mut chunks = vec![0usize; self.key];
        let mut rail = 0;
        let mut down = true;

        for _ in text.chars() {
            chunks[rail] += 1;
            match down {
                true => rail += 1,
                false => rail -= 1,
            }
            if rail == 0 || rail == self.key-1 {
                down = !down
            }
        }

        // Rebuild the rails
        let mut fence= Vec::new();
        let mut ctr = 0;
        for num in chunks {
            fence.push(text[ctr..ctr+num].chars());
            ctr += num
        }

        // Read up and down the rails
        let mut rail = 0;
        let mut down = true;
        let mut out = String::with_capacity(text.chars().count());

        for _ in text.chars() {
            let c = fence[rail].next();
            match c {
                Some(symbol) => out.push(symbol),
                None => break
            }
            match down {
                true => rail += 1,
                false => rail -= 1,
            }
            if rail == 0 || rail == self.key-1 {
                down = !down
            }
        }

        out
    }

    fn encrypt_file(&self, source: &str, target: &str) -> Result<(),Error> {

        let mut target_file = File::create(target.to_string())?;
    
        let mut source_file = File::open(source)?;
        let mut source_text = String::new();
        source_file.read_to_string(&mut source_text)?;
    
        let ciphertext = self.encrypt(&source_text);
    
        target_file.write(ciphertext.as_bytes())?;

        Ok(())
    }

    fn decrypt_file(&self, source: &str, target: &str) -> Result<(),Error> {

        let mut target_file = File::create(target.to_string())?;
    
        let mut source_file = File::open(source)?;
        let mut source_text = String::new();
        source_file.read_to_string(&mut source_text)?;
    
        let ciphertext = self.decrypt(&source_text);
    
        target_file.write(ciphertext.as_bytes())?;

        Ok(())
    }

}


impl fmt::Display for RailFence {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RailFence\nkey: {}",self.key)
    }
}