use std::{cell::Cell, fmt};
use rand::{prelude::SliceRandom, thread_rng};
use itertools::Itertools;

const RS44_DIGRAPHS: [&'static str; 25] = ["aa", "ba", "ca", "da", "ea", "ab", "bb", "cb", "db", "eb", "ac", "bc", "cc", "dc", "ec", "ad", "bd", "cd", "dd", "de", "ae", "be", "ce", "de", "ee"];

fn tuple_to_usize(cell: (usize,usize)) -> usize {
    cell.0*24 + cell.1
}

fn usize_to_tuple(n: usize) -> (usize,usize) {
    (n / 24, n % 24)
}

pub fn random_RS44_grid_holes() -> [usize; 240] {
    let mut rng = thread_rng();
    let mut out = [0usize; 240];
    let mut nums = vec![0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24];
    let mut ctr = 0;
    for row in 0..24 {
        nums.shuffle(&mut rng);
        for i in nums[..10].iter().sorted() {
            out[ctr] = i + (row * 25);
            ctr += 1
        }
    }
    out
}

pub fn create_grid_vector(holes: [usize; 240]) -> Vec<char> {
    let mut g = Vec::with_capacity(600);
    for i in 0..600 {
        if holes.contains(&i) {
            g.push('_')
        } else {
            g.push('#')
        }
    }
    g
}

pub fn display_grid_blank(holes: [usize; 240]) -> String {
    let v = create_grid_vector(holes);
    let mut out = String::new();
    for row in 0..24 {
        let lo = row*25;
        let hi = lo+25;
        out.push_str(&v[lo..hi].iter().join(" "));
        out.push('\n')
    }
    out
}


/// The Rasterschlüssel 44 (RS44) Cipher is a sophisticated grille cipher
pub struct RS44 {
    column_nums: [usize; 25],
    column_pairs: [&'static str; 25],
    row_pairs: [&'static str; 24],
    dimensions: (usize,usize), // there are always 24 rows and 25 columns (ROWS BEFORE COLUMNS)
    spaces: [usize; 240], // exactly ten spaces are left open in each row 
    start: Cell<(usize,usize)>,
    time: Cell<usize>,
}


impl RS44 {
    pub fn new(column_nums: [usize; 25], column_pairs: [&'static str; 25], row_pairs: [&'static str; 24], spaces: [usize; 240]) -> RS44 {
        RS44{ column_nums, column_pairs, row_pairs, dimensions: (24,25), spaces, start: Cell::new((0,0)), time: Cell::new(0) }
    }

/*     fn create_grid_vector(&self) -> Vec<Vec<char>> {
        let mut grid = Vec::new();
        let mut ctr = 0;
        for _ in 0..self.dimensions.0 {
            let mut v = Vec::with_capacity(self.dimensions.1);
            for _ in 0..self.dimensions.1 {
                if self.spaces.contains(&ctr) {
                    v.push('_')
                } else {
                    v.push('#')
                }
                ctr += 1
            }
            grid.push(v)
        }
        grid
    } */

    pub fn set_start(&self, cell: (usize,usize)) {
        if self.spaces.contains(&tuple_to_usize(cell)) {
            panic!("Must start in an open space")
        }
        self.start.set(cell)
    }

    pub fn set_time(&self, time: usize) {
        self.time.set(time)
    }


}

impl crate::auxiliary::Cipher for RS44 {
    
    fn encrypt(&self, text: &str) -> String {
        let tlen = text.chars().count();
        if tlen > 240 {
            panic!("The maximum text length for the RS44 cipher is 240 characters")
        }
        let symbols = text.chars();
        let out = String::with_capacity(tlen);
        let mut v = create_grid_vector(self.spaces);
        let mut ctr = tuple_to_usize(self.start.get());
        for s in symbols {
            while v[ctr] == '#' {
                ctr += 1
            }
            v[ctr] = s;
            ctr += 1
        }

        out
    }

    fn decrypt(&self, text: &str) -> String {
        let out = String::with_capacity(text.chars().count());

        out
    }

}

impl fmt::Display for RS44 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RS44 Composite Cipher")
    }
}


#[test]
fn test_random_grid() {
    let a = random_RS44_grid_holes();
    //println!("{:?}",a);

    //let v = create_grid_vector(a);
    //println!("{:?}",v);

    let g = display_grid_blank(a);
    println!("{}",g);

}