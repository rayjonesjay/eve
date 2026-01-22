// src/math/mod.rs

///is_prime: check if number is prime
/// ## Arguments
///* `num` - unsigned 64-bit number
/// ## Returns
/// `bool` - true if prime else false
pub fn is_prime(num: u64) -> bool {
    // 1 and numbers less than 1 are not primes
    if num <= 1 {
        return false;
    };
    // 2 and 3 are primes
    if num <= 3 {
        return true;
    };
    // other multiples of 2 and 3 are not prime
    if num % 2 == 0 || num % 3 == 0 {
        return false;
    };

    // compute integer square root
    let sqrt_limit = integer_sqrt(num);
    for i in (5..=sqrt_limit).step_by(6) {
        if num % i == 0 || num % (i + 2) == 0 {
            return false;
        };
    }
    true
}

pub fn integer_sqrt(num: u64) -> u64 {
    let mut initial_guess = num / 2;
    let mut y = 1;
    while initial_guess > y {
        initial_guess = (initial_guess + y) / 2;
        y = num/initial_guess
    }
    initial_guess
}