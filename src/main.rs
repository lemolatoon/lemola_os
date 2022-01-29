#![no_std]
#![no_main]
#![feature(abi_efiapi)]

use core::panic::PanicInfo;
use uefi_lemola_os::{uefi::*, println};


use utf16_literal::utf16;
#[no_mangle]
pub extern "C" fn efi_main(_image_handle: EfiHandle, system_table: *mut  EfiSystemTable) {
    let output_protocol = unsafe { &*(*system_table).con_out };
    unsafe {
        WRITER.output_protocol.set(Some(output_protocol));
    }
    (output_protocol.reset)(output_protocol, true);
    (output_protocol.output_string)(output_protocol, utf16!("Hello World from Rust").as_ptr());
    unsafe {
        WRITER.output_protocol.get().unwrap().output_string("   Hello from WRITER");
        WRITER.output_protocol.get().unwrap().output_string("????");
    }
    println!("Hello World from macro");
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop{}
}
