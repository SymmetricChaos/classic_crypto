use std::fmt;
//use num::Integer;


// This will have to be a whole family of ciphers

pub struct Route<'a> {
    dimensions: (usize,usize),
    text_length: usize,
    key: &'a str,
}

impl Route<'_> {
    pub fn new<'a>(dimensions: (usize,usize), key: &'a str) -> Route {
        let text_length = dimensions.0 * dimensions.1;
        Route{ dimensions, text_length, key }
    }

    fn create_grid<'b>(&self, text: &'b str) -> Vec<Vec<char>> {
        let mut grid = Vec::new();
        let mut c = text.chars();
        for _ in 0..self.dimensions.0 {
            let mut v = Vec::new();
            for _ in 0..self.dimensions.1 {
                v.push(c.next().unwrap())
            }
            grid.push(v)
        }
        grid
    }

    fn encrypt_stripe(&self, text: &str) -> String {
        let tlen = text.chars().count();
        let mut out = String::with_capacity(tlen);
        let grid = self.create_grid(text);
        for col in 0..self.dimensions.0 {
            for row in 0..self.dimensions.1 {
                out.push(grid[row][col])
            }
        }
        out
    }

    // This version of the stripe route is reciprocal (and extremely weak)
    fn decrypt_stripe(&self, text: &str) -> String {
        self.encrypt_snake(text)
    }

    fn encrypt_snake(&self, text: &str) -> String {
        let tlen = text.chars().count();
        let mut out = String::with_capacity(tlen);
        let grid = self.create_grid(text);
        let mut updown = 0;
        for col in 0..self.dimensions.0 {
            if updown == 0 {
                for row in 0..self.dimensions.1 {
                    out.push(grid[row][col])
                }
                updown = 1
            } else {
                for row in (0..self.dimensions.1).rev() {
                    out.push(grid[row][col])
                }
                updown = 0
            }
        }
        out
    }

    fn decrypt_snake(&self, text: &str) -> String {
        let tlen = text.chars().count();
        let mut out = String::with_capacity(tlen);
        let grid = self.create_grid(text);
        let mut updown = 0;
        for row in 0..self.dimensions.1 {
            if updown == 0 {
                for col in 0..self.dimensions.0 {
                    out.push(grid[row][col])
                }
                updown = 1
            } else {
                for col in (0..self.dimensions.0).rev() {
                    out.push(grid[row][col])
                }
                updown = 0
            }
        }
        out
    }

}

impl crate::Cipher for Route<'_> {

    fn encrypt(&self, text: &str) -> String {
        let tlen = text.chars().count();
        if tlen != self.text_length {
            panic!("For a route cipher using a {} by {} grid the text must have exactly {} symbols, the text provided has {} symbols",
                   self.dimensions.0, self.dimensions.1, self.text_length, tlen)
        }

        match self.key {
            "stripe" => self.encrypt_stripe(text),
            "snake" => self.encrypt_snake(text),
            _ => panic!("invalid key"),
        }
    }

    fn decrypt(&self, text: &str) -> String {
        let tlen = text.chars().count();
        if tlen != self.text_length {
            panic!("For a route cipher using a {} by {} grid the text must have exactly {} symbols, the text provided has {} symbols",
                   self.dimensions.0, self.dimensions.1, self.text_length, tlen)
        }

        match self.key {
            "stripe" => self.decrypt_stripe(text),
            "snake" => self.decrypt_snake(text),
            _ => panic!("invalid key"),
        }
    }

}

impl fmt::Display for Route<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Route\nkey: {}",self.key)
    }
}