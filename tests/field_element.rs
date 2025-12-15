use eve::FieldElement;

#[test]
fn test_field_element_addition() {
    let field_a = FieldElement::new(3,7);
    let field_b = FieldElement::new(1,7);
    let field_c = (field_a + field_b).unwrap();
    assert_eq!(field_c, FieldElement::new(4,7));
}