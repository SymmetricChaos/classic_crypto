use std::fmt;
use std::char;

pub struct VicCheckerboard {
    numbering: Vec<usize>,
    alphabet: String,
    row1: usize,
    row2: usize,
    digit_mode: bool,
}

// need to handle the digit encoding scheme
impl VicCheckerboard {

    pub fn new(numbering: Vec<u8>) -> VicCheckerboard {
        let numbering: Vec<usize> = numbering.iter().map(|x| *x as usize).collect();
        let row1 = numbering[2];
        let row2 = numbering[6];
        let alphabet= "AT ONE SIRBCDFGHJKLMPQUVWXYZ./".to_string();
        //let alphhabet= "SN EGO PADBCFHIJKLMQRTUVWXYZ./".to_string();
        VicCheckerboard{ numbering, alphabet, row1, row2, digit_mode: false }
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

impl crate::auxiliary::Cipher for VicCheckerboard {
    
    fn encrypt(&self, text: &str) -> String {
        let mut out = String::new();
        for s in text.chars() {
            out.push_str(&self.encrypt_char(s))
        }
        out
    }

    fn decrypt(&self, text: &str) -> String {
        let mut out = String::new();
/*         let mut symbols = text.chars();

        while let Some(c) = symbols.next() {
            let y = c.to_digit(10).unwrap() as usize;

            if y == self.row1 {
                let x = symbols.next().unwrap().to_digit(10).unwrap() as usize;
                out.push(self.rows.chars().nth(x + 10).unwrap())
            
            } else if y == self.row2 {
                let x = symbols.next().unwrap().to_digit(10).unwrap() as usize;
                out.push(self.rows.chars().nth(x + 20).unwrap())
            
            } else {
                out.push(self.rows.chars().nth(y).unwrap())
            }
        } */
        out
    }

}


#[test]
fn vic_checkerboard() {
    use crate::Cipher;
    let vc = VicCheckerboard::new(vec![5,9,6,1,3,2,8,4,7,0]);
    let plaintext = "ATTACKATDAWN";
    let ciphertext = vc.encrypt(plaintext);
    println!("{}",ciphertext);
}