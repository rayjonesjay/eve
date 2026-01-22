use tonia::math::is_prime;
use tonia::math::integer_sqrt;

/// test_prime - test prime check function
#[test]
fn test_prime() {
    assert_eq!(is_prime(64), false);
    assert_eq!(is_prime(20_000_011), false);
    assert_eq!(is_prime(134567897162732), false);
    assert_eq!(is_prime(134567897162731), false);
}

#[test]
fn test_integer_sqrt() {
    assert_eq!(integer_sqrt(64), 8);
    assert_eq!(integer_sqrt(0), 0);
    assert_eq!(integer_sqrt(25),5);
}