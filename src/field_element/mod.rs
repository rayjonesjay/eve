// src/field_element.rs
use crate::math::is_prime;
use std::fmt;
use std::ops::{Add, Mul, Sub, Div};

#[derive(Clone, Copy)]
pub struct FieldElement {
    pub element: u64,
    pub prime: u64,
}

impl fmt::Debug for FieldElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "FieldElement {{ element: {}, prime: {} }}",
            self.element, self.prime
        )
    }
}

impl FieldElement {
    pub fn new(element: u64, prime: u64) -> Result<Self, String> {
        if element >= prime {
            return Err(format!("{} not in field range 0 to {}", element, prime - 1));
        }
        if !is_prime(prime) {
            return Err(format!("{} is not prime", prime));
        }
        Ok(FieldElement { element, prime })
    }
}

impl Add for FieldElement {
    type Output = Result<FieldElement, String>;

    fn add(self, other: FieldElement) -> Result<FieldElement, String> {
        if self.prime != other.prime {
            return Err("cannot subtract field elements from different fields".to_string());
        }
        let result: u64 = (self.element + other.element) % self.prime;
        Ok(FieldElement {
            element: result,
            prime: self.prime,
        })
    }
}

// implement subtraction - operation for finite field element
impl Sub for FieldElement {
    type Output = Result<FieldElement, String>;

    fn sub(self, other: Self) -> Result<Self, String> {
        if self.prime != other.prime {
            const ERR_MSG: &str = "cannot subtract elements from different fields.";
            return Err(ERR_MSG.to_string());
        }
        let result: u64 = (self.element - other.element) % self.prime;
        Ok(FieldElement {
            element: result,
            prime: self.prime,
        })
    }
}

impl Mul for FieldElement {
    type Output = Result<FieldElement, String>;
    fn mul(self, other: FieldElement) -> Result<FieldElement, String> {
        if self.prime != other.prime {
            return Err("cannot multiply elements from different fields.".to_string());
        }
        let result: u64 = (self.element * other.element) % self.prime;
        Ok(FieldElement {
            element: result,
            prime: self.prime,
        })
    }
}

// PartialEq checks if two objects of type FieldElement are equal
impl PartialEq for FieldElement {
    fn eq(&self, other: &Self) -> bool {
        self.prime == other.prime && self.element == other.element
    }
}

impl Div for FieldElement {
    type Output = Result<FieldElement, String>;
    fn div(self, other: FieldElement) -> Result<FieldElement, String> {
       if self.prime != other.prime {
           return Err("cannot divide elements from different fields.".to_string());
       }
        let mut inverse_of_divisor = 0;
        // loop from 0 up to prime-1
        for i in 0..self.prime{
            if (other.element * i) % self.prime == 1 {
                inverse_of_divisor = i;
                break
            }
        }
        Ok(FieldElement{element:(self.element * inverse_of_divisor) % self.prime,prime:self.prime})
    }
}

impl Eq for FieldElement {}
