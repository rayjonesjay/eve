use tonia::field_element::FieldElement;
use tonia::math::is_prime;

/// `test_field_element_addition`: tests addition between finite field elements
#[test]
fn test_field_element_addition() {
    let field_a = FieldElement::new(3, 7).unwrap();
    let field_b = FieldElement::new(1, 7).unwrap();
    let field_c = (field_a + field_b).unwrap();
    assert_eq!(field_c, FieldElement::new(4, 7).unwrap());
}

/// `test_field_element_subtraction`: tests subtraction between finite field elements
#[test]
fn test_field_element_subtraction() {
    let field_a = FieldElement::new(3, 7).unwrap();
    let field_b = FieldElement::new(1, 7).unwrap();
    let field_c = (field_a - field_b).unwrap();
    assert_eq!(field_c, FieldElement::new(2, 7).unwrap());
}

/// `test_field_element_multiplication`: tests multiplication between finite field elements
#[test]
fn test_field_element_multiplication() {
    let field_a = FieldElement::new(3, 7).unwrap();
    let field_b = FieldElement::new(1, 7).unwrap();
    let field_c = (field_a * field_b).unwrap();
    assert_eq!(field_c, FieldElement::new(3, 7).unwrap());
}

/// `test_correct_prime` - tests if the FieldElement constructor will pass if created with a prime number
#[test]
fn test_correct_prime() {
    let field = FieldElement::new(3, 7).unwrap();
    assert_eq!(field, FieldElement::new(3, 7).unwrap());
}

/// `test_wrong_prime_value` - tests if the FieldElement constructor will fail if created with a non-prime number
#[test]
fn test_wrong_prime_value() {
    let field = FieldElement::new(3, 9);
    assert!(field.is_err());
}

#[test]
fn test_equality() {
    let a = FieldElement::new(3, 7).unwrap();
    let b = FieldElement::new(3, 7).unwrap();
    assert_eq!(a == b, true);
}

#[test]
fn test_field_element_division() {
    let field_a = FieldElement::new(300, 2003).unwrap();
    let field_b = FieldElement::new(400, 2003).unwrap();
    let field_c = (field_a / field_b).unwrap();
    assert_eq!(field_c, FieldElement::new(1503, 2003).unwrap());
}
