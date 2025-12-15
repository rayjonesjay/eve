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

    // get the square root of num by converting it to float then computing square root
    // then convert to u64
    let sqrt_limit = (num as f64).sqrt() as u64;
    for i in (5..=sqrt_limit).step_by(6) {
        if num % i == 0 || num % (i + 2) == 0 {
            return false;
        };
    }
    true
}
