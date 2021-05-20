use crate::auxiliary::PrimeSieve;


pub struct Godel<'a> {
    alphabet: &'a str,
}

impl Godel<'_> {

    pub fn new<'a>(alphabet: &'a str) -> Godel {

        Godel{ alphabet }
    }

}


impl crate::Code for Godel<'_> {

    fn encode(&self, text: &str) -> String {
        let mut out = 1usize;
        let mut pset = PrimeSieve::new();
        let alpha_vec: Vec<char> = self.alphabet.chars().collect();

        for t in text.chars() {
            let n = alpha_vec.iter().position(|x| *x == t).unwrap() + 1;
            out = out * pset.next().unwrap().pow(n as u32)
            
        }

        format!("{}",out)
    }

    fn decode(&self, text: &str) -> String {
        let mut out = String::new();

        out
    }

    fn char_map(&self) -> String {
        let mut out = String::new();

        out
    }
}