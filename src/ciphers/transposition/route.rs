use std::fmt;

use itertools::Itertools;

use crate::grid::Grid;

pub struct Route<'a> {
    dimensions: (usize,usize),
    text_length: usize,
    route_type: &'a str,
    key: Vec<usize>,
}

impl Route<'_> {
    pub fn new<'a>(dimensions: (usize,usize), route_type: &'a str, key: Vec<usize>) -> Route {
        if key.len() != dimensions.1 {
            panic!("key must have as many values as the grid has columns")
        }
        let text_length = dimensions.0 * dimensions.1;
        Route{ dimensions, text_length, route_type, key }
    }

    // This is equivalent to columnar transposition
    fn encrypt_stripe(&self, text: &str) -> String {
        // Read the data in row by row
        let g = Grid::new(text, self.dimensions.0, self.dimensions.1);

        let mut out = String::with_capacity(text.len());
        // Then read it out by columns according to the key
        for k in self.key.iter() {
            let s: String = g.read_col_n(*k).iter().collect();
            out.push_str(&s);
        }
        out = out.replace('\0', "");
        out = out.replace('\u{1f}', "");
        out
    }

    // This is equivalent to columnar transposition
    fn decrypt_stripe(&self, text: &str) -> String {
        let mut g = Grid::new_empty(self.dimensions.0, self.dimensions.1);
        let columns = text.chars().collect_vec();

        // Read the text in by columns according to the key
        for (k, col) in self.key.iter().zip( columns.chunks_exact(self.dimensions.0) ) {
            let c: String = col.iter().collect();
            g.write_col_n(*k, &c );
        }

        // Read it off row by row
        let mut out = String::with_capacity(text.len());
        for i in 0..self.dimensions.0 {
            let s: String = g.read_row_n(i).iter().collect();
            out.push_str(&s)
        }
        out = out.replace('\0', "");
        out = out.replace('\u{1f}', "");
        out
    }

    fn encrypt_snake(&self, text: &str) -> String {
        let mut g = Grid::new_empty( self.dimensions.0, self.dimensions.1);

        // Read the text into the rows reversing the order each time
        let rows = text.chars().collect_vec();
        for (n, row) in rows.chunks_exact(self.dimensions.1).enumerate() {
            if n % 2 == 0 {
                let r: String = row.iter().collect();
                g.write_row_n(n, &r);

            } else {
                let r: String = row.iter().rev().collect();
                g.write_row_n(n, &r);
            }
        }
        
        // Then read it out by columns according to the key
        let mut out = String::with_capacity(text.len());
        for k in self.key.iter() {
            let s: String = g.read_col_n(*k).iter().collect();
            out.push_str(&s);
        }
        out = out.replace('\0', "");
        out = out.replace('\u{1f}', "");
        out
    }

    // This is equivalent to columnar transposition
    fn decrypt_snake(&self, text: &str) -> String {
        let mut g = Grid::new_empty(self.dimensions.0, self.dimensions.1);
        let columns = text.chars().collect_vec();

        // Read the text in by columns according to the key
        for (k, col) in self.key.iter().zip( columns.chunks_exact(self.dimensions.0) ) {
            let c: String = col.iter().collect();
            g.write_col_n(*k, &c );
        }

        // Read it of row by row reversing the order each time
        let mut out = String::with_capacity(text.len());
        for n in 0..self.dimensions.0 {
            if n % 2 == 0{
                let s: String = g.read_row_n(n).iter().collect();
                out.push_str(&s)
            } else {
                let s: String = g.read_row_n(n).iter().rev().collect();
                out.push_str(&s)
            }

        }
        out = out.replace('\0', "");
        out = out.replace('\u{1f}', "");
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

        match self.route_type {
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

        match self.route_type {
            "stripe" => self.decrypt_stripe(text),
            "snake" => self.decrypt_snake(text),
            _ => panic!("invalid route type"),
        }
    }

}

impl fmt::Display for Route<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Route\nroute type: {}\nkey: {:?}",self.route_type, self.key)
    }
}