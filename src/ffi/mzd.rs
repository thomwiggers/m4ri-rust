/// Links to mzd.h
///
/// FIXME implement missing functions
use libc;
use std::mem::size_of;

use ffi::misc::m4ri_one;
use ffi::misc::m4ri_radix;
use ffi::misc::Rci;
use ffi::misc::Wi;
use ffi::misc::Word;
use ffi::misc::BIT;

/// Represents the blocks used by M4RI internally
#[repr(C)]
struct MzdBlock {
    size: libc::size_t,
    begin: *mut Word,
    end: *mut Word,
}

/// Represents the Mzd data type used by M4RI
#[repr(C)]
pub struct Mzd {
    /// Number of rows
    pub nrows: Rci,
    /// Number of columns
    pub ncols: Rci,
    /// Number of words with valid bits:
    /// width = ceil(ncols / m4ri_radix)
    pub width: Wi,

    /// Offset in words between rows
    /// ``
    /// rowstride = (width < mzd_paddingwidth || (width & 1) == 0) ? width : width + 1;
    /// ``
    /// where width is the width of the underlying non-windowed matrix
    rowstride: Wi,

    /// Offset in words from start of block to first word
    ///
    /// rows[0] = blocks[0].begin + offset_vector
    offset_vector: Wi,

    /// Number of rows to the first row counting from the start of the
    /// first block
    row_offset: Wi,

    /// Booleans to speed up things
    ///
    /// The bits have the following meaning
    ///
    /// 1: Has non-zero excess
    /// 2. Is windowed, but has zero offset
    /// 3. Is windowed, but has zero excess
    /// 4. Is windowed, but owns the Blocks allocations
    /// 5. Spans more than 1 Block
    flags: u8,

    /// blockrows_log = log2(blockrows)
    /// where blockrows is the number of rows in one block,
    /// which is a power of 2.
    blockrows_log: u8,

    // Ensures sizeof(mzd_t) == 64
    padding: [u8; 62 - 2 * size_of::<Rci>() - 4 * size_of::<Wi>() - size_of::<Word>()
        - 2 * size_of::<*const libc::c_void>()],

    /// Mask for valid bits in the word with the highest index (width - 1)
    high_bitmask: Word,
    /// Pointers to the actual blocks of memory containing the values packed into words
    blocks: *const MzdBlock,
    /// Address of first word in each row, so the first word of row [i] is in m->rows[i]
    pub rows: *const *mut Word,
}

/// Flag when `ncols%64 == 0`
pub static MZD_FLAG_NONZERO_EXCESS: u8 = 0x2;

/// Flag for windowed matrix
pub static MZD_FLAG_WINDOWED_ZEROOFFSET: u8 = 0x4;

/// Flag for windowed matrix where `ncols % 64 == 0`
pub static MZD_FLAG_WINDOWED_ZEROEXCESS: u8 = 0x8;

/// Flag for windowed matrix which owns its memory
pub static MZD_FLAG_WINDOWED_OWNSBLOCKS: u8 = 0x10;

/// Flag for multiple blocks
pub static MZD_FLAG_MULTIPLE_BLOCKS: u8 = 0x20;

extern "C" {
    /// Create a new rows x columns matrix
    pub fn mzd_init(rows: Rci, columns: Rci) -> *mut Mzd;

    /// Free a matrix created with mzd_init.
    /// Automatically done by the Deref trait on Mzd
    pub fn mzd_free(matrix: *mut Mzd);

    /// \brief Create a window/view into the matrix M.
    ///
    /// A matrix window for M is a meta structure on the matrix M. It i
    /// setup to point into the matrix so M \em must \em not be freed while the
    /// matrix window is used.
    ///
    /// This function puts the restriction on the provided parameters that
    /// all parameters must be within range for M which is not enforced
    /// currently.
    ///
    /// Use mzd_free_window to free the window.
    ///
    /// \param M Matrix
    /// \param lowr Starting row (inclusive)
    /// \param lowc Starting column (inclusive, **must be multiple of m4ri_radix**)
    /// \param highr End row (exclusive)
    /// \param highc End column (exclusive)
    pub fn mzd_init_window(
        matrix: *mut Mzd,
        lowr: Rci,
        lowc: Rci,
        highr: Rci,
        highc: Rci,
    ) -> *mut Mzd;

    /// Create a const window/view into a const matrix
    pub fn mzd_init_window_const(
        matrix: *const Mzd,
        lowr: Rci,
        lowc: Rci,
        highr: Rci,
        highc: Rci,
    ) -> *const Mzd;

    /// Swap the two rows rowa and rowb
    pub fn mzd_row_swap(matrix: *mut Mzd, rowa: Rci, rowb: Rci);

    /// \brief copy row j from A to row i from B.
    ///
    /// The offsets of A and B must match and the number of columns of A
    /// must be less than or equal to the number of columns of B.
    ///
    /// \param B Target matrix.
    /// \param i Target row index.
    /// \param A Source matrix.
    /// \param j Source row index.
    pub fn mzd_copy_row(b: *mut Mzd, i: Rci, a: *const Mzd, j: Rci);

    /// Swap the two columns cola and colb
    pub fn mzd_col_swap(matrix: *mut Mzd, cola: Rci, colb: Rci);

    /// Transpose a matrix
    /// Dest may be null for automatic creation
    pub fn mzd_transpose(dest: *mut Mzd, source: *const Mzd) -> *mut Mzd;

    /// naive cubic matrix multiplication
    /// the first argument may be null for automatic creation
    pub fn mzd_mul_naive(dest: *mut Mzd, a: *const Mzd, b: *const Mzd) -> *mut Mzd;

    /// naive cubic matrix multiplication and addition
    ///
    /// C == C + AB
    pub fn mzd_addmul_naive(c: *mut Mzd, a: *const Mzd, b: *const Mzd) -> *mut Mzd;

    /// Matrix multiplication optimized for v*A where v is a vector
    ///
    /// param C: preallocated product matrix
    /// param v: input matrix v
    /// param A: input matrix A
    /// param clear: if set clear C first, otherwise add result to C
    pub fn _mzd_mul_va(c: *mut Mzd, v: *const Mzd, a: *const Mzd, clear: libc::c_int) -> *mut Mzd;

    /// Fill the matrix m with uniformly distributed bits.
    pub fn mzd_randomize(m: *mut Mzd);

    /// Return true if A == B
    pub fn mzd_equal(a: *const Mzd, b: *const Mzd) -> libc::c_int;

    /// Copy a matrix to dest
    ///
    /// Dest may be null for automatic creation
    pub fn mzd_copy(dest: *mut Mzd, src: *const Mzd) -> *mut Mzd;

    /// Concatenate B to A and write the result to C
    pub fn mzd_concat(c: *mut Mzd, a: *const Mzd, b: *const Mzd) -> *mut Mzd;

    /// Set to identity matrix if the second argument is 1
    pub fn mzd_set_ui(a: *mut Mzd, n: libc::c_uint);

    /// Stack A on top of B into C
    pub fn mzd_stack(c: *mut Mzd, a: *mut Mzd, b: *const Mzd) -> *mut Mzd;

    /// Copy a submatrix
    /// first argument may be preallocated space or null
    pub fn mzd_submatrix(
        s: *mut Mzd,
        m: *const Mzd,
        lowr: Rci,
        lowc: Rci,
        highr: Rci,
        highc: Rci,
    ) -> *mut Mzd;

    /// Invert the target matrix using gaussian elimination
    /// To avoid recomputing the identity matrix over and over again,
    /// I may be passed in as identity parameter
    /// The first parameter may be null to have the space automatically allocated
    pub fn mzd_invert_naive(inv: *mut Mzd, a: *const Mzd, identity: *const Mzd) -> *mut Mzd;

    /// Set C = A + B
    /// If C is passed in, the result is written there
    /// otherwise a new matrix is created
    pub fn mzd_add(c: *mut Mzd, a: *const Mzd, b: *const Mzd) -> *mut Mzd;

    /// Set C = A - B
    /// If C is passed in, the result is written there
    /// otherwise a new matrix is created
    ///
    /// Secretly an alias for mzd_add
    pub fn mzd_sub(c: *mut Mzd, a: *const Mzd, b: *const Mzd) -> *mut Mzd;

    /// Zero test for matrix
    pub fn mzd_is_zero(a: *const Mzd);


    /// Clear the given row, but only begins at the column coloffset.
    ///
    /// param M Matrix
    /// param row Index of row
    /// param coloffset Column offset
    pub fn mzd_row_clear_offset(m: *mut Mzd, row: Rci, coloffset: Rci);

}

/// Write the bit to position M[row, col]
#[inline]
pub unsafe fn mzd_write_bit(matrix: *mut Mzd, row: Rci, col: Rci, value: BIT) {
    let therow: *const *mut Word = (*matrix).rows.offset(row as isize);
    let column: *mut Word = (*therow).offset((col / m4ri_radix) as isize);
    let pos = col % m4ri_radix;
    let column_bitmasked: Word = *column & !(m4ri_one << pos);
    let column_newbit: Word = (value as Word & m4ri_one) << pos;
    debug_assert_eq!(column_newbit.count_ones(), value as u32);
    *column = column_bitmasked | column_newbit;
}

/// Read the bit at position M[row, col]
///
/// # Unsafe behaviour
/// No bounds checking
///
/// Reimplemented in Rust as the C library declares it as inline
#[inline]
pub unsafe fn mzd_read_bit(matrix: *const Mzd, row: Rci, col: Rci) -> BIT {
    let therow: *const *mut Word = (*matrix).rows.offset(row as isize);
    let column: Word = *(*therow).offset((col / m4ri_radix) as isize);

    let thebit = (column >> (col % m4ri_radix)) & m4ri_one;
    thebit as BIT
}

/// Get a pointer to the first word
///
/// param: M: matrix
///
/// Return a pointer to the first word of the first row
#[inline]
pub unsafe fn mzd_first_row(matrix: *const Mzd) -> *mut Word {
    let result: *mut Word = (*(*matrix).blocks)
        .begin
        .offset((*matrix).offset_vector as isize);
    debug_assert!(
        (*matrix).nrows == 0 || result == *(*matrix).rows,
        "Result is not the expected ptr"
    );
    result
}

/// Get pointer to first word of row
///
/// Param M Matrix
/// Param row the row index
#[inline]
pub unsafe fn mzd_row(matrix: *const Mzd, row: Rci) -> *mut Word {
    let big_vector: Wi = (*matrix).offset_vector + row * (*matrix).rowstride;
    let mut result: *mut Word = (*(*matrix).blocks).begin.offset(big_vector as isize);

    // FIXME __M4RI_UNLIKELY -> _builtin_expect
    if (*matrix).flags & MZD_FLAG_MULTIPLE_BLOCKS != 0 {
        let n = ((*matrix).row_offset + row) >> (*matrix).blockrows_log;
        result = (*(*matrix).blocks.offset(n as isize)).begin.offset(
            (big_vector - n * ((*(*matrix).blocks).size / ::std::mem::size_of::<Word>()) as i32)
                as isize,
        );
    }

    debug_assert_eq!(
        result,
        *(*matrix).rows.offset(row as isize),
        "Result is not the expected ptr"
    );
    result
}

/// Test if a matrix is windowed
///
/// return a non-zero value if the matrix is windowed, otherwise return zero
#[inline]
pub unsafe fn mzd_is_windowed(m: *const Mzd) -> u8 {
    (*m).flags & MZD_FLAG_WINDOWED_ZEROOFFSET
}

/// Test if this mzd_t should free blocks
#[inline]
pub unsafe fn mzd_owns_blocks(m: *const Mzd) -> bool {
    !(*m).blocks.is_null()
        && (mzd_is_windowed(m) == 0 || ((*m).flags & MZD_FLAG_WINDOWED_OWNSBLOCKS != 0))
}

impl Drop for Mzd {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            mzd_free(self);
        }
    }
}

/// Free a matrix window created with mzd_init_window
///
/// This is actually just `mzd_free` so call `ptr::drop_in_place` instead
#[inline]
pub unsafe fn mzd_free_window(matrix: *mut Mzd) {
    mzd_free(matrix);
}

#[cfg(test)]
mod test {
    use super::*;
    use std::mem;
    use std::ptr;

    #[test]
    fn init() {
        for _ in 0..100 {
            let result: libc::c_int;
            unsafe {
                assert_eq!(mem::size_of::<Mzd>(), 64);
                let matrix = mzd_init(10, 10);
                assert!(!(*matrix).blocks.is_null());
                assert!(!(*matrix).rows.is_null());
                mzd_randomize(matrix);
                result = mzd_equal(matrix, matrix);
                assert_eq!(result, 1);
                let m2 = mzd_copy(ptr::null_mut(), matrix);
                mzd_randomize(m2);
                assert_eq!(mzd_equal(m2, matrix), 0);
                ptr::drop_in_place(matrix);
                ptr::drop_in_place(m2);
            }
        }
    }
    #[test]
    fn test_mzd_first_row() {
        for _ in 0..100 {
            unsafe {
                let matrix = mzd_init(10, 10);
                mzd_set_ui(matrix, 0);
                mzd_first_row(matrix);
                ptr::drop_in_place(matrix);
            }
        }
    }

    #[test]
    fn test_mzd_row() {
        for _ in 0..100 {
            unsafe {
                let matrix = mzd_init(10, 10);
                mzd_set_ui(matrix, 0);
                mzd_row(matrix, 5);
                ptr::drop_in_place(matrix);
            }
        }
    }

    #[test]
    fn test_mzd_read_bit() {
        for _ in 0..10 {
            unsafe {
                let matrix = mzd_init(1000, 1000);
                mzd_set_ui(matrix, 1);
                for i in 0..1000 {
                    for j in 0..1000 {
                        let bit = mzd_read_bit(matrix, i as Rci, j as Rci);
                        assert_eq!(bit == 1, i == j, "Should be unit matrix");
                    }
                }
                ptr::drop_in_place(matrix);
            }
        }
    }

    #[test]
    fn test_mzd_write_bit() {
        for _ in 0..10 {
            unsafe {
                let matrix = mzd_init(1000, 1000);
                for i in 0..1000 {
                    for j in 0..1000 {
                        mzd_write_bit(matrix, i as Rci, j as Rci, if i == j { 1 } else { 0 });
                    }
                }
                for i in 0..1000 {
                    for j in 0..1000 {
                        let bit = mzd_read_bit(matrix, i as Rci, j as Rci);
                        assert_eq!(bit == 1, i == j, "Should be unit matrix");
                    }
                }
                ptr::drop_in_place(matrix);
            }
        }
    }
}
