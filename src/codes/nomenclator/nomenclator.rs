use itertools::Itertools;
use rand::Rng;
use rand::thread_rng;
use rand_xoshiro::rand_core::SeedableRng;
use rand_xoshiro::Xoshiro256StarStar;


struct Codes {
    alphabet: Vec<char>, // alphabet used to construct the code groups
    code_width: usize, // number of symbols in each code group
    alpha_len: usize,
    rng: ThreadRng,
}

impl Codes {
    pub fn new(alphabet: &str, code_width: usize) {
        let alphabet = alphabet.chars().collect_vec();
        let alpha_len = alphabet.len();
        let rng = thread_rng();
        Codes{ alphabet, code_width, alpha_len, rng }
    }
}

impl Iterator for Codes {
    type Item = String;

    fn next(&mut self) -> Option::<Self::Item> {
        for i in 0..self.alpha_len {
            
        }

        Option(self.alphabet[..self.alpha_len].iter().collect::<String>())
    }

}

pub struct Nomenclator<'a> {
    map: HashMap<&'a str, &'a str>,
    map_inv: HashMap<&'a str, &'a str>,
    alphabet: &'a str, // alphabet used to construct the code groups
    code_width: usize, // number of symbols in each code group
    valid: Vec<&'a str>, // code groups that produce output
    null: Vec<&'a str>, // code groups that give no output
    super_null: Vec<&'a str> // code groups that give no output BUT also delete the next group
}

// Needs to include ordinary code groups, nulls, and super nulls that remove the next code group
// use NUL for null and DEL for super null

impl Nomeclator<'_> {
    pub fn random(alphabet: &'a str, code_width: usize) -> Nomenclator {
        let mut rng = thread_rng();
        // Check here to avoid some weird scenario with a large alphabet and wide code groups
        let num_codes = alphabet.chars().count().checked_pow(code_width).unwrap();
        

    }
}
