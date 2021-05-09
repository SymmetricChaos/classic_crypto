use std::{collections::VecDeque, fmt, fs::File, io::{Error, Read}};

fn beaufort_encrypt(n: usize, k: usize, l: usize) -> usize {
    (l + k - n) % l
}

pub struct Beaufort<'a> {
    key_vals: Vec<usize>,
    key: &'a str,
    alphabet: &'a str,
    length: usize,
}

impl Beaufort<'_> {
    pub fn new<'a>(key: &'a str, alphabet: &'a str) -> Beaufort<'a> {
        let key_vals: Vec<usize> = key.chars().map(|x| alphabet.chars().position(|c| c == x).unwrap()).collect();
        Beaufort{ key, key_vals, alphabet, length: alphabet.chars().count() }
    }
}

impl crate::Cipher for Beaufort<'_> {

    fn encrypt(&self, text: &str) -> String {
        let nums: Vec<usize> = text.chars().map( |x| self.alphabet.chars().position(|c| c == x).unwrap() ).collect();
        let ckey = self.key_vals.iter().cycle();
        let mut out = "".to_string();
        for (n,k) in nums.iter().zip(ckey) {
            let m = beaufort_encrypt(*n,*k,self.length);
            out.push(self.alphabet.chars().nth( m ).unwrap() )
        }
        out
    }

    // The Beaufort cipher is involutive
    fn decrypt(&self, text: &str) -> String {
        self.encrypt(text)
    }

}

impl fmt::Display for Beaufort<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Beaufort Cipher\nkey: {:?}",self.key)
    }
}





pub struct BeaufortAutokey<'a> {
    key_vals: Vec<usize>,
    key: &'a str,
    alphabet: &'a str,
    length: usize,
}

impl BeaufortAutokey<'_> {
    pub fn new<'a>(key: &'a str, alphabet: &'a str) -> BeaufortAutokey<'a> {
        let key_vals: Vec<usize> = key.chars().map( |x| alphabet.chars().position(|c| c == x).unwrap() ).collect();
        BeaufortAutokey{ key, key_vals, alphabet, length: alphabet.chars().count() }
    }
}

impl crate::auxiliary::Cipher for BeaufortAutokey<'_> {

    fn encrypt(&self, text: &str) -> String {
        let mut out = "".to_string();
        let mut akey: VecDeque<usize> = self.key_vals.clone().into_iter().collect();
        let nums: Vec<usize> = text.chars().map( |x| self.alphabet.chars().position(|c| c == x).unwrap() ).collect();
        for n in nums {
            akey.push_back(n);
            let k = akey.pop_front().unwrap();
            let m = beaufort_encrypt(n,k,self.length);
            out.push(self.alphabet.chars().nth( m ).unwrap() )
        }
        out
    }

    // The BeaufortAutokey is not involutive like the Beaufort
    fn decrypt(&self, text: &str) -> String {
        let mut out = "".to_string();
        let mut akey: VecDeque<usize> = self.key_vals.clone().into_iter().collect();
        let nums: Vec<usize> = text.chars().map( |x| self.alphabet.chars().position(|c| c == x).unwrap() ).collect();
        for n in nums {
            let k = akey.pop_front().unwrap();
            let m = beaufort_encrypt(n,k,self.length);
            akey.push_back(m);
            out.push(self.alphabet.chars().nth( m ).unwrap() )
        }
        out
    }

}

impl fmt::Display for BeaufortAutokey<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Beaufort Autokey Cipher\nkey: {:?}",self.key)
    }
}




pub struct BeaufortRunningKey<'a> {
    key_file: &'a str,
    alphabet: &'a str,
    length: usize,
}


impl BeaufortRunningKey<'_> {
    pub fn new<'a>(key_file: &'a str, alphabet: &'a str) -> BeaufortRunningKey<'a> {
        BeaufortRunningKey{ key_file, alphabet, length: alphabet.chars().count() }
    }

    // This is very inefficient 
    fn encrypt(&self, text: &str) -> Result<String,Error> {
        let mut key_text = String::new();
        let mut source = File::open(self.key_file)?;
        source.read_to_string(&mut key_text)?;
        let key: Vec<usize> = key_text.chars().map( |x| self.alphabet.chars().position(|c| c == x).unwrap() ).collect();
        let nums: Vec<usize> = text.chars().map( |x| self.alphabet.chars().position(|c| c == x).unwrap() ).collect();

        let mut out = String::new();

        for (n,k) in nums.iter().zip(key) {
            out.push(self.alphabet.chars().nth( (self.length+k-n)%self.length ).unwrap() )
        }
        Ok(out)
    }

    // The running key is an involution
    fn decrypt(&self, text: &str) -> Result<String,Error> {
        self.encrypt(text)
    }

}

impl fmt::Display for BeaufortRunningKey<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Beaufort Running Key Cipher\nkey file: {:?}",self.key_file)
    }
}






#[test]
fn beaufort() {
    use crate::Cipher;
    use crate::alphabets::LATIN26;
    let beau = Beaufort::new("SECRET", LATIN26);
    let plaintext = "THEQUICKBROWNFOXJUMPSOVERTHELAZYDOG";
    let ciphertext = beau.encrypt(plaintext);
    let decrypted = beau.decrypt(&ciphertext);

    println!("{}\n{}\n{}",plaintext,ciphertext,decrypted);
    
}



#[test]
fn beaufort_autokey() {
    use crate::Cipher;
    use crate::alphabets::LATIN26;
    let auto = BeaufortAutokey::new("SECRET", LATIN26);
    let plaintext = "THEQUICKBROWNFOXJUMPSOVERTHELAZYDOG";
    let ciphertext = auto.encrypt(plaintext);
    let decrypted = auto.decrypt(&ciphertext);

    println!("{}\n{}\n{}",plaintext,ciphertext,decrypted);
    
}



#[test]
fn beaufort_running_key() -> Result<(),Error> {
    use crate::alphabets::LATIN26;
    let auto = BeaufortRunningKey::new("dickens.txt", LATIN26);
    let plaintext = "THEQUICKBROWNFOXJUMPSOVERTHELAZYDOG";
    let ciphertext = auto.encrypt(plaintext)?;
    let decrypted = auto.decrypt(&ciphertext)?;

    println!("{}\n{}\n{}",plaintext,ciphertext,decrypted);
    
    Ok(())
}