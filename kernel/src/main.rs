#![no_std]
#![no_main]
use core::arch::asm;
use core::panic::PanicInfo;

#[no_mangle]
extern "C" fn kernel_main() -> usize {
    loop {
        unsafe { asm!("hlt") };
    }
    let base_addr: u64 = 80000000;
    let size = 0x1D5000;
    for i in 0..size / 4 {
        unsafe {
            *((base_addr as *mut u8).add(i)) = 0xff;
        }
    }
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        unsafe { asm!("hlt") };
    }
}
