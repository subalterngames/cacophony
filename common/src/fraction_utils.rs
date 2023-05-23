use fraction::GenericFraction;

/// The unsinged fraction type.
type Fu = u32;
/// Type alias for a fraction.
pub type Fraction = GenericFraction<Fu>;
/// Serializable array of the numerator and denominator of a fraction.
pub type SerializableFraction = [Fu; 2];

/// Convert a fraction in a 2-element array that can be serialized.
pub(crate) fn serialize_fraction(fraction: &Fraction) -> SerializableFraction {
    match fraction.numer() {
        Some(numer) => match fraction.denom() {
            Some(denom) => [*numer, *denom],
            None => panic!("Fraction {:?} has an invalid denominator", fraction),
        },
        None => panic!("Fraction {:?} has an invalid numerator", fraction),
    }
}

/// Deserialize a 2-element array into a fraction.
pub(crate) fn deserialize_fraction(fraction: &SerializableFraction) -> Fraction {
    Fraction::new(fraction[0], fraction[1])
}

/// Parse a fraction from a string.
pub fn fraction_from_str(s: &str) -> Fraction {
    let nd: Vec<&str> = s.split('/').collect();
    match nd[0].parse::<u32>() {
        Ok(n) => match nd[1].parse::<u32>() {
            Ok(d) => Fraction::new(n, d),
            Err(error) => panic!("Invalid denominator in fraction {}: {}", s, error),
        },
        Err(error) => panic!("Invalid numerator in fraction {}: {}", s, error),
    }
}
