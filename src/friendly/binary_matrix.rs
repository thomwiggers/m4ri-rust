use ffi::*;
use friendly::binary_vector::BinVector;
use libc::c_int;
use std::clone;
use std::cmp;
use std::ops;
use std::ptr;
use vob::Vob;

/// Structure to represent matrices
#[derive(Debug)]
pub struct BinMatrix {
    mzd: ptr::NonNull<Mzd>,
}

unsafe impl Send for BinMatrix {}
unsafe impl Sync for BinMatrix {}

impl ops::Drop for BinMatrix {
    fn drop(&mut self) {
        unsafe {
            ptr::drop_in_place(self.mzd.as_ptr())
        }
    }
}

macro_rules! nonnull {
    ($exp:expr) => {
        ptr::NonNull::new_unchecked($exp)
    };
}

impl BinMatrix {
    /// Create a new matrix
    pub fn new(rows: Vec<BinVector>) -> BinMatrix {
        if rows.len() == 0 {
            panic!("Can't create a 0 matrix");
        }
        let first_col_length = rows[0].len();
        if cfg!(not(ndebug)) {
            for row in rows.iter() {
                debug_assert_eq!(first_col_length, row.len());
            }
        }
        let mzd_ptr = unsafe {
            mzd_init(rows.len() as c_int, rows[0].len() as c_int)
        };

        // can we do this faster?
        // Yes we can, but it's a bit scary.
        // FIXME
        for (row_index, row) in rows.into_iter().enumerate() {
            for (column_index, bit) in row.into_iter().enumerate() {
                unsafe {
                    mzd_write_bit(
                        mzd_ptr,
                        row_index as c_int,
                        column_index as c_int,
                        bit as BIT,
                    );
                }
            }
        }

        unsafe {
            BinMatrix { mzd: nonnull!(mzd_ptr) }
        }
    }

    pub fn random(rows: usize, columns: usize) -> BinMatrix {
        let mzd = unsafe { mzd_init(rows as Rci, columns as Rci) };
        // Randomize
        unsafe {
            mzd_randomize(mzd);
        }
        unsafe { BinMatrix { mzd: nonnull!(mzd) } }
    }

    pub fn from_mzd(mzd: *mut Mzd) -> BinMatrix {
        let mzd = ptr::NonNull::new(mzd).expect("Can't be NULL");
        BinMatrix { mzd }
    }

    /// Get an identity matrix
    #[inline]
    pub fn identity(rows: usize) -> BinMatrix {
        unsafe {
            let mzd_ptr = mzd_init(rows as c_int, rows as c_int);
            mzd_set_ui(mzd_ptr, 1);
            let mzd = nonnull!(mzd_ptr);
            BinMatrix { mzd }
        }
    }

    /// Augment the matrix:
    ///  [A] [B] => [A B]
    #[inline]
    pub fn augmented(&self, other: &BinMatrix) -> BinMatrix {
        let mzd = unsafe {
            nonnull!(mzd_concat(
                ptr::null_mut(),
                self.mzd.as_ptr(),
                other.mzd.as_ptr()
            ))
        };
        BinMatrix { mzd }
    }

    /// Stack the matrix with another and return the result
    #[inline]
    pub fn stacked(&self, other: &BinMatrix) -> BinMatrix {
        let mzd = unsafe {
            nonnull!(mzd_stack(
                ptr::null_mut(),
                self.mzd.as_ptr(),
                other.mzd.as_ptr()
            ))
        };
        BinMatrix { mzd }
    }

    /// Get the rank of the matrix
    ///
    /// Does an echelonization and throws it away!
    #[inline]
    pub fn rank(&self) -> usize {
        self.clone().echelonize()
    }

    /// Echelonize this matrix in-place
    ///
    /// Return: the rank of the matrix
    #[inline]
    pub fn echelonize(&mut self) -> usize {
        let rank = unsafe { mzd_echelonize(self.mzd.as_ptr(), false as c_int) };
        rank as usize
    }

    /// Compute the inverse of this matrix, returns a new matrix
    #[inline]
    pub fn inverted(&self) -> BinMatrix {
        let mzd = unsafe { nonnull!(mzd_inv_m4ri(ptr::null_mut(), self.mzd.as_ptr(), 0 as c_int)) };
        BinMatrix { mzd }
    }

    /// Compute the transpose of the matrix
    #[inline]
    pub fn transpose(&self) -> BinMatrix {
        let mzd;
        unsafe {
            let mzd_ptr = mzd_transpose(ptr::null_mut(), self.mzd.as_ptr());
            mzd = nonnull!(mzd_ptr);
        }
        BinMatrix { mzd }
    }

    /// Get the number of rows
    ///
    /// O(1)
    #[inline]
    pub fn nrows(&self) -> usize {
        unsafe { self.mzd.as_ref().nrows as usize }
    }

    /// Get the number of columns
    ///
    /// O(1)
    #[inline]
    pub fn ncols(&self) -> usize {
        unsafe { self.mzd.as_ref().ncols as usize }
    }

    /// Get as a vector
    ///
    /// Works both on single-column and single-row matrices
    pub fn as_vector(&self) -> BinVector {
        if self.nrows() != 1 {
            assert_eq!(self.ncols(), 1, "needs to have only one column or row");
            let mut b = BinVector::with_capacity(self.ncols());
            for i in 0..self.nrows() {
                let bit = unsafe { mzd_read_bit(self.mzd.as_ptr(), i as Rci, 0) == 1 };
                b.push(bit);
            }
            b
        } else {
            assert_eq!(self.nrows(), 1, "needs to have only one column or row");
            let mut b = BinVector::with_capacity(self.ncols());
            for i in 0..self.ncols() {
                let bit = unsafe { mzd_read_bit(self.mzd.as_ptr(), 0, i as Rci) == 1 };
                b.push(bit);
            }
            b
        }
    }
}

impl cmp::PartialEq for BinMatrix {
    fn eq(&self, other: &BinMatrix) -> bool {
        unsafe { mzd_equal(self.mzd.as_ptr(), other.mzd.as_ptr()) == 1 }
    }
}

impl cmp::Eq for BinMatrix {}

impl ops::Mul<BinMatrix> for BinMatrix {
    type Output = BinMatrix;

    /// Computes the product of two matrices
    #[inline]
    fn mul(self, other: BinMatrix) -> Self::Output {
        &self * &other
    }
}

impl clone::Clone for BinMatrix {
    fn clone(&self) -> Self {
        let mzd = unsafe { nonnull!(mzd_copy(ptr::null_mut(), self.mzd.as_ptr())) };
        BinMatrix { mzd }
    }
}

impl<'a> ops::Mul<&'a BinMatrix> for &'a BinMatrix {
    type Output = BinMatrix;
    /// Computes the product of two matrices
    #[inline]
    fn mul(self, other: &BinMatrix) -> Self::Output {
        unsafe {
            let mzd_ptr = mzd_mul_naive(ptr::null_mut(), self.mzd.as_ptr(), other.mzd.as_ptr());

            BinMatrix {
                mzd: ptr::NonNull::new(mzd_ptr).expect("Multiplication failed"),
            }
        }
    }
}

impl<'a> ops::Add<&'a BinMatrix> for &'a BinMatrix {
    type Output = BinMatrix;

    /// Add up two matrices
    #[inline]
    fn add(self, other: &BinMatrix) -> Self::Output {
        let mzd = unsafe {
            nonnull!(mzd_add(
                ptr::null_mut(),
                self.mzd.as_ptr(),
                other.mzd.as_ptr()
            ))
        };
        BinMatrix { mzd }
    }
}

impl ops::Add<BinMatrix> for BinMatrix {
    type Output = BinMatrix;

    /// Add up two matrices, re-uses memory of A
    #[inline]
    fn add(self, other: BinMatrix) -> Self::Output {
        let mzd = unsafe {
            nonnull!(mzd_add(
                self.mzd.as_ptr(),
                self.mzd.as_ptr(),
                other.mzd.as_ptr()
            ))
        };
        BinMatrix { mzd }
    }
}

impl ops::AddAssign<BinMatrix> for BinMatrix {
    /// Add up two matrices, re-uses memory of A
    #[inline]
    fn add_assign(&mut self, other: BinMatrix) {
        unsafe {
            mzd_add(self.mzd.as_ptr(), self.mzd.as_ptr(), other.mzd.as_ptr());
        }
    }
}

impl<'a> ops::AddAssign<&'a BinMatrix> for BinMatrix {
    /// Add up two matrices, re-uses memory of A
    #[inline]
    fn add_assign(&mut self, other: &BinMatrix) {
        unsafe {
            mzd_add(self.mzd.as_ptr(), self.mzd.as_ptr(), other.mzd.as_ptr());
        }
    }
}

impl<'a> ops::Mul<&'a BinVector> for &'a BinMatrix {
    type Output = BinVector;
    /// Computes (A * v^T)
    #[inline]
    fn mul(self, other: &BinVector) -> Self::Output {
        debug_assert_eq!(
            self.ncols(),
            other.len(),
            "Mismatched sizes: ({}x{}) * ({}x1)",
            self.nrows(),
            self.ncols(),
            other.len()
        );

        let vec_mzd = unsafe {
            let vec_mzd = mzd_init(other.len() as Rci, 1);
            debug_assert_eq!((*vec_mzd).nrows as usize, other.len(), "Row length doesn't match");
            debug_assert_eq!((*vec_mzd).ncols as usize, 1, "column length doesn't match");
            vec_mzd
        };

        for (pos, bit) in other.iter().enumerate() {
            unsafe {
                // FIXME can maybe be done faster
                // We're writing as a row here
                mzd_write_bit(vec_mzd, pos as Rci, 0, bit as BIT);
            }
        }

        let mut result = Vob::with_capacity(other.len());
        unsafe {
            let result_mzd = mzd_mul_naive(ptr::null_mut(), self.mzd.as_ptr(), vec_mzd);
            ptr::drop_in_place(vec_mzd);
            debug_assert_eq!(
                (*result_mzd).ncols as usize,
                1,
                "result is {}x{}",
                (*result_mzd).nrows,
                (*result_mzd).ncols
            );
            debug_assert_eq!((*result_mzd).nrows as usize, self.nrows());
            for i in 0..self.nrows() {
                // FIXME can be done faster
                result.push(mzd_read_bit(result_mzd, i as Rci, 0) != 0);
            }
            ptr::drop_in_place(result_mzd);
        }
        BinVector::from(result)
    }
}

impl ops::Mul<BinVector> for BinMatrix {
    type Output = BinVector;
    /// Computes (A * v^T)
    fn mul(self, other: BinVector) -> Self::Output {
        &self * &other
    }
}

impl<'a> ops::Mul<&'a BinMatrix> for &'a BinVector {
    type Output = BinVector;

    #[inline]
    /// computes v^T * A
    fn mul(self, other: &BinMatrix) -> Self::Output {
        let vec_mzd = unsafe { mzd_init(1, self.len() as Rci) };

        unsafe {
            debug_assert_eq!((*vec_mzd).ncols as usize, self.len());
            debug_assert_eq!((*vec_mzd).nrows as usize, 1);
        }

        for (pos, bit) in self.iter().enumerate() {
            unsafe {
                // FIXME can maybe be done faster
                // not sure, because we're writing as a column.
                mzd_write_bit(vec_mzd, 0, pos as Rci, bit as BIT);
            }
        }

        let tmp = unsafe {
            let tmp = mzd_init(1, self.len() as Rci);
            _mzd_mul_va(tmp, vec_mzd, other.mzd.as_ptr(), 1)
        };

        unsafe {
            debug_assert_eq!((*tmp).nrows as usize, 1);
            debug_assert_eq!((*tmp).ncols as usize, self.len());
        }

        // FIXME can this be done faster.
        let resultvob = (0..self.len())
            .map(|i| unsafe { mzd_read_bit(tmp, 0, i as Rci) == 1 })
            .collect::<Vob>();

        unsafe {
            ptr::drop_in_place(tmp);
            ptr::drop_in_place(vec_mzd);
        }

        BinVector::from(resultvob)
    }
}

impl ops::Mul<BinMatrix> for BinVector {
    type Output = BinVector;

    #[inline]
    /// computes v^T * A
    fn mul(self, other: BinMatrix) -> Self::Output {
        &self * &other
    }
}

/// Solve AX = B for X
///
/// Modifies B in-place
///
/// B will contain the solution afterwards
///
/// Return True if it succeeded
pub fn solve_left(a: BinMatrix, b: &mut BinMatrix) -> bool {
    let result = unsafe { mzd_solve_left(a.mzd.as_ptr(), b.mzd.as_ptr(), 0, 1) };

    result == 0
}

#[cfg(test)]
mod test {
    use super::*;
    use vob::Vob;

    #[test]
    fn new() {
        let _m = BinMatrix::new(vec![
            BinVector::from(vob![true, false, true]),
            BinVector::from(vob![true, true, true]),
        ]);
    }

    #[test]
    fn identity() {
        let id = BinMatrix::new(vec![
            BinVector::from(vob![
                true, false, false, false, false, false, false, false, false, false
            ]),
            BinVector::from(vob![
                false, true, false, false, false, false, false, false, false, false
            ]),
            BinVector::from(vob![
                false, false, true, false, false, false, false, false, false, false
            ]),
            BinVector::from(vob![
                false, false, false, true, false, false, false, false, false, false
            ]),
            BinVector::from(vob![
                false, false, false, false, true, false, false, false, false, false
            ]),
            BinVector::from(vob![
                false, false, false, false, false, true, false, false, false, false
            ]),
            BinVector::from(vob![
                false, false, false, false, false, false, true, false, false, false
            ]),
            BinVector::from(vob![
                false, false, false, false, false, false, false, true, false, false
            ]),
            BinVector::from(vob![
                false, false, false, false, false, false, false, false, true, false
            ]),
            BinVector::from(vob![
                false, false, false, false, false, false, false, false, false, true
            ]),
        ]);

        let id_gen = BinMatrix::identity(10);
        assert_eq!(id.nrows(), id_gen.nrows());
        assert_eq!(id.ncols(), id_gen.ncols());
        for i in 0..8 {
            for j in 0..8 {
                let m1 = id.mzd.as_ptr();
                let m2 = id_gen.mzd.as_ptr();
                unsafe {
                    assert_eq!(
                        mzd_read_bit(m1, i, j),
                        mzd_read_bit(m2, i, j),
                        "({}, {})",
                        i,
                        j
                    );
                }
            }
        }
        unsafe {
            assert!(mzd_equal(id.mzd.as_ptr(), id_gen.mzd.as_ptr()) != 0);
        }
    }

    #[test]
    fn mul() {
        let m1 = BinMatrix::identity(8);
        let m2 = BinMatrix::identity(8);
        let m3 = BinMatrix::identity(8);
        let prod = m1 * m2;
        unsafe {
            assert!(mzd_equal(prod.mzd.as_ptr(), m3.mzd.as_ptr()) != 0);
        }
    }

    #[test]
    fn vecmul() {
        let m1 = BinMatrix::identity(10);
        let binvec = BinVector::from(Vob::from_elem(10, true));

        let result: BinVector = &m1 * &binvec;
        assert_eq!(result, binvec);

        let result: BinVector = &binvec * &m1;
        assert_eq!(result, binvec);
    }

    #[test]
    fn test_random() {
        BinMatrix::random(10, 1);
    }

    #[test]
    fn test_as_vector() {
        let m1 = BinMatrix::random(10, 1);
        let vec = m1.as_vector();
        assert_eq!(vec.len(), 10);
        assert!(m1 == vec.as_column_matrix());
    }
}
