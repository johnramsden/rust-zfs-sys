#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
include!(concat!(env!("OUT_DIR"),"/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init() {
        unsafe {
            let lzc = libzfs_core_init();

            let hdl = libzfs_init();

            assert!(!hdl.is_null());
        }
    }

}
