use fraction::Fraction;

/// Convert a fraction in a 2-element array that can be serialized.
///
/// - `fraction` The fraction we want to serialize.
pub fn serialize_fraction(fraction: &Fraction) -> [u64; 2] {
    match fraction.numer() {
        Some(numer) => match fraction.denom() {
            Some(denom) => [*numer, *denom],
            None => panic!("Fraction {:?} has an invalid denominator", fraction),
        },
        None => panic!("Fraction {:?} has an invalid numerator", fraction),
    }
}
