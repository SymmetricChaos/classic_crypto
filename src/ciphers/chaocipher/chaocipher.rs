use std::fmt;
use std::collections::VecDeque;
use std::iter::FromIterator;


pub struct Chaocipher {
    alpha_l: VecDeque<char>,
    alpha_r: VecDeque<char>,
}

impl Chaocipher {
    pub fn new(alphabet_left: &str, alphabet_right: &str) -> Chaocipher {
        let alpha_l = VecDeque::from_iter(alphabet_left.chars());
        let alpha_r = VecDeque::from_iter(alphabet_right.chars());
        Chaocipher{ alpha_l, alpha_r }
    }

    fn permute_l(&mut self, n: usize) {
        self.alpha_l.rotate_left(n);
        let t = self.alpha_l.remove(1).unwrap();
        self.alpha_l.insert(13, t);
    }

    fn permute_r(&mut self, n: usize) {
        self.alpha_r.rotate_left(n)
    }
}

impl crate::Cipher for Chaocipher {

    fn encrypt(&self, text: &str) -> String {

        let mut out = String::new();
        out

    }

    fn decrypt(&self, text: &str) -> String {

        let mut out = String::new();
        out
        
    }

}

/* impl fmt::Display for Chaocipher<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Chaocipher Cipher\nkey: {}",self.key_name)
    }
} */

#[test]
fn test_chaociper() {
    let mut c = Chaocipher::new("HXUCZVAMDSLKPEFJRIGTWOBNYQ","PTLNBQDEOYSFAVZKGJRIHWXUMC");
    c.permute_l(12);
    let l: String = c.alpha_l.iter().collect();
    println!("{}",l);
    println!("{}","PFJRIGTWOBNYQEHXUCZVAMDSLK");
}