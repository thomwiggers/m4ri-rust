/// Implement binary vectors to help implement functions on matrices
///
/// Wraps the `bit_vec` crate.
use bit_vec::BitVec;
use std::iter;
use std::ops;

/// Wrapper around BitVec
#[derive(Clone, Debug, PartialEq)]
pub struct BinVector {
    vec: BitVec,
}

impl ops::Deref for BinVector {
    type Target = BitVec;
    fn deref(&self) -> &Self::Target {
        &self.vec
    }
}

impl BinVector {
    pub fn from(vec: BitVec) -> Self {
        BinVector { vec }
    }

    pub fn count_ones(&self) -> u32 {
        self.storage().iter().fold(0u32, |acc, block| acc + block.count_ones())
    }

    pub fn to_bitvec(self) -> BitVec {
        self.vec
    }
}

impl iter::IntoIterator for BinVector {
    type Item = bool;
    type IntoIter = ::bit_vec::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.vec.into_iter()
    }
}

impl<'a> ops::Add<&'a BinVector> for &'a BinVector {
    type Output = BinVector;
    #[inline]
    fn add(self, other: &BinVector) -> Self::Output {
        assert_eq!(self.len(), other.len(), "unequal length vectors");
        let len = self.len();
        let mut new_bitvec = BitVec::with_capacity(self.len());
        {
            let storage: &mut Vec<u32>;
            unsafe {
                {
                    if self.len() % 32 == 0 {
                        new_bitvec.set_len(len);
                    } else {
                        // set one longer as we need to truncate
                        new_bitvec.set_len(len + 1);
                    }
                }
                storage = new_bitvec.storage_mut();
                storage.set_len(self.storage().len());
            }
            for (block, (a, b)) in storage
                .iter_mut()
                .zip(self.storage().iter().zip(other.storage().iter()))
            {
                *block = a ^ b;
            }
        }
        new_bitvec.truncate(len);
        BinVector::from(new_bitvec)
    }
}

impl ops::Add<BinVector> for BinVector {
    type Output = BinVector;

    #[inline]
    fn add(self, other: BinVector) -> Self::Output {
        assert_eq!(self.len(), other.len(), "unequal length vectors");
        let len = self.len();
        let mut new_bitvec = BitVec::with_capacity(self.len());
        {
            let storage;
            unsafe {
                {
                    if self.len() % 32 == 0 {
                        new_bitvec.set_len(len);
                    } else {
                        // set one longer as we need to truncate
                        new_bitvec.set_len(len + 1);
                    }
                }
                storage = new_bitvec.storage_mut();
                storage.set_len(self.storage().len());
            }
            for (block, (a, b)) in storage
                .iter_mut()
                .zip(self.storage().into_iter().zip(other.storage().into_iter()))
            {
                *block = a ^ b;
            }
        }
        new_bitvec.truncate(len);
        BinVector::from(new_bitvec)
    }
}

impl<'a> ops::AddAssign<&'a BinVector> for BinVector {
    #[inline]
    fn add_assign(&mut self, other: &BinVector) {
        let storage;
        unsafe {
            storage = (&mut self.vec).storage_mut();
        }
        for (mut a, b) in storage.iter_mut().zip(other.storage().iter()) {
            *a ^= b;
        }
    }
}

impl ops::AddAssign<BinVector> for BinVector {
    #[inline]
    fn add_assign(&mut self, other: BinVector) {
        let storage;
        unsafe {
            storage = (&mut self.vec).storage_mut();
        }
        for (mut a, b) in storage.iter_mut().zip(other.storage().into_iter()) {
            *a ^= b;
        }
    }
}

impl<'a> ops::Mul<&'a BinVector> for &'a BinVector {
    type Output = bool;

    #[inline]
    fn mul(self, other: &BinVector) -> Self::Output {
        let mut vec = self.clone();
        (&mut vec.vec).intersect(&other);
        if vec.count_ones() % 2 == 1 {
            true
        } else {
            false
        }
    }
}

impl ops::Mul<BinVector> for BinVector {
    type Output = bool;

    #[inline]
    fn mul(self, other: BinVector) -> Self::Output {
        let mut vec = self.clone();
        (&mut vec.vec).intersect(&other);
        if vec.count_ones() % 2 == 1 {
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use bit_vec::BitVec;

    #[test]
    fn init() {
        let b = BitVec::from_elem(10, false);
        let b = BinVector::from(b);
        assert_eq!(b.len(), 10);
    }

    #[test]
    fn add() {
        let a = BinVector::from(BitVec::from_elem(10, false));
        let b = BinVector::from(BitVec::from_elem(10, false));

        let c = &a + &b;

        assert_eq!(c.len(), 10, "length incorrect");
        assert_eq!(BitVec::from_elem(10, false), *c);
        assert_eq!(c, a + b);
    }

    #[test]
    fn mul() {
        let a = BinVector::from(BitVec::from_elem(10, true));
        let b = BinVector::from(BitVec::from_elem(10, false));

        let c = &a * &b;

        assert_eq!(c.len(), 10, "length incorrect");
        assert_eq!(BitVec::from_elem(10, false), *c);
        assert_eq!(c, a * b);
    }
}
