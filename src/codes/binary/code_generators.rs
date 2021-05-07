
pub struct FixedWidthInteger {
    width: usize,
    arr: Vec<u8>,
}

impl FixedWidthInteger {
    pub fn new(width: usize) -> FixedWidthInteger  {
        let arr = vec![0;width];
        FixedWidthInteger{ width, arr }
    }
}

impl Iterator for FixedWidthInteger {
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





struct Fib {
    a: usize,
    b: usize,
}

impl Fib {
    pub fn new() -> Fib  {
        Fib{ a: 2, b: 3 }
    }
}

impl Iterator for Fib {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        let t = self.b;
        self.b += self.a;
        self.a = t;
        Some(t)
    }
}

// https://en.wikipedia.org/wiki/Fibonacci_coding
pub struct FibonacciCode {
    fib: Fib,
    vector: Vec<usize>,
    next_fib: usize,
    n: usize,
}

impl FibonacciCode {
    pub fn new() -> FibonacciCode  {
        let fib = Fib::new();
        let vector = vec![1];
        let n = 1;
        let next_fib = 2;
        FibonacciCode{ fib, vector, next_fib, n }
    }
}

impl Iterator for FibonacciCode {
    type Item = String;

    fn next(&mut self) -> Option<String> {

        // Go through the bits backward adding a 1 or 0 depending on if its part of the partition
        let mut bits = "".to_string();
        let mut val = self.n;
        for f in self.vector.iter().rev() {
            if f <= &val {
                bits.push('1');
                val -= f;
            } else {
                bits.push('0')
            }
        }

        // Reverse the bits, collect them into a String, and append a 1
        let mut output = bits.chars().rev().collect::<String>();
        output.push('1');

        // Increment the counter and append the next fibonacci number
        self.n += 1;
        if self.next_fib == self.n {
            self.vector.push(self.next_fib);
            self.next_fib = self.fib.next().unwrap()
        }
        
        Some(output)
    }
}





#[test]
fn bacon_code_gen() {
    let mut code = FixedWidthInteger::new(3);
    for _ in 0..10 {
        println!("{}",code.next().unwrap())
    }
}


#[test]
fn fib_code_gen() {
    let mut code = FibonacciCode::new();
    for _ in 0..8 {
        println!("{}",code.next().unwrap())
    }
}
