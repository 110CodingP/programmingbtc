use finite_fields::FieldElement;
use rug::Integer;

fn main() {
    let field_a = FieldElement::new(
        Integer::from(5), 
        Integer::from(31)
    );
    // println!("Multiplication of terms, {}", 3 * field_a);
}
