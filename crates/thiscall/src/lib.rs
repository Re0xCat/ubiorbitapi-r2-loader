extern "C" {
    fn get_this_ptr() -> u32;
    fn set_this_ptr(ptr: u32);
}

#[inline]
pub fn get_this_ptr_cxx() -> u32 {
    unsafe {
        return get_this_ptr();
    }
}

#[inline]
pub fn set_this_ptr_cxx(ptr: u32) {
    unsafe {
        return set_this_ptr(ptr);
    }
}
