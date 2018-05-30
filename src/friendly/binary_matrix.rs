use ffi::*;
use friendly::binary_vector::BinVector;
use libc::c_int;
use std::clone;
use std::cmp;
use std::ops;
use std::ptr;

/// Structure to represent matrices
#[derive(Debug)]
pub struct BinMatrix {
    mzd: ptr::NonNull<Mzd>,
}

unsafe impl Sync for BinMatrix {}

impl ops::Drop for BinMatrix {
    fn drop(&mut self) {
        unsafe { ptr::drop_in_place(self.mzd.as_ptr()) }
    }
}

macro_rules! nonnull {
    ($exp:expr) => {
        ptr::NonNull::new_unchecked($exp)
    };
}

impl BinMatrix {
    /// Create a zero matrix
    pub fn zero(rows: usize, cols: usize) -> BinMatrix {
        if rows == 0 || cols == 0 {
            panic!("Can't create a 0 matrix");
        }
        let mzd = unsafe { nonnull!(mzd_init(rows as c_int, cols as c_int)) };
        BinMatrix { mzd }
    }

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
        let mzd_ptr = unsafe { mzd_init(rows.len() as c_int, rows[0].len() as c_int) };

        // Directly write to the underlying Mzd storage
        for (row_index, row) in rows.into_iter().enumerate() {
            let row_ptr: *const *mut Word = unsafe { (*mzd_ptr).rows.offset(row_index as isize) };
            for (block_index, row_block) in row.iter_storage().enumerate() {
                assert_eq!(
                    ::std::mem::size_of::<usize>(),
                    ::std::mem::size_of::<u64>(),
                    "only works on 64 bit"
                );
                unsafe {
                    *((*row_ptr).offset(block_index as isize)) = row_block as u64;
                }
            }
        }

        unsafe {
            BinMatrix {
                mzd: nonnull!(mzd_ptr),
            }
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
        debug_assert_eq!(self.nrows(), other.nrows(), "The rows need to be equal");
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
            self.transpose().as_vector()
        } else {
            assert_eq!(self.nrows(), 1, "needs to have only one column or row");
            let mut bits = BinVector::with_capacity(self.ncols());
            {
                let collector: &mut Vec<usize> = unsafe { bits.get_storage_mut() };
                for i in 0..(self.ncols() / 64) {
                    let row_ptr: *const *mut Word = unsafe { (*self.mzd.as_ptr()).rows };
                    let word_ptr: *const Word =
                        unsafe { ((*row_ptr) as *const Word).offset(i as isize) };
                    collector.push(unsafe { *word_ptr as usize });
                }
                // process last block
                if self.ncols() % 64 != 0 {
                    let row_ptr: *const *mut Word = unsafe { (*self.mzd.as_ptr()).rows };
                    let word_ptr: *const Word =
                        unsafe { (*row_ptr).offset((self.ncols() as isize - 1) / 64) };
                    let word = unsafe { *word_ptr };
                    collector.push(word as usize);
                }
            }
            unsafe {
                bits.set_len(self.ncols());
                bits.mask_last_block();
            }

            bits
        }
    }

    /// Get a certain bit
    pub fn bit(&self, row: usize, col: usize) -> bool {
        let bit = unsafe { mzd_read_bit(self.mzd.as_ptr(), row as Rci, col as Rci) };
        debug_assert!(bit == 0 || bit == 1, "Invalid bool for bit??");
        bit == 1
    }

    /// Get a window from the matrix
    pub fn get_window(
        &self,
        start_row: usize,
        start_col: usize,
        high_row: usize,
        high_col: usize,
    ) -> BinMatrix {
        let (rows, cols) = (high_row - start_row, high_col - start_col);
        let mzd_ptr = unsafe { mzd_init(rows as Rci, cols as Rci) };
        for (r, i) in (start_row..high_row).enumerate() {
            // FIXME speed
            for (c, j) in (start_col..high_col).enumerate() {
                let bit = self.bit(i, j);
                unsafe {
                    mzd_write_bit(mzd_ptr, r as Rci, c as Rci, bit as BIT);
                }
            }
        }
        BinMatrix::from_mzd(mzd_ptr)
    }


    /// Set a window in the matrix to another matrix
    ///
    /// Currently does bit-by-bit, should use more optimal means
    /// if alignment allows it
    pub fn set_window(&mut self, start_row: usize, start_col: usize, other: &BinMatrix) {
        let highr = start_row + other.nrows();
        let highc = start_col + other.ncols();
        debug_assert!(self.ncols() >= highc, "This matrix is too small!");
        debug_assert!(self.nrows() >= highr, "This matrix has too few rows !");
        let mzd_ptr = self.mzd.as_ptr();

        for r in start_row..highr {
            for c in start_col..highc {
                let bit = other.bit(r - start_row, c - start_col);
                unsafe {
                    mzd_write_bit(mzd_ptr, r as Rci, c as Rci, bit as BIT);
                }
            }
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
            let mzd_ptr = mzd_mul(ptr::null_mut(), self.mzd.as_ptr(), other.mzd.as_ptr(), 0);

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
        let other_mat = other.as_matrix();
        let result_dest = unsafe{ mzd_init(self.nrows() as Rci, 1) };
        let result = unsafe { _mzd_mul_naive(result_dest, self.mzd.as_ptr(), other_mat.mzd.as_ptr(), 0) };
        let matresult = BinMatrix::from_mzd(result);

        matresult.as_vector()
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
        let vec_mzd = self.as_matrix();
        let tmp = &vec_mzd * other;

        tmp.as_vector()
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
        assert_eq!(id, id_gen);
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

        let m1 = BinMatrix::random(10, 3);
        let result = &binvec * &m1;
        assert_eq!(result.len(), 3);
    }

    #[test]
    fn test_random() {
        BinMatrix::random(10, 1);
    }

    #[test]
    fn test_as_vector_column() {
        for i in 1..25 {
            let m1 = BinMatrix::random(i, 1);
            let vec = m1.as_vector();
            assert_eq!(vec.len(), i);
            assert!(m1 == vec.as_column_matrix());
        }
    }

    #[test]
    fn test_as_vector_row() {
        for i in 1..25 {
            let m1 = BinMatrix::random(1, i);
            let vec = m1.as_vector();
            assert_eq!(vec.len(), i);
            assert!(m1 == vec.as_matrix());
        }
    }

    #[test]
    fn zero() {
        let m1 = BinMatrix::zero(10, 3);
        for i in 0..10 {
            for j in 0..3 {
                assert_eq!(m1.bit(i, j), false);
            }
        }
    }

    #[test]
    fn set_window() {
        let mut m1 = BinMatrix::zero(10, 10);
        m1.set_window(5, 5, &BinMatrix::identity(5));
        for i in 0..5 {
            for j in 0..5 {
                assert_eq!(m1.bit(i, j), false);
            }
        }
        for i in 5..10 {
            for j in 5..10 {
                let bit = m1.bit(i, j);
                assert_eq!(bit, i == j, "bit ({},{}) was {}", i, j, bit);
            }
        }

        let mut m1 = BinMatrix::random(10, 10);
        m1.set_window(5, 5, &BinMatrix::identity(5));
        for i in 5..10 {
            for j in 5..10 {
                let bit = m1.bit(i, j);
                assert_eq!(bit, i == j, "bit ({},{}) was {}", i, j, bit);
            }
        }
    }

    #[test]
    fn memory_leaks() {
        for _ in 0..100000 {
            let v = BinVector::random(1000);
            let m = BinMatrix::identity(1000);
            let _i = m * v;
        }
    }

}
