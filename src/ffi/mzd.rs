/// Links to mzd.h
use libc;

/// Represents the Mzd data type used by M4RI
#[repr(C)]
pub struct Mzd {
    private: [u8; 0],
}

pub type Rci = libc::c_int;
pub type BIT = libc::c_int;

#[link(name = "m4ri")]
extern "C" {
    /// Create a new rows x columns matrix
    pub fn mzd_init(rows: Rci, columns: Rci) -> *mut Mzd;

    /// Free a matrix created with mzd_init.
    pub fn mzd_free(matrix: *mut Mzd);

    /// \brief Create a window/view into the matrix M.
    ///
    /// A matrix window for M is a meta structure on the matrix M. It is
    /// setup to point into the matrix so M \em must \em not be freed while the
    /// matrix window is used.
    ///
    /// This function puts the restriction on the provided parameters that
    /// all parameters must be within range for M which is not enforced
    /// currently .
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

    /// Read the bit at position M[row, col]
    ///
    /// # Unsafe behaviour
    /// No bounds checking
    pub fn mzd_read_bit(matrix: *const Mzd, row: Rci, col: Rci) -> BIT;

    /// Write the bit to position M[row, col]
    pub fn mzd_write_bit(matrix: *const Mzd, row: Rci, col: Rci, value: BIT);

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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn init() {
        let result: libc::c_int;
        unsafe {
            let matrix = mzd_init(10, 10);
            result = mzd_equal(matrix, matrix);
            mzd_free(matrix);
        }
        assert!(result != 0);
    }
}
