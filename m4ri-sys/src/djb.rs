//! Corresponds to djb.h
use crate::misc::Rci;
use crate::mzd::Mzd;

#[repr(C)]
pub struct Djb {
    private: [u8; 0],
}

#[repr(C)]
pub enum Srctyp {
    /// Add from target matrix
    SourceTarget,
    /// Add from source matrix
    SourceSource,
}

extern "C" {
    /// Allocate a new DJB linear map
    ///
    /// nrows: number of rows
    /// ncols: Number of columns
    pub fn djb_init(nrows: Rci, ncols: Rci) -> *mut Djb;

    /// free a DJB linear map
    pub fn djb_free(m: *mut Djb);

    /// Add a new operation ``out[target] ^= srctype[source]`` to queue
    ///
    /// z: DJB linear map
    /// target: output index
    /// source: input index
    /// srctyp: Type of input (source_source or source_target)
    pub fn djb_push_back(z: *mut Djb, target: Rci, source: Rci, srctyp: Srctyp);

    /// Compile an new DJB linear map from A
    ///
    /// param: A
    pub fn djb_compile(a: *mut Mzd) -> *mut Djb;

    /// apply the linear map m to V and write the result in W
    ///
    /// z: DJB linear map
    /// W: output matrix
    /// V: input matrix
    pub fn djb_apply_mzd(z: *mut Djb, w: *mut Mzd, v: *const Mzd);

    /// Print information on linear map mA
    pub fn djb_info(z: *const Djb);
}
