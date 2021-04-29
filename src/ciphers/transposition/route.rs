use std::fmt;
//use num::Integer;


// This will have to be a whole family of ciphers

pub struct Route {
    key: usize,
}

impl Route {
    pub fn new(key: usize) -> Route {
        Route{ key }
    }

/*     pub fn encrypt(&self, text: &str) -> String {
        let n_cols = text.len().div_ceil(&self.key);
        let symbols = text.chars();
        let mut rows = Vec::new();

        Ok("".to_string())
    } */

/*     pub fn decrypt(&self, text: &str) -> String {

    } */
}

impl fmt::Display for Route {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Route\nkey: {}",self.key)
    }
}

/* #[test]
fn route() {

    let Route = Route::new(3);
    println!("Route Cipher: {}",Route);
    let plaintext = "WEAREDISCOVEREDFLEEATONCEQKJEU";
    let ciphertext = Route.encrypt(plaintext);

} */