// src/math/mod.rs

pub fn is_prime(num: u64) -> bool {
    if num <= 1 {
        return false;
    };
    if num <= 3 {
        return true;
    };
    if num % 2 == 0 || num % 3 == 0 {
        return false;
    };
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
        y = num / initial_guess
    }
    initial_guess
}
