use std::fmt;


pub struct StraddlingCheckerboard {
    rows: String,
    gaps: (usize,usize),
}

impl StraddlingCheckerboard {
    pub fn new(alphabet: &str, gaps: (usize,usize)) -> StraddlingCheckerboard {
        let mut symbols = alphabet.chars();
        let mut rows = "".to_string();
        for i in 0..30 {
            if gaps.0 == i || gaps.1 == i {
                rows.push(' ')
            } else {
                rows.push(symbols.next().unwrap())
            }
        }

        StraddlingCheckerboard{ rows, gaps }
    }

/*     fn encode_char(&self, c: char) -> String {

    } */
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

        s.push_str(&format!("\n{}",self.gaps.0));
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

    //let plaintext = "THEQUICKBROWNFOXIUMPSOVERTHELAZYDOG";
    //let ciphertext = checkerboard.encrypt(plaintext);
    //let cleartext = checkerboard.decrypt(&ciphertext);


    //println!("{}\n{}\n{}",plaintext,ciphertext,cleartext);
}