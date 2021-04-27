use std::fmt;
use std::ops::{Add, Mul, Neg, Sub, Div};



fn egcd(a: i32, b: i32) -> (i32,i32,i32) {
    if a == 0 {
        (b,0,1)
    } else {
        let (g, y, x) = egcd(b%a, a);
        (g,x-(b/a)*y,y)
    }
}


pub trait Modular {
    fn to_modulo(self, modulus: u32) -> Modulo;
}

impl Modular for u32 {
    fn to_modulo(self, modulus: u32) -> Modulo {
        Modulo { value: self % modulus, modulus }
    }
}

#[derive(Debug,PartialEq,Eq,Clone,Copy,Hash)]
pub struct Modulo {
    value: u32,
    modulus: u32
}

impl Modulo {

    pub fn new(value: u32, modulus: u32) -> Modulo {
        Modulo {value, modulus }
    }

    pub fn zero(modulus: u32) -> Modulo {
        Modulo {value: 0, modulus}
    }

    pub fn one(modulus: u32) -> Modulo {
        Modulo {value: 1, modulus}
    }

    pub fn value(&self) -> u32 {
        self.value
    }

    pub fn modulus(&self) -> u32 {
        self.modulus
    }

    pub fn mul_inv(&self) -> Option<Modulo> {
        let (g, x, _) = egcd(self.value as i32, self.modulus as i32);
        if g != 1 {
            None 
        } else {
            Some( Modulo{ value: (x.rem_euclid(self.modulus as i32) ) as u32, modulus: self.modulus } )
        }
    }
}


#[macro_export]
macro_rules! modulo {
    ($rem:expr, $div:expr) => {
        $rem.to_modulo($div)
    };
}


impl fmt::Display for Modulo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} % {:?}", self.value, self.modulus)
    }
}



impl Add for Modulo {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        if self.modulus() != rhs.modulus() {
            panic!("Addition is only valid for modulo numbers with the same dividend")
        }

        (self.value() + rhs.value()).to_modulo(self.modulus())
    }
}

impl Sub for Modulo {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        if self.modulus() != rhs.modulus() {
            panic!("Subtraction is only valid for modulo numbers with the same dividend")
        }

        if self.value() >= rhs.value() {
            modulo!(self.value() - rhs.value(), self.modulus())
        } else {
            modulo!(
                self.modulus() + self.value() - rhs.value(),
                self.modulus()
            )
        }
    }
}

// For our purposes this won't be an issue but this should handle overflow
impl Mul for Modulo {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        if self.modulus() != rhs.modulus() {
            panic!("Multiplication is only valid for modulo numbers with the same dividend")
        }

        (self.value() * rhs.value()).to_modulo(self.modulus())
    }
}

impl Neg for Modulo {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            value: self.modulus - self.value,
            modulus: self.modulus,
        }
    }
}



impl Div for Modulo {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        if self.modulus() != rhs.modulus() {
            panic!("Multiplication is only valid for modulo numbers with the same dividend")
        }

        let inv = rhs.mul_inv();
        if inv.is_none() {
            panic!("Dvisior has no modnular multiplicative inverse.")
        } else {
            (self.value() * inv.unwrap().value()).to_modulo(self.modulus())
        }

    }
}


#[cfg(test)]
mod test {
    use super::*;

    
    #[test]
    fn negation() {
        let n1 = modulo!(14,26);
        let i1 = -n1;
        assert_eq!(i1,modulo!(12,26));
    }

     #[test]
    fn reciprocal() {
        let n1 = modulo!(7,26);
        let i1 = n1.mul_inv().unwrap();
        assert_eq!(i1*n1,Modulo::one(26));
    }

    #[test]
    fn addition() {
        let n1 = modulo!(14,26);
        let n2 = modulo!(23,26);
        assert_eq!(n1+n2,11.to_modulo(26));
    }

    #[test]
    fn subtraction() {
        let n1 = modulo!(14,26);
        let n2 = modulo!(23,26);
        assert_eq!(n1-n2,17.to_modulo(26));
    }

    #[test]
    fn division() {
        let n1 = modulo!(15,26);
        let n2 = modulo!(7,26);
        assert_eq!(n1/n2,17.to_modulo(26));
    }
}