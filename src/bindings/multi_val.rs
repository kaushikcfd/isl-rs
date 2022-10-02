use libc::uintptr_t;
use std::os::raw::c_char;
use crate::bindings::Context;
use std::ffi::{CString, CStr};

pub struct Val {
    pub ptr: uintptr_t,
}

extern "C" {

    fn isl_val_zero(ctx: uintptr_t) -> uintptr_t;

    fn isl_val_one(ctx: uintptr_t) -> uintptr_t;

    fn isl_val_negone(ctx: uintptr_t) -> uintptr_t;

    fn isl_val_nan(ctx: uintptr_t) -> uintptr_t;

    fn isl_val_infty(ctx: uintptr_t) -> uintptr_t;

    fn isl_val_neginfty(ctx: uintptr_t) -> uintptr_t;

    fn isl_val_int_from_si(ctx: uintptr_t, i: i64) -> uintptr_t;

    fn isl_val_int_from_ui(ctx: uintptr_t, u: u64) -> uintptr_t;

    fn isl_val_copy(v: uintptr_t) -> uintptr_t;

    fn isl_val_free(v: uintptr_t) -> uintptr_t;

    fn isl_val_get_ctx(val: uintptr_t) -> uintptr_t;

    fn isl_val_get_hash(val: uintptr_t) -> u32;

    fn isl_val_get_num_si(v: uintptr_t) -> i64;

    fn isl_val_get_den_si(v: uintptr_t) -> i64;

    fn isl_val_get_den_val(v: uintptr_t) -> uintptr_t;

    fn isl_val_get_d(v: uintptr_t) -> f64;

    fn isl_val_n_abs_num_chunks(v: uintptr_t, size: usize) -> i32;

    fn isl_val_set_si(v: uintptr_t, i: i64) -> uintptr_t;

    fn isl_val_abs(v: uintptr_t) -> uintptr_t;

    fn isl_val_neg(v: uintptr_t) -> uintptr_t;

    fn isl_val_inv(v: uintptr_t) -> uintptr_t;

    fn isl_val_floor(v: uintptr_t) -> uintptr_t;

    fn isl_val_ceil(v: uintptr_t) -> uintptr_t;

    fn isl_val_trunc(v: uintptr_t) -> uintptr_t;

    fn isl_val_2exp(v: uintptr_t) -> uintptr_t;

    fn isl_val_pow2(v: uintptr_t) -> uintptr_t;

    fn isl_val_min(v1: uintptr_t, v2: uintptr_t) -> uintptr_t;

    fn isl_val_max(v1: uintptr_t, v2: uintptr_t) -> uintptr_t;

    fn isl_val_add(v1: uintptr_t, v2: uintptr_t) -> uintptr_t;

    fn isl_val_add_ui(v1: uintptr_t, v2: u64) -> uintptr_t;

    fn isl_val_sub(v1: uintptr_t, v2: uintptr_t) -> uintptr_t;

    fn isl_val_sub_ui(v1: uintptr_t, v2: u64) -> uintptr_t;

    fn isl_val_mul(v1: uintptr_t, v2: uintptr_t) -> uintptr_t;

    fn isl_val_mul_ui(v1: uintptr_t, v2: u64) -> uintptr_t;

    fn isl_val_div(v1: uintptr_t, v2: uintptr_t) -> uintptr_t;

    fn isl_val_div_ui(v1: uintptr_t, v2: u64) -> uintptr_t;

    fn isl_val_mod(v1: uintptr_t, v2: uintptr_t) -> uintptr_t;

    fn isl_val_gcd(v1: uintptr_t, v2: uintptr_t) -> uintptr_t;

    fn isl_val_sgn(v: uintptr_t) -> i32;

    fn isl_val_is_zero(v: uintptr_t) -> i32;

    fn isl_val_is_one(v: uintptr_t) -> i32;

    fn isl_val_is_negone(v: uintptr_t) -> i32;

    fn isl_val_is_nonneg(v: uintptr_t) -> i32;

    fn isl_val_is_nonpos(v: uintptr_t) -> i32;

    fn isl_val_is_pos(v: uintptr_t) -> i32;

    fn isl_val_is_neg(v: uintptr_t) -> i32;

    fn isl_val_is_int(v: uintptr_t) -> i32;

    fn isl_val_is_rat(v: uintptr_t) -> i32;

    fn isl_val_is_nan(v: uintptr_t) -> i32;

    fn isl_val_is_infty(v: uintptr_t) -> i32;

    fn isl_val_is_neginfty(v: uintptr_t) -> i32;

    fn isl_val_cmp_si(v: uintptr_t, i: i64) -> i32;

    fn isl_val_lt(v1: uintptr_t, v2: uintptr_t) -> i32;

    fn isl_val_le(v1: uintptr_t, v2: uintptr_t) -> i32;

    fn isl_val_gt(v1: uintptr_t, v2: uintptr_t) -> i32;

    fn isl_val_gt_si(v: uintptr_t, i: i64) -> i32;

    fn isl_val_ge(v1: uintptr_t, v2: uintptr_t) -> i32;

    fn isl_val_eq(v1: uintptr_t, v2: uintptr_t) -> i32;

    fn isl_val_eq_si(v: uintptr_t, i: i64) -> i32;

    fn isl_val_ne(v1: uintptr_t, v2: uintptr_t) -> i32;

    fn isl_val_abs_eq(v1: uintptr_t, v2: uintptr_t) -> i32;

    fn isl_val_is_divisible_by(v1: uintptr_t, v2: uintptr_t) -> i32;

    fn isl_val_read_from_str(ctx: uintptr_t, str_: *const c_char) -> uintptr_t;

    fn isl_val_dump(v: uintptr_t);

    fn isl_val_to_str(v: uintptr_t) -> *const c_char;

}

impl Val {
    fn zero(ctx: &Context) -> Val {
        let ctx = ctx.ptr;
        let isl_rs_result = unsafe { isl_val_zero(ctx) };
        let isl_rs_result = Val { ptr: isl_rs_result };
        isl_rs_result
    }

    fn one(ctx: &Context) -> Val {
        let ctx = ctx.ptr;
        let isl_rs_result = unsafe { isl_val_one(ctx) };
        let isl_rs_result = Val { ptr: isl_rs_result };
        isl_rs_result
    }

    fn negone(ctx: &Context) -> Val {
        let ctx = ctx.ptr;
        let isl_rs_result = unsafe { isl_val_negone(ctx) };
        let isl_rs_result = Val { ptr: isl_rs_result };
        isl_rs_result
    }

    fn nan(ctx: &Context) -> Val {
        let ctx = ctx.ptr;
        let isl_rs_result = unsafe { isl_val_nan(ctx) };
        let isl_rs_result = Val { ptr: isl_rs_result };
        isl_rs_result
    }

    fn infty(ctx: &Context) -> Val {
        let ctx = ctx.ptr;
        let isl_rs_result = unsafe { isl_val_infty(ctx) };
        let isl_rs_result = Val { ptr: isl_rs_result };
        isl_rs_result
    }

    fn neginfty(ctx: &Context) -> Val {
        let ctx = ctx.ptr;
        let isl_rs_result = unsafe { isl_val_neginfty(ctx) };
        let isl_rs_result = Val { ptr: isl_rs_result };
        isl_rs_result
    }

    fn int_from_si(ctx: &Context, i: i64) -> Val {
        let ctx = ctx.ptr;
        let isl_rs_result = unsafe { isl_val_int_from_si(ctx, i) };
        let isl_rs_result = Val { ptr: isl_rs_result };
        isl_rs_result
    }

    fn int_from_ui(ctx: &Context, u: u64) -> Val {
        let ctx = ctx.ptr;
        let isl_rs_result = unsafe { isl_val_int_from_ui(ctx, u) };
        let isl_rs_result = Val { ptr: isl_rs_result };
        isl_rs_result
    }

    fn copy(&self) -> Val {
        let v = self;
        let v = v.ptr;
        let isl_rs_result = unsafe { isl_val_copy(v) };
        let isl_rs_result = Val { ptr: isl_rs_result };
        isl_rs_result
    }

    fn free(self) -> Val {
        let v = self;
        let v = v.ptr;
        let isl_rs_result = unsafe { isl_val_free(v) };
        let isl_rs_result = Val { ptr: isl_rs_result };
        isl_rs_result
    }

    fn get_ctx(&self) -> Context {
        let val = self;
        let val = val.ptr;
        let isl_rs_result = unsafe { isl_val_get_ctx(val) };
        let isl_rs_result = Context { ptr: isl_rs_result };
        isl_rs_result
    }

    fn get_hash(&self) -> u32 {
        let val = self;
        let val = val.ptr;
        let isl_rs_result = unsafe { isl_val_get_hash(val) };
        isl_rs_result
    }

    fn get_num_si(&self) -> i64 {
        let v = self;
        let v = v.ptr;
        let isl_rs_result = unsafe { isl_val_get_num_si(v) };
        isl_rs_result
    }

    fn get_den_si(&self) -> i64 {
        let v = self;
        let v = v.ptr;
        let isl_rs_result = unsafe { isl_val_get_den_si(v) };
        isl_rs_result
    }

    fn get_den_val(&self) -> Val {
        let v = self;
        let v = v.ptr;
        let isl_rs_result = unsafe { isl_val_get_den_val(v) };
        let isl_rs_result = Val { ptr: isl_rs_result };
        isl_rs_result
    }

    fn get_d(&self) -> f64 {
        let v = self;
        let v = v.ptr;
        let isl_rs_result = unsafe { isl_val_get_d(v) };
        isl_rs_result
    }

    fn n_abs_num_chunks(&self, size: usize) -> i32 {
        let v = self;
        let v = v.ptr;
        let isl_rs_result = unsafe { isl_val_n_abs_num_chunks(v, size) };
        isl_rs_result
    }

    fn set_si(self, i: i64) -> Val {
        let v = self;
        let v = v.ptr;
        let isl_rs_result = unsafe { isl_val_set_si(v, i) };
        let isl_rs_result = Val { ptr: isl_rs_result };
        isl_rs_result
    }

    fn abs(self) -> Val {
        let v = self;
        let v = v.ptr;
        let isl_rs_result = unsafe { isl_val_abs(v) };
        let isl_rs_result = Val { ptr: isl_rs_result };
        isl_rs_result
    }

    fn neg(self) -> Val {
        let v = self;
        let v = v.ptr;
        let isl_rs_result = unsafe { isl_val_neg(v) };
        let isl_rs_result = Val { ptr: isl_rs_result };
        isl_rs_result
    }

    fn inv(self) -> Val {
        let v = self;
        let v = v.ptr;
        let isl_rs_result = unsafe { isl_val_inv(v) };
        let isl_rs_result = Val { ptr: isl_rs_result };
        isl_rs_result
    }

    fn floor(self) -> Val {
        let v = self;
        let v = v.ptr;
        let isl_rs_result = unsafe { isl_val_floor(v) };
        let isl_rs_result = Val { ptr: isl_rs_result };
        isl_rs_result
    }

    fn ceil(self) -> Val {
        let v = self;
        let v = v.ptr;
        let isl_rs_result = unsafe { isl_val_ceil(v) };
        let isl_rs_result = Val { ptr: isl_rs_result };
        isl_rs_result
    }

    fn trunc(self) -> Val {
        let v = self;
        let v = v.ptr;
        let isl_rs_result = unsafe { isl_val_trunc(v) };
        let isl_rs_result = Val { ptr: isl_rs_result };
        isl_rs_result
    }

    fn 2exp(self) -> Val {
        let v = self;
        let v = v.ptr;
        let isl_rs_result = unsafe { isl_val_2exp(v) };
        let isl_rs_result = Val { ptr: isl_rs_result };
        isl_rs_result
    }

    fn pow2(self) -> Val {
        let v = self;
        let v = v.ptr;
        let isl_rs_result = unsafe { isl_val_pow2(v) };
        let isl_rs_result = Val { ptr: isl_rs_result };
        isl_rs_result
    }

    fn min(self, v2: Val) -> Val {
        let v1 = self;
        let v1 = v1.ptr;
        let v2 = v2.ptr;
        let isl_rs_result = unsafe { isl_val_min(v1, v2) };
        let isl_rs_result = Val { ptr: isl_rs_result };
        isl_rs_result
    }

    fn max(self, v2: Val) -> Val {
        let v1 = self;
        let v1 = v1.ptr;
        let v2 = v2.ptr;
        let isl_rs_result = unsafe { isl_val_max(v1, v2) };
        let isl_rs_result = Val { ptr: isl_rs_result };
        isl_rs_result
    }

    fn add(self, v2: Val) -> Val {
        let v1 = self;
        let v1 = v1.ptr;
        let v2 = v2.ptr;
        let isl_rs_result = unsafe { isl_val_add(v1, v2) };
        let isl_rs_result = Val { ptr: isl_rs_result };
        isl_rs_result
    }

    fn add_ui(self, v2: u64) -> Val {
        let v1 = self;
        let v1 = v1.ptr;
        let isl_rs_result = unsafe { isl_val_add_ui(v1, v2) };
        let isl_rs_result = Val { ptr: isl_rs_result };
        isl_rs_result
    }

    fn sub(self, v2: Val) -> Val {
        let v1 = self;
        let v1 = v1.ptr;
        let v2 = v2.ptr;
        let isl_rs_result = unsafe { isl_val_sub(v1, v2) };
        let isl_rs_result = Val { ptr: isl_rs_result };
        isl_rs_result
    }

    fn sub_ui(self, v2: u64) -> Val {
        let v1 = self;
        let v1 = v1.ptr;
        let isl_rs_result = unsafe { isl_val_sub_ui(v1, v2) };
        let isl_rs_result = Val { ptr: isl_rs_result };
        isl_rs_result
    }

    fn mul(self, v2: Val) -> Val {
        let v1 = self;
        let v1 = v1.ptr;
        let v2 = v2.ptr;
        let isl_rs_result = unsafe { isl_val_mul(v1, v2) };
        let isl_rs_result = Val { ptr: isl_rs_result };
        isl_rs_result
    }

    fn mul_ui(self, v2: u64) -> Val {
        let v1 = self;
        let v1 = v1.ptr;
        let isl_rs_result = unsafe { isl_val_mul_ui(v1, v2) };
        let isl_rs_result = Val { ptr: isl_rs_result };
        isl_rs_result
    }

    fn div(self, v2: Val) -> Val {
        let v1 = self;
        let v1 = v1.ptr;
        let v2 = v2.ptr;
        let isl_rs_result = unsafe { isl_val_div(v1, v2) };
        let isl_rs_result = Val { ptr: isl_rs_result };
        isl_rs_result
    }

    fn div_ui(self, v2: u64) -> Val {
        let v1 = self;
        let v1 = v1.ptr;
        let isl_rs_result = unsafe { isl_val_div_ui(v1, v2) };
        let isl_rs_result = Val { ptr: isl_rs_result };
        isl_rs_result
    }

    fn mod_(self, v2: Val) -> Val {
        let v1 = self;
        let v1 = v1.ptr;
        let v2 = v2.ptr;
        let isl_rs_result = unsafe { isl_val_mod(v1, v2) };
        let isl_rs_result = Val { ptr: isl_rs_result };
        isl_rs_result
    }

    fn gcd(self, v2: Val) -> Val {
        let v1 = self;
        let v1 = v1.ptr;
        let v2 = v2.ptr;
        let isl_rs_result = unsafe { isl_val_gcd(v1, v2) };
        let isl_rs_result = Val { ptr: isl_rs_result };
        isl_rs_result
    }

    fn sgn(&self) -> i32 {
        let v = self;
        let v = v.ptr;
        let isl_rs_result = unsafe { isl_val_sgn(v) };
        isl_rs_result
    }

    fn is_zero(&self) -> bool {
        let v = self;
        let v = v.ptr;
        let isl_rs_result = unsafe { isl_val_is_zero(v) };
        let isl_rs_result = match isl_rs_result {
            0 => false,
            1 => true,
            _ => panic!("Got isl_bool = -1"),
        };
        isl_rs_result
    }

    fn is_one(&self) -> bool {
        let v = self;
        let v = v.ptr;
        let isl_rs_result = unsafe { isl_val_is_one(v) };
        let isl_rs_result = match isl_rs_result {
            0 => false,
            1 => true,
            _ => panic!("Got isl_bool = -1"),
        };
        isl_rs_result
    }

    fn is_negone(&self) -> bool {
        let v = self;
        let v = v.ptr;
        let isl_rs_result = unsafe { isl_val_is_negone(v) };
        let isl_rs_result = match isl_rs_result {
            0 => false,
            1 => true,
            _ => panic!("Got isl_bool = -1"),
        };
        isl_rs_result
    }

    fn is_nonneg(&self) -> bool {
        let v = self;
        let v = v.ptr;
        let isl_rs_result = unsafe { isl_val_is_nonneg(v) };
        let isl_rs_result = match isl_rs_result {
            0 => false,
            1 => true,
            _ => panic!("Got isl_bool = -1"),
        };
        isl_rs_result
    }

    fn is_nonpos(&self) -> bool {
        let v = self;
        let v = v.ptr;
        let isl_rs_result = unsafe { isl_val_is_nonpos(v) };
        let isl_rs_result = match isl_rs_result {
            0 => false,
            1 => true,
            _ => panic!("Got isl_bool = -1"),
        };
        isl_rs_result
    }

    fn is_pos(&self) -> bool {
        let v = self;
        let v = v.ptr;
        let isl_rs_result = unsafe { isl_val_is_pos(v) };
        let isl_rs_result = match isl_rs_result {
            0 => false,
            1 => true,
            _ => panic!("Got isl_bool = -1"),
        };
        isl_rs_result
    }

    fn is_neg(&self) -> bool {
        let v = self;
        let v = v.ptr;
        let isl_rs_result = unsafe { isl_val_is_neg(v) };
        let isl_rs_result = match isl_rs_result {
            0 => false,
            1 => true,
            _ => panic!("Got isl_bool = -1"),
        };
        isl_rs_result
    }

    fn is_int(&self) -> bool {
        let v = self;
        let v = v.ptr;
        let isl_rs_result = unsafe { isl_val_is_int(v) };
        let isl_rs_result = match isl_rs_result {
            0 => false,
            1 => true,
            _ => panic!("Got isl_bool = -1"),
        };
        isl_rs_result
    }

    fn is_rat(&self) -> bool {
        let v = self;
        let v = v.ptr;
        let isl_rs_result = unsafe { isl_val_is_rat(v) };
        let isl_rs_result = match isl_rs_result {
            0 => false,
            1 => true,
            _ => panic!("Got isl_bool = -1"),
        };
        isl_rs_result
    }

    fn is_nan(&self) -> bool {
        let v = self;
        let v = v.ptr;
        let isl_rs_result = unsafe { isl_val_is_nan(v) };
        let isl_rs_result = match isl_rs_result {
            0 => false,
            1 => true,
            _ => panic!("Got isl_bool = -1"),
        };
        isl_rs_result
    }

    fn is_infty(&self) -> bool {
        let v = self;
        let v = v.ptr;
        let isl_rs_result = unsafe { isl_val_is_infty(v) };
        let isl_rs_result = match isl_rs_result {
            0 => false,
            1 => true,
            _ => panic!("Got isl_bool = -1"),
        };
        isl_rs_result
    }

    fn is_neginfty(&self) -> bool {
        let v = self;
        let v = v.ptr;
        let isl_rs_result = unsafe { isl_val_is_neginfty(v) };
        let isl_rs_result = match isl_rs_result {
            0 => false,
            1 => true,
            _ => panic!("Got isl_bool = -1"),
        };
        isl_rs_result
    }

    fn cmp_si(&self, i: i64) -> i32 {
        let v = self;
        let v = v.ptr;
        let isl_rs_result = unsafe { isl_val_cmp_si(v, i) };
        isl_rs_result
    }

    fn lt(&self, v2: &Val) -> bool {
        let v1 = self;
        let v1 = v1.ptr;
        let v2 = v2.ptr;
        let isl_rs_result = unsafe { isl_val_lt(v1, v2) };
        let isl_rs_result = match isl_rs_result {
            0 => false,
            1 => true,
            _ => panic!("Got isl_bool = -1"),
        };
        isl_rs_result
    }

    fn le(&self, v2: &Val) -> bool {
        let v1 = self;
        let v1 = v1.ptr;
        let v2 = v2.ptr;
        let isl_rs_result = unsafe { isl_val_le(v1, v2) };
        let isl_rs_result = match isl_rs_result {
            0 => false,
            1 => true,
            _ => panic!("Got isl_bool = -1"),
        };
        isl_rs_result
    }

    fn gt(&self, v2: &Val) -> bool {
        let v1 = self;
        let v1 = v1.ptr;
        let v2 = v2.ptr;
        let isl_rs_result = unsafe { isl_val_gt(v1, v2) };
        let isl_rs_result = match isl_rs_result {
            0 => false,
            1 => true,
            _ => panic!("Got isl_bool = -1"),
        };
        isl_rs_result
    }

    fn gt_si(&self, i: i64) -> bool {
        let v = self;
        let v = v.ptr;
        let isl_rs_result = unsafe { isl_val_gt_si(v, i) };
        let isl_rs_result = match isl_rs_result {
            0 => false,
            1 => true,
            _ => panic!("Got isl_bool = -1"),
        };
        isl_rs_result
    }

    fn ge(&self, v2: &Val) -> bool {
        let v1 = self;
        let v1 = v1.ptr;
        let v2 = v2.ptr;
        let isl_rs_result = unsafe { isl_val_ge(v1, v2) };
        let isl_rs_result = match isl_rs_result {
            0 => false,
            1 => true,
            _ => panic!("Got isl_bool = -1"),
        };
        isl_rs_result
    }

    fn eq(&self, v2: &Val) -> bool {
        let v1 = self;
        let v1 = v1.ptr;
        let v2 = v2.ptr;
        let isl_rs_result = unsafe { isl_val_eq(v1, v2) };
        let isl_rs_result = match isl_rs_result {
            0 => false,
            1 => true,
            _ => panic!("Got isl_bool = -1"),
        };
        isl_rs_result
    }

    fn eq_si(&self, i: i64) -> bool {
        let v = self;
        let v = v.ptr;
        let isl_rs_result = unsafe { isl_val_eq_si(v, i) };
        let isl_rs_result = match isl_rs_result {
            0 => false,
            1 => true,
            _ => panic!("Got isl_bool = -1"),
        };
        isl_rs_result
    }

    fn ne(&self, v2: &Val) -> bool {
        let v1 = self;
        let v1 = v1.ptr;
        let v2 = v2.ptr;
        let isl_rs_result = unsafe { isl_val_ne(v1, v2) };
        let isl_rs_result = match isl_rs_result {
            0 => false,
            1 => true,
            _ => panic!("Got isl_bool = -1"),
        };
        isl_rs_result
    }

    fn abs_eq(&self, v2: &Val) -> bool {
        let v1 = self;
        let v1 = v1.ptr;
        let v2 = v2.ptr;
        let isl_rs_result = unsafe { isl_val_abs_eq(v1, v2) };
        let isl_rs_result = match isl_rs_result {
            0 => false,
            1 => true,
            _ => panic!("Got isl_bool = -1"),
        };
        isl_rs_result
    }

    fn is_divisible_by(&self, v2: &Val) -> bool {
        let v1 = self;
        let v1 = v1.ptr;
        let v2 = v2.ptr;
        let isl_rs_result = unsafe { isl_val_is_divisible_by(v1, v2) };
        let isl_rs_result = match isl_rs_result {
            0 => false,
            1 => true,
            _ => panic!("Got isl_bool = -1"),
        };
        isl_rs_result
    }

    fn read_from_str(ctx: &Context, str_: &str) -> Val {
        let ctx = ctx.ptr;
        let str_ = CString::new(str_).unwrap();
        let str_ = str_.as_ptr();
        let isl_rs_result = unsafe { isl_val_read_from_str(ctx, str_) };
        let isl_rs_result = Val { ptr: isl_rs_result };
        isl_rs_result
    }

    fn dump(&self) {
        let v = self;
        let v = v.ptr;
        let isl_rs_result = unsafe { isl_val_dump(v) };
        isl_rs_result
    }

    fn to_str(&self) -> &str {
        let v = self;
        let v = v.ptr;
        let isl_rs_result = unsafe { isl_val_to_str(v) };
        let isl_rs_result = unsafe { CStr::from_ptr(isl_rs_result) };
        let isl_rs_result = isl_rs_result.to_str().unwrap();
        isl_rs_result
    }
}

impl Drop for Val {
    fn drop(&mut self) {
        unsafe { isl_val_free(self.ptr); }
    }
}