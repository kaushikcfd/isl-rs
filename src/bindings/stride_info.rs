// Automatically generated by isl_bindings_generator.
// LICENSE: MIT

use crate::bindings::{Aff, Context, Val};
use libc::uintptr_t;

/// Wraps `isl_stride_info`.
pub struct StrideInfo {
    pub ptr: uintptr_t,
    pub should_free_on_drop: bool,
}

extern "C" {

    fn isl_stride_info_get_ctx(si: uintptr_t) -> uintptr_t;

    fn isl_stride_info_get_stride(si: uintptr_t) -> uintptr_t;

    fn isl_stride_info_get_offset(si: uintptr_t) -> uintptr_t;

    fn isl_stride_info_free(si: uintptr_t) -> uintptr_t;

    fn isl_stride_info_copy(si: uintptr_t) -> uintptr_t;

}

impl StrideInfo {
    /// Wraps `isl_stride_info_get_ctx`.
    pub fn get_ctx(&self) -> Context {
        let si = self;
        let si = si.ptr;
        let isl_rs_result = unsafe { isl_stride_info_get_ctx(si) };
        let isl_rs_result = Context { ptr: isl_rs_result,
                                      should_free_on_drop: true };
        let mut isl_rs_result = isl_rs_result;
        isl_rs_result.do_not_free_on_drop();
        isl_rs_result
    }

    /// Wraps `isl_stride_info_get_stride`.
    pub fn get_stride(&self) -> Val {
        let si = self;
        let si = si.ptr;
        let isl_rs_result = unsafe { isl_stride_info_get_stride(si) };
        let isl_rs_result = Val { ptr: isl_rs_result,
                                  should_free_on_drop: true };
        isl_rs_result
    }

    /// Wraps `isl_stride_info_get_offset`.
    pub fn get_offset(&self) -> Aff {
        let si = self;
        let si = si.ptr;
        let isl_rs_result = unsafe { isl_stride_info_get_offset(si) };
        let isl_rs_result = Aff { ptr: isl_rs_result,
                                  should_free_on_drop: true };
        isl_rs_result
    }

    /// Wraps `isl_stride_info_free`.
    pub fn free(self) -> StrideInfo {
        let si = self;
        let mut si = si;
        si.do_not_free_on_drop();
        let si = si.ptr;
        let isl_rs_result = unsafe { isl_stride_info_free(si) };
        let isl_rs_result = StrideInfo { ptr: isl_rs_result,
                                         should_free_on_drop: true };
        isl_rs_result
    }

    /// Wraps `isl_stride_info_copy`.
    pub fn copy(&self) -> StrideInfo {
        let si = self;
        let si = si.ptr;
        let isl_rs_result = unsafe { isl_stride_info_copy(si) };
        let isl_rs_result = StrideInfo { ptr: isl_rs_result,
                                         should_free_on_drop: true };
        isl_rs_result
    }

    /// Does not call isl_xxx_free() on being dropped. (For internal use only.)
    pub fn do_not_free_on_drop(&mut self) {
        self.should_free_on_drop = false;
    }
}

impl Drop for StrideInfo {
    fn drop(&mut self) {
        if self.should_free_on_drop {
            unsafe {
                isl_stride_info_free(self.ptr);
            }
        }
    }
}
