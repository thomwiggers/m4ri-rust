/// System solving with matrix routines
///
/// Corresponds to m4ri/solve.h
use libc;
use ffi::mzd::Mzd;
use ffi::mzp::Mzp;
use ffi::misc::Rci;

extern "C" {
    /// Solves A X = B with A and B matrices. 
    /// 
    /// The solution X is stored inplace on B.
    /// 
    /// param A Input matrix (overwritten).
    /// param B Input matrix, being overwritten by the solution matrix X
    /// param cutoff Minimal dimension for Strassen recursion (default: 0).
    /// param inconsistency_check decide wether or not to perform a check
    ///       for incosistency (faster without but output not defined if
    ///       system is not consistent).
    /// return 0 if a solution was found, -1 otherwise
    pub fn mzd_solve_left(a: *mut Mzd, b: *mut Mzd, cutoff: libc::c_int, inconsistency_check: libc::c_int) -> libc::c_int;

    /// Solves (P L U Q) X = B
    ///
    /// A is an input matrix supposed to store both:
    ///
    ///  *  an upper right triangular matrix U
    ///  *  a lower left unitary triangular matrix L.
    ///
    /// The solution X is stored inplace on B
    ///
    /// This version assumes that the matrices are at an even position on
    /// the ``m4ri_radix`` grid and that their dimension is a multiple of m4ri_radix.
    ///
    /// param A Input upper/lower triangular matrices.
    /// param rank is rank of A.
    /// param P Input row permutation matrix.
    /// param Q Input column permutation matrix.
    /// param B Input matrix, being overwritten by the solution matrix X.
    /// param cutoff Minimal dimension for Strassen recursion (default: 0).
    /// param inconsistency_check decide whether or not to perform a check
    ///       for incosistency (faster without but output not defined if
    ///       system is not consistent).  
    ///
    /// return 0 if a solution was found, -1 otherwise
    pub fn mzd_pluq_solve_left(a: *const Mzd, rank: Rci, p: *const Mzp, q: *const Mzp, b: *mut Mzd, cutoff: libc::c_int, inconsistency_check: libc::c_int) -> libc::c_int;
}
