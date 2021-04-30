use std::collections::HashMap;
use num::Integer;

// https://en.wikipedia.org/wiki/Fibonacci_coding
pub struct FibonnaciCode {

}

// Effectively just fixed width integers in binary
pub struct BaconCode {
    width: usize,
    arr: Vec<u8>,
}

impl BaconCode {
    pub fn new(width: usize) -> BaconCode  {
        let arr = vec![0;width];
        BaconCode{ width, arr }
    }
}

impl Iterator for BaconCode {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        let output = self.arr.iter().rev().map(|n| if *n == 0 {'0'} else {'1'}).collect();
        let mut carry = 1;
        for i in 0..self.width {
            if carry == 1 {
                self.arr[i] += 1;
                carry = self.arr[i] / 2;
                self.arr[i] %= 2;
            }
        }
        Some(output)
    }
}


#[test]
fn bacon_code_gen() {
    let mut code = BaconCode::new(3);
    for i in 0..10 {
        println!("{}",code.next().unwrap())
    }
}
