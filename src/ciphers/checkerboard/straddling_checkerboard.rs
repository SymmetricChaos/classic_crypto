use std::fmt;
use std::char;
use std::{fs::File, io::{Error, Read, Write}};

pub struct StraddlingCheckerboard {
    rows: String,
    gaps: (usize,usize),
}

// need to handle the digit encoding scheme
impl StraddlingCheckerboard {

    pub fn new(alphabet: &str, gaps: (usize,usize)) -> StraddlingCheckerboard {
        
        let v: Vec<char> = alphabet.chars().collect();
        if !v.contains(&'/') {
            panic!("the symbol `/` must be present in the alphabet as it is a special character used for encoding digits")
        }
        if v.len() != 28 {
            panic!("alphabet must have exactly 28 characters")
        }

        let mut rows = "".to_string();
        let mut symbols = v.iter();
        for i in 0..30 {
            if gaps.0 == i || gaps.1 == i {
                rows.push(' ')
            } else {
                rows.push(*symbols.next().unwrap())
            }
        }

        StraddlingCheckerboard{ rows, gaps }
    }

    fn encrypt_char(&self, symbol: char) -> String {
        let num = self.rows.chars().position(|x| x == symbol).unwrap();
        let b = num % 10;
        match num / 10 {
            0 => format!("{}",b),
            1 => format!("{}{}",self.gaps.0,b),
            2 => format!("{}{}",self.gaps.1,b),
            _ => panic!("row greater than 2")
        }
    }

}

impl crate::Cipher for StraddlingCheckerboard {
    
    fn encrypt(&self, text: &str) -> String {
        let mut out = String::new();
        for s in text.chars() {
            out.push_str(&self.encrypt_char(s))
        }
        out
    }

    fn decrypt(&self, text: &str) -> String {
        let mut out = String::new();
        let mut symbols = text.chars();

        while let Some(c) = symbols.next() {
            let y = c.to_digit(10).unwrap() as usize;

            if y == self.gaps.0 {
                let x = symbols.next().unwrap().to_digit(10).unwrap() as usize;
                out.push(self.rows.chars().nth(x + 10).unwrap())
            
            } else if y == self.gaps.1 {
                let x = symbols.next().unwrap().to_digit(10).unwrap() as usize;
                out.push(self.rows.chars().nth(x + 20).unwrap())
            
            } else {
                out.push(self.rows.chars().nth(y).unwrap())
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

impl fmt::Display for StraddlingCheckerboard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = "Straddling Checkerboard Cipher\n  0 1 2 3 4 5 6 7 8 9\n ".to_string();
        let mut symbols = self.rows.chars();

        for _ in 0..10 {
            s.push(' ');
            s.push(symbols.next().unwrap())
        }

        s.push_str(&format!("\n{}",self.gaps.0));
        for _ in 0..10 {
            s.push(' ');
            s.push(symbols.next().unwrap())
        }

        s.push_str(&format!("\n{}",self.gaps.1));
        for _ in 0..10 {
            s.push(' ');
            s.push(symbols.next().unwrap())
        }

        write!(f, "{}", s)
    }
}


#[test]
fn checkerboard() {
    use crate::Cipher;

    let checkerboard = StraddlingCheckerboard::new("ETAONRISBCDFGHJKLMPQ/UVWXYZ.",(2,6));

    let plaintext = "ATTACKATDAWN";
    let ciphertext = checkerboard.encrypt(plaintext);
    assert_eq!(ciphertext,"3113212731223655");

    let cleartext = checkerboard.decrypt(&ciphertext);
    assert_eq!(cleartext,"ATTACKATDAWN");
}