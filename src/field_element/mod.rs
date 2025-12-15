// src/field_element.rs
use std::fmt;
use std::ops::{Add, Sub};
use crate::math::is_prime;

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
        if !is_prime(prime) {
            return Err("order must be a prime number".to_string());
        }
        if element >= prime {
            return Err(format!("{} not in field range 0 to {}", element, prime - 1));
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

// PartialEq checks if two objects of type FieldElement are equal
impl PartialEq for FieldElement {
    fn eq(&self, other: &Self) -> bool {
        self.prime == other.prime && self.element == other.element
    }
}

// leave the body empty
impl Eq for FieldElement {}

