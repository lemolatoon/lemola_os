#![no_std]
#![no_main]
use core::arch::asm;
use core::panic::PanicInfo;

#[no_mangle]
extern "C" fn kernel_main() {
    let base_addr = 1921024;
    let size = 1921024;
    for i in 0..size / 4 {
        unsafe {
            *((base_addr as *mut u8).add(i)) = 0xff;
        }
    }
    loop {
        unsafe { asm!("hlt") };
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        unsafe { asm!("hlt") };
    }
}
