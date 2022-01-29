#![no_std]
#![no_main]
#![feature(abi_efiapi)]

use core::panic::PanicInfo;
use uefi_lemola_os::uefi::*;


use utf16_literal::utf16;
#[no_mangle]
pub extern "C" fn efi_main(_image_handle: EfiHandle, system_table: *mut  EfiSystemTable) {
    unsafe {
        let output_protocol = (*system_table).con_out;
        ((*output_protocol).reset)(output_protocol, true);
        ((*output_protocol).output_string)(output_protocol, utf16!("Hello World from Rust!\n").as_ptr());
    }
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop{}
}
