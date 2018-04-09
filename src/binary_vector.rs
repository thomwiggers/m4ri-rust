/// Implement binary vectors to help implement functions on matrices
///
/// Wraps the `bit_vec` crate.
use std::ops;
use vob::Vob;

/// Wrapper around BitVec
#[derive(Clone, Debug, PartialEq)]
pub struct BinVector {
    vec: Vob,
}

impl ops::Deref for BinVector {
    type Target = Vob;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.vec
    }
}

impl ops::DerefMut for BinVector {

    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.vec
    }
}

impl BinVector {
    #[inline]
    pub fn new() -> Self {
        BinVector::from(Vob::new())
    }

    #[inline]
    pub fn from(vec: Vob) -> Self {
        BinVector { vec }
    }

    #[inline]
    pub fn with_capacity(len: usize) -> Self {
        BinVector::from(Vob::with_capacity(len))
    }

    /// Create a new BinVector from an `&[u8]`.
    #[inline]
    pub fn from_bytes(bytes: &[u8]) -> BinVector  {
        let mut vec = Vob::with_capacity(bytes.len());
        // TODO Speed this up
        for byte in bytes {
            for i in (0..7).rev() {
                vec.push(byte >> i & 1u8 == 1u8);
            }
        }

        BinVector { vec }
    }

    #[inline]
    pub fn count_ones(&self) -> u32 {
        self.iter_storage().fold(0u32, |acc, block| acc + block.count_ones())
    }

    #[inline]
    pub fn to_vob(self) -> Vob {
        self.vec
    }
}

impl<'a> ops::Add<&'a BinVector> for &'a BinVector {
    type Output = BinVector;
    #[inline]
    fn add(self, other: &BinVector) -> Self::Output {
        let mut new = self.clone();
        new += other;
        new
    }
}

impl ops::Add<BinVector> for BinVector {
    type Output = BinVector;

    #[inline]
    fn add(self, other: BinVector) -> Self::Output {
        assert_eq!(self.len(), other.len(), "unequal length vectors");
        let mut new = self.clone();
        new += other;
        new
    }
}

impl<'a> ops::AddAssign<&'a BinVector> for BinVector {
    #[inline]
    fn add_assign(&mut self, other: &BinVector) {
        assert_eq!(self.len(), other.len(), "unequal length vectors");
        self.xor(&*other);
    }
}

impl ops::AddAssign<BinVector> for BinVector {
    #[inline]
    fn add_assign(&mut self, other: BinVector) {
        assert_eq!(self.len(), other.len(), "unequal length vectors");
        self.xor(&*other);
    }
}

impl<'a> ops::Mul<&'a BinVector> for &'a BinVector {
    type Output = bool;

    #[inline]
    fn mul(self, other: &BinVector) -> Self::Output {
        let mut vec = self.clone();
        vec.and(&other);
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
        vec.and(&other);
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
    use vob::Vob;

    #[test]
    fn init() {
        let b = Vob::from_elem(10, false);
        let b = BinVector::from(b);
        assert_eq!(b.len(), 10);
    }

    #[test]
    fn add() {
        let a = BinVector::from(Vob::from_elem(10, false));
        let b = BinVector::from(Vob::from_elem(10, false));

        let c = &a + &b;

        assert_eq!(c.len(), 10, "length incorrect");
        assert_eq!(Vob::from_elem(10, false), *c);
        assert_eq!(c, a + b);
    }

    #[test]
    fn mul() {
        let a = BinVector::from(Vob::from_elem(10, true));
        let b = BinVector::from(Vob::from_elem(10, false));

        let c = &a * &b;

        assert_eq!(false, c);
        assert_eq!(c, a * b);
    }
}
