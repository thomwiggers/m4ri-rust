use bit_vec::BitVec;
use ffi::*;
use libc::c_int;
use std::ops;
use std::ptr;

pub struct BinMatrix {
    mzd: ptr::NonNull<Mzd>,
}

impl BinMatrix {
    pub fn new(rows: Vec<BitVec>) -> BinMatrix {
        if rows.len() == 0 {
            panic!("Can't create a 0 matrix");
        }
        let first_col_length = rows[0].len();
        if cfg!(not(ndebug)) {
            for row in rows.iter() {
                debug_assert_eq!(first_col_length, row.len());
            }
        }
        let mzd_ptr;
        unsafe {
            mzd_ptr = mzd_init(rows.len() as c_int, rows[0].len() as c_int);
        }

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

        let mzd;
        unsafe {
            mzd = ptr::NonNull::new_unchecked(mzd_ptr);
        }
        BinMatrix { mzd }
    }

    pub fn identity(rows: usize) -> BinMatrix {
        unsafe {
            let mzd_ptr = mzd_init(rows as c_int, rows as c_int);
            mzd_set_ui(mzd_ptr, 1);
            let mzd = ptr::NonNull::new_unchecked(mzd_ptr);
            BinMatrix { mzd }
        }
    }

    pub fn transpose(&self) -> BinMatrix {
        let mzd;
        unsafe {
            let mzd_ptr = mzd_transpose(ptr::null_mut(), self.mzd.as_ptr());
            mzd = ptr::NonNull::new_unchecked(mzd_ptr);
        }
        BinMatrix { mzd }
    }

    pub fn nrows(&self) -> usize {
        unsafe { self.mzd.as_ref().nrows as usize }
    }

    pub fn ncols(&self) -> usize {
        unsafe { self.mzd.as_ref().ncols as usize }
    }
}

impl ops::Mul<BinMatrix> for BinMatrix {
    type Output = BinMatrix;

    fn mul(self, other: BinMatrix) -> Self::Output {
        &self * &other
    }
}

impl<'a> ops::Mul<&'a BinMatrix> for &'a BinMatrix {
    type Output = BinMatrix;
    fn mul(self, other: &BinMatrix) -> Self::Output {
        unsafe {
            let mzd_ptr = mzd_mul(ptr::null_mut(), self.mzd.as_ptr(), other.mzd.as_ptr(), 0);

            BinMatrix {
                mzd: ptr::NonNull::new(mzd_ptr).expect("Multiplication failed"),
            }
        }
    }
}

/// Computes (v^T * A^T) (so the other way around!)
impl<'a> ops::Mul<&'a BitVec> for &'a BinMatrix {
    type Output = BitVec;
    fn mul(self, other: &BitVec) -> Self::Output {
        debug_assert_eq!(self.nrows(), other.len(), "Mismatched sizes");
        let vec_mzd;
        unsafe {
            vec_mzd = mzd_init(other.len() as Rci, 1);
            debug_assert_eq!((*vec_mzd).nrows as usize, other.len());
            debug_assert_eq!((*vec_mzd).ncols as usize, 1);
        }
        for (pos, bit) in other.iter().enumerate() {
            unsafe {
                // FIXME can be done faster
                mzd_write_bit(vec_mzd, pos as Rci, 0, bit as BIT);
            }
        }

        let mut result = BitVec::with_capacity(other.len());
        unsafe {
            let result_mzd = mzd_mul(ptr::null_mut(), self.mzd.as_ptr(), vec_mzd, 0);
            for i in 0..other.len() {
            debug_assert_eq!((*result_mzd).nrows as usize, other.len());
            debug_assert_eq!((*result_mzd).ncols as usize, 1);
            // FIXME can be done faster
                result.push(mzd_read_bit(result_mzd, i as Rci, 0) != 0);
            }
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use bit_vec::BitVec;

    #[test]
    fn new() {
        let _m = BinMatrix::new(vec![
            BitVec::from_bytes(&[0b01010101]),
            BitVec::from_bytes(&[0b01010101]),
        ]);
    }

    #[test]
    fn identity() {
        let id = BinMatrix::new(vec![
            BitVec::from_bytes(&[0b10000000]),
            BitVec::from_bytes(&[0b01000000]),
            BitVec::from_bytes(&[0b00100000]),
            BitVec::from_bytes(&[0b00010000]),
            BitVec::from_bytes(&[0b00001000]),
            BitVec::from_bytes(&[0b00000100]),
            BitVec::from_bytes(&[0b00000010]),
            BitVec::from_bytes(&[0b00000001]),
        ]);

        let id_gen = BinMatrix::identity(8);
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
        let bitvec = BitVec::from_elem(10, true);

        let result: BitVec = &m1 * &bitvec;

        assert_eq!(result, bitvec);
    }
}
