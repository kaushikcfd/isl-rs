use libc::uintptr_t;

pub struct MultiId {
    pub ptr: uintptr_t,
}

extern "C" {

}

impl MultiId {
}

impl Drop for MultiId {
    fn drop(&mut self) {
        unsafe { isl_mutli_id_free(self.ptr); }
    }
}