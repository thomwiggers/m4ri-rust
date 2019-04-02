//! References `ple.h`

use crate::misc::Rci;
use crate::mzd::Mzd;
use crate::mzp::Mzp;
use libc;

extern "C" {

    /// PLUQ matrix decomposition.
    ///
    /// Returns (P,L,U,Q) satisfying PLUQ = A where P and Q are two
    /// permutation matrices, of dimension respectively m x m and n x n, L
    /// is m x r unit lower triangular and U is r x n upper triangular.
    ///
    /// P and Q must be preallocated but they don't have to be
    /// identity permutations. If cutoff is zero a value is chosen
    /// automatically. It is recommended to set cutoff to zero for most
    /// applications.
    ///
    /// The row echelon form (not reduced) can be read from the upper
    /// triangular matrix U. See mzd_echelonize_pluq() for details.
    ///
    /// This is the wrapper function including bounds checks. See
    /// _mzd_pluq() for implementation details.
    ///
    /// A Input m x n matrix
    /// P Output row permutation of length m
    /// Q Output column permutation matrix of length n
    /// cutoff Minimal dimension for Strassen recursion.
    ///
    /// See also `_mzd_pluq()` `_mzd_pluq_mmpf()` `mzd_echelonize_pluq()`
    ///
    /// return Rank of A.
    pub fn mzd_pluq(a: *mut Mzd, p: *mut Mzp, q: *mut Mzp, cutoff: libc::c_int) -> Rci;

    /// PLE matrix decomposition.
    ///
    /// Computes the PLE matrix decomposition using a block recursive
    /// algorithm.
    ///
    /// Returns (P,L,S,Q) satisfying PLE = A where P is a permutation matrix
    /// of dimension m x m, L is m x r unit lower triangular and S is an r
    /// x n matrix which is upper triangular except that its columns are
    /// permuted, that is S = UQ for U r x n upper triangular and Q is a n
    /// x n permutation matrix. The matrix L and S are stored in place over
    /// A.
    ///
    /// P and Q must be preallocated but they don't have to be
    /// identity permutations. If cutoff is zero a value is chosen
    /// automatically. It is recommended to set cutoff to zero for most
    /// applications.
    ///
    /// This is the wrapper function including bounds checks. See
    /// `_mzd_ple()` for implementation details.
    ///
    /// A Input m x n matrix
    /// P Output row permutation of length m
    /// Q Output column permutation matrix of length n
    /// cutoff Minimal dimension for Strassen recursion.
    ///
    /// See also `_mzd_ple()` `_mzd_pluq()` `_mzd_pluq_mmpf()` `mzd_echelonize_pluq()`
    ///
    /// Returns Rank of A.
    pub fn mzd_ple(a: *mut Mzd, p: *mut Mzp, q: *mut Mzp, cutoff: libc::c_int) -> Rci;

}
