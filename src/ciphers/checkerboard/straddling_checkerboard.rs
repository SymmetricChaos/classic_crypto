use std::fmt;
use std::char;

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
            if self.gaps.0 == y || self.gaps.1 == y {
                let x = symbols.next().unwrap().to_digit(10).unwrap() as usize;
                println!("{}{}",y,x);
                out.push(self.rows.chars().nth(x + y * 10).unwrap())
            } else {
                println!("{}",y);
                out.push(self.rows.chars().nth(y).unwrap())
            }
        }
        out
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

    let checkerboard = StraddlingCheckerboard::new("ETAONRISBCDFGHJKLMPQ/UVWXYZ.",(2,6));

    println!("{}",checkerboard);

    let plaintext = "ATTACKATDAWN";
    let ciphertext = checkerboard.encrypt(plaintext);
    
    println!("\n{}",ciphertext);
    let cleartext = checkerboard.decrypt(&ciphertext);

    //println!("\n{}",cleartext);

    //println!("{}\n{}\n{}",plaintext,ciphertext,cleartext);
}