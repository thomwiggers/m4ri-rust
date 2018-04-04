/// Links to mzd.h
///
/// FIXME implement missing functions
use libc;
use std::mem::size_of;

use ffi::misc::BIT;
use ffi::misc::Rci;
use ffi::misc::Wi;
use ffi::misc::Word;
use ffi::misc::m4ri_one;
use ffi::misc::m4ri_radix;

/// Represents the blocks used by M4RI internally
#[repr(C)]
struct MzdBlock {
    private: [u8; 0],
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

extern "C" {
    /// Create a new rows x columns matrix
    pub fn mzd_init(rows: Rci, columns: Rci) -> *mut Mzd;

    /// Free a matrix created with mzd_init.
    /// Automatically done by the Deref trait on Mzd
    fn mzd_free(matrix: *mut Mzd);

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
    /// \param lowc Starting column (inclusive, must be multiple of m4ri_radix)
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

    /// Free a matrix window created with mzd_init_window
    pub fn mzd_free_window(matrix: *mut Mzd);

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
    pub fn mzd_copy_row(b: *mut Mzd, a: *const Mzd, j: Rci);

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
    pub fn mzd_add(c: *mut Mzd, a: *const Mzd, b: *const Mzd);

    /// Set C = A - B
    /// If C is passed in, the result is written there
    /// otherwise a new matrix is created
    ///
    /// Secretly an alias for mzd_add
    pub fn mzd_sub(c: *mut Mzd, a: *const Mzd, b: *const Mzd);

    /// Zero test for matrix
    pub fn mzd_is_zero(a: *const Mzd);

}

/// Write the bit to position M[row, col]
#[inline]
pub unsafe fn mzd_write_bit(matrix: *mut Mzd, row: Rci, col: Rci, value: BIT) {
    println!("({}, {}) = {}", row, col, value);
    let therow: *const *mut Word = (*matrix).rows.offset(row as isize);
    let column: *mut Word = (*therow).offset((col / m4ri_radix) as isize);
    let pos = col % m4ri_radix;
    let column_bitmasked: Word = *column & (m4ri_one << pos);
    let column_newbit: Word = (value as Word & m4ri_one) << pos;
    debug_assert!(column_newbit.count_ones() <= 1);
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

impl Drop for Mzd {
    fn drop(&mut self) {
        unsafe {
            mzd_free(self);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::mem;
    use std::ptr;

    #[test]
    fn init() {
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
        }
    }
}
