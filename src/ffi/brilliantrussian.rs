use ffi::misc::Rci;
use ffi::mzd::Mzd;
/// See `brilliantrussian.h`
use libc;

extern "C" {

    /// Constructs all possible 2^k row combinations using the
    /// gray code table
    ///
    /// M: matrix to generate tables from
    /// r: starting row
    /// c: starting column
    /// k
    /// T: preallocated matrix of dimension 2^k x m->ncols
    /// L: preallocated table of length 2^k
    pub fn mzd_make_table(m: *const Mzd, r: Rci, c: Rci, k: libc::c_int, t: *mut Mzd, l: *mut Rci);

    /// The function looks up k bits from position i, startcol in each
    /// row and adds the appropriate row from T to the row i
    ///
    /// This process is iterated for i from startrow to stoprow (exclusive)
    ///
    /// M: Matrix to operate on
    /// startrow: top row which is operated on
    /// endrow: bottom row which is operated on
    /// k: M4RI parameter
    /// T: contains the correct row to be added
    /// L: contains row number to be added
    pub fn mzd_process_rows(
        m: *mut Mzd,
        startrow: Rci,
        endrow: Rci,
        startcol: Rci,
        k: libc::c_int,
        t: *const Mzd,
        l: *const Rci,
    );

    /// Same as `mzd_process_rows` but works with two Gray code
    /// tables in parallel
    ///
    /// m: Matrix to operate on
    /// startrow: top row hich is operated on
    /// startcol: starting column for addition
    /// k: M4ri param
    /// T0: contains the correct row to be added
    /// L0: contains row number to be added
    /// T1: contains the correct row to be added
    /// L1: contains the row number to be added
    pub fn mzd_process_rows2(
        m: *mut Mzd,
        startrow: Rci,
        endrow: Rci,
        startcol: Rci,
        k: libc::c_int,
        t0: *const Mzd,
        l0: *const Rci,
        t1: *const Mzd,
        l1: *const Rci,
    );

    /// Same as `mzd_process_rows` but works with three Gray code
    /// tables in parallel
    ///
    /// m: Matrix to operate on
    /// startrow: top row hich is operated on
    /// startcol: starting column for addition
    /// k: M4ri param
    /// T0: contains the correct row to be added
    /// L0: contains row number to be added
    /// T1: contains the correct row to be added
    /// L1: contains the row number to be added
    /// T2: contains the correct row to be added
    /// L2: contains the row number to be added
    pub fn mzd_process_rows3(
        m: *mut Mzd,
        startrow: Rci,
        endrow: Rci,
        startcol: Rci,
        k: libc::c_int,
        t0: *const Mzd,
        l0: *const Rci,
        t1: *const Mzd,
        l1: *const Rci,
        t2: *const Mzd,
        l2: *const Rci,
    );

    /// Same as `mzd_process_rows` but works with four Gray code
    /// tables in parallel
    ///
    /// m: Matrix to operate on
    /// startrow: top row hich is operated on
    /// startcol: starting column for addition
    /// k: M4ri param
    /// T0: contains the correct row to be added
    /// L0: contains row number to be added
    /// T1: contains the correct row to be added
    /// L1: contains the row number to be added
    /// T2: contains the correct row to be added
    /// L2: contains the row number to be added
    /// T3: contains the correct row to be added
    /// L3: contains the row number to be added
    pub fn mzd_process_rows4(
        m: *mut Mzd,
        startrow: Rci,
        endrow: Rci,
        startcol: Rci,
        k: libc::c_int,
        t0: *const Mzd,
        l0: *const Rci,
        t1: *const Mzd,
        l1: *const Rci,
        t2: *const Mzd,
        l2: *const Rci,
        t3: *const Mzd,
        l3: *const Rci,
    );

    /// Same as `mzd_process_rows` but works with five Gray code
    /// tables in parallel
    ///
    /// m: Matrix to operate on
    /// startrow: top row hich is operated on
    /// startcol: starting column for addition
    /// k: M4ri param
    /// T0: contains the correct row to be added
    /// L0: contains row number to be added
    /// T1: contains the correct row to be added
    /// L1: contains the row number to be added
    /// T2: contains the correct row to be added
    /// L2: contains the row number to be added
    /// T3: contains the correct row to be added
    /// L3: contains the row number to be added
    /// T4: contains the correct row to be added
    /// L4: contains the row number to be added
    pub fn mzd_process_rows5(
        m: *mut Mzd,
        startrow: Rci,
        endrow: Rci,
        startcol: Rci,
        k: libc::c_int,
        t0: *const Mzd,
        l0: *const Rci,
        t1: *const Mzd,
        l1: *const Rci,
        t2: *const Mzd,
        l2: *const Rci,
        t3: *const Mzd,
        l3: *const Rci,
        t4: *const Mzd,
        l4: *const Rci,
    );

    /// Same as `mzd_process_rows` but works with six Gray code
    /// tables in parallel
    ///
    /// m: Matrix to operate on
    /// startrow: top row hich is operated on
    /// startcol: starting column for addition
    /// k: M4ri param
    /// T0: contains the correct row to be added
    /// L0: contains row number to be added
    /// T1: contains the correct row to be added
    /// L1: contains the row number to be added
    /// T2: contains the correct row to be added
    /// L2: contains the row number to be added
    /// T3: contains the correct row to be added
    /// L3: contains the row number to be added
    /// T4: contains the correct row to be added
    /// L4: contains the row number to be added
    /// T5: contains the correct row to be added
    /// L5: contains the row number to be added
    pub fn mzd_process_rows6(
        m: *mut Mzd,
        startrow: Rci,
        endrow: Rci,
        startcol: Rci,
        k: libc::c_int,
        t0: *const Mzd,
        l0: *const Rci,
        t1: *const Mzd,
        l1: *const Rci,
        t2: *const Mzd,
        l2: *const Rci,
        t3: *const Mzd,
        l3: *const Rci,
        t4: *const Mzd,
        l4: *const Rci,
        t5: *const Mzd,
        l5: *const Rci,
    );

    /// Matrix elimination using the Method of the four russians (m4ri)
    ///
    /// M: Matrix to be reduced
    /// k: M4ri parameter, may be 0 for auto-choose
    pub fn mzd_top_echelonize_m4ri(m: *mut Mzd, k: libc::c_int);

    /// Invert the matrix using Konrod's method
    ///
    /// dst: Matrix to hold the inverse (may be Null)
    /// src: Matrix to be inverted
    /// k: table size parameter, set 0 for automatic choice
    ///
    /// Return inverse of src if src has full rank
    pub fn mzd_inv_m4ri(dst: *mut Mzd, src: *const Mzd, k: libc::c_int) -> *mut Mzd;

    /// Matrix multiplication using Konrods Method
    ///
    /// c: preallocated product matrix, may be NULL for automatic creation
    /// a: input matrix
    /// b: input matrix
    /// k: M4RI parameter, may be 0 for automatic choice
    pub fn mzd_mul_m4rm(c: *mut Mzd, a: *const Mzd, b: *const Mzd, k: libc::c_int) -> *mut Mzd;

    /// Set C to C + AB using Konrods Method
    ///
    /// c: input and result matrix
    /// a: input matrix
    /// b: input matrix
    /// k: M4RI parameter, may be 0 for automatic choice
    pub fn mzd_addmul_m4rm(c: *mut Mzd, a: *const Mzd, b: *const Mzd, k: libc::c_int) -> *mut Mzd;
}
