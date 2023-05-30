use core::ops::{Add, AddAssign};
use crate::biguint::IntDigits;

use crate::biguint::addition::vec_add;

use super::ApUint;

impl<'a> Add<&'a ApUint> for ApUint {
    type Output = ApUint;

    fn add(mut self, other: &ApUint) -> ApUint {
        self.len = self.len.max(other.len);
        self += other;
        self
    }
}
impl<'a> AddAssign<&'a ApUint> for ApUint {
    #[inline]
    fn add_assign(&mut self, other: &ApUint) {
        vec_add(&mut self.data, &other.data);
        self.normalize();
    }
}

forward_all_binop_to_val_ref_commutative!(impl Add for ApUint, add);
forward_val_assign!(impl AddAssign for ApUint, add_assign);

