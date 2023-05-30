use crate::BigUint;

use super::ApUint;

impl From<&ApUint> for BigUint {
    fn from(apuint: &ApUint) -> BigUint {
        BigUint::from_digits(&apuint.data)
    }
}
