use eve::math::is_prime;

/// test_prime - test prime check function
#[test]
fn test_prime() {
    assert_eq!(is_prime(1), false);
    assert_eq!(is_prime(7), true);
    assert_eq!(is_prime(134567897162732), false);
}
