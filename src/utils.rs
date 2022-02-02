use core::arch::asm;

pub fn loop_with_hlt() -> ! {
    loop {
        unsafe {
            asm! {"hlt"};
        }
    }
}
