#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
include!(concat!(env!("OUT_DIR"),"/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;

    #[test]
    fn init() {
        let c_str_program = CString::new("zroot").unwrap();
        let c_str_pool = CString::new("zroot").unwrap();

        let instrlimit: u64 = 10 * 1000 * 1000;

        let memlimit: u64 = 10 * 1024 * 1024;

        let nvl_ptr: *mut nvlist_t = unsafe { fnvlist_alloc() };

        unsafe {
            fnvlist_add_string(
                nvl_ptr, CString::new("1").unwrap().as_ptr(), c_str_pool.as_ptr());
        };

        let outnvl_ptr_ptr: *mut *mut nvlist_t = &mut unsafe { fnvlist_alloc() };

        unsafe {
            let lzc = libzfs_core_init();
        }
    }

}
