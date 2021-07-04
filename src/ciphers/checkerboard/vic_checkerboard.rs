use std::fmt;
use std::char;
use std::{fs::File, io::{Error, Read, Write}};

pub struct VicCheckerboard {
    numbering: Vec<usize>,
    alphabet: String,
    row1: usize,
    row2: usize,
}

// need to handle the digit encoding scheme
impl VicCheckerboard {

    pub fn new(numbering: Vec<u8>) -> VicCheckerboard {
        let numbering: Vec<usize> = numbering.iter().map(|x| *x as usize).collect();
        let row1 = numbering[2];
        let row2 = numbering[6];
        let alphabet= "AT ONE SIRBCDFGHJKLMPQUVWXYZ./".to_string();
        //let alphhabet= "SN EGO PADBCFHIJKLMQRTUVWXYZ./".to_string();
        VicCheckerboard{ numbering, alphabet, row1, row2 }
    }

    // need to research encryption of numbers
    fn encrypt_char(&self, symbol: char) -> String {
        let n = self.alphabet.chars().position(|x| x == symbol).unwrap();
        let x_pos = self.numbering[n % 10];
        match n / 10 {
            0 => format!("{}",x_pos),
            1 => format!("{}{}",self.row1,x_pos),
            2 => format!("{}{}",self.row2,x_pos),
            _ => panic!("row greater than 2")
        }
    }

}

impl crate::Cipher for VicCheckerboard {
    
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
            let y_pos = self.numbering.iter().position(|x| x == &y).unwrap();

            if y == self.row1 {
                let x = symbols.next().unwrap().to_digit(10).unwrap() as usize;
                let x_pos = self.numbering.iter().position(|a| a == &x).unwrap();
                out.push(self.alphabet.chars().nth(x_pos + 10).unwrap())
            
            } else if y == self.row2 {
                let x = symbols.next().unwrap().to_digit(10).unwrap() as usize;
                let x_pos = self.numbering.iter().position(|a| a == &x).unwrap();
                out.push(self.alphabet.chars().nth(x_pos + 20).unwrap())
            
            } else {
                out.push(self.alphabet.chars().nth(y_pos).unwrap())
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

impl fmt::Display for VicCheckerboard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = "VIC Checkerboard Cipher\n ".to_string();
        for n in self.numbering.iter() {
            s.push_str( &format!(" {}",n) )
        }
        s.push_str("\n ");
        let mut symbols = self.alphabet.chars();

        for _ in 0..10 {
            s.push(' ');
            s.push(symbols.next().unwrap())
        }

        s.push_str(&format!("\n{}",self.row1));
        for _ in 0..10 {
            s.push(' ');
            s.push(symbols.next().unwrap())
        }

        s.push_str(&format!("\n{}",self.row2));
        for _ in 0..10 {
            s.push(' ');
            s.push(symbols.next().unwrap())
        }

        write!(f, "{}", s)
    }
}


#[test]
fn vic_checkerboard() {
    use crate::Cipher;
    let vc = VicCheckerboard::new(vec![5,9,6,1,3,2,8,4,7,0]);
    let plaintext = "ATTACKATDAWN";
    let ciphertext = vc.encrypt(plaintext);
    let decrypted = vc.decrypt(&ciphertext);

    assert_eq!(ciphertext,"5995696459665833");
    assert_eq!(decrypted,"ATTACKATDAWN");
}