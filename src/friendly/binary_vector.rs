/// Implement binary vectors to help implement functions on matrices
///
/// Wraps the `bit_vec` crate.
use std::ops;
use vob::Vob;

use rand;
use rand::Rng;

use friendly::binary_matrix::BinMatrix;

/// Wrapper around BitVec
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
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
    pub fn from_elem(len: usize, elem: bool) -> Self {
        BinVector::from(Vob::from_elem(len, elem))
    }

    #[inline]
    pub fn from_bools(bools: &[bool]) -> BinVector {
        let vec = bools.iter().cloned().collect::<Vob>();
        BinVector { vec }
    }

    /// Construct the BinVector from the result of a function
    ///
    /// # Example
    /// ```
    /// # use m4ri_rust::friendly::BinVector;
    /// let v = BinVector::from_function(4, |i| i % 2 == 0);
    /// assert_eq!(v.get(0), Some(true));
    /// assert_eq!(v.get(1), Some(false));
    /// ```
    #[inline]
    pub fn from_function(len: usize, f: fn(usize) -> bool) -> BinVector {
        let mut vob = Vob::with_capacity(len);
        for i in 0..len {
            vob.push(f(i));
        }
        BinVector::from(vob)
    }

    #[inline]
    pub fn random(len: usize) -> BinVector {
        let mut rng = rand::thread_rng();
        let mut vob = Vob::with_capacity(len);
        for _ in 0..len {
            vob.push(rng.gen());
        }
        BinVector::from(vob)
    }

    #[inline]
    pub fn with_capacity(len: usize) -> Self {
        BinVector::from(Vob::with_capacity(len))
    }

    /// Create a new BinVector from an `&[u8]`.
    #[inline]
    pub fn from_bytes(bytes: &[u8]) -> BinVector {
        let vec: Vob = Vob::from_bytes(bytes);

        BinVector { vec }
    }

    #[inline]
    pub fn count_ones(&self) -> u32 {
        self.iter_storage()
            .fold(0u32, |acc, block| acc + block.count_ones())
    }

    #[inline]
    pub fn extend_from_binvec(&mut self, other: &BinVector) {
        self.vec.extend_from_vob(&other.vec);
    }

    #[inline]
    pub fn to_vob(self) -> Vob {
        self.vec
    }

    pub fn as_matrix(&self) -> BinMatrix {
        BinMatrix::new(vec![self.clone()])
    }

    pub fn as_column_matrix(&self) -> BinMatrix {
        self.as_matrix().transpose()
    }

    pub fn as_u32(&self) -> u32 {
        assert!(self.len() < 32, "Can't convert this to a >32 bit number");
        if let Some(i) = self.iter_storage().next() {
            i as u32
        } else {
            0
        }
    }
    pub fn as_u64(&self) -> u64 {
        assert!(self.len() < 64, "Can't convert this to a >32 bit number");
        if let Some(i) = self.iter_storage().next() {
            i as u64
        } else {
            0
        }
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
    fn from_bytes() {
        let b = BinVector::from_bytes(&[0b11111111]);
        assert_eq!(b.len(), 8);

        let b = BinVector::from_bytes(&[0b10000000]);
        assert_eq!(b.get(0), Some(true));
        assert_eq!(b.get(1), Some(false));
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

    #[test]
    fn as_matrix() {
        let a = BinVector::random(10);
        let amat = a.as_matrix();
        assert_eq!(amat.ncols(), 10);
        assert_eq!(amat.nrows(), 1);
        assert_eq!(amat.as_vector(), a);
    }

    #[test]
    fn as_column_matrix() {
        let a = BinVector::random(10);
        let amat = a.as_column_matrix();
        assert_eq!(amat.ncols(), 1);
        assert_eq!(amat.nrows(), 10);
        assert_eq!(amat.as_vector(), a);
    }

    #[test]
    fn count_ones() {
        let a = BinVector::from_elem(10, true);
        let b = BinVector::from_elem(10, false);
        assert_eq!(a.count_ones(), 10);
        assert_eq!(b.count_ones(), 0);
        assert_eq!(BinVector::from_bytes(&[0b10101000]).count_ones(), 3);
    }
}
