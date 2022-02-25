use std::arch::asm;

#[no_mangle]
#[inline(never)]
pub extern "C" fn get_this_ptr_cxx() -> u32 {
    let this: u32;

    unsafe {
        asm!("mov {0}, ecx", out(reg) this);
    }

    this
}

#[no_mangle]
#[inline(never)]
pub extern "C" fn set_this_ptr_cxx(this: u32) {
    unsafe {
        asm!("mov ecx, {0}", in(reg) this);
    }
}
