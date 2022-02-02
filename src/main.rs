#![no_std]
#![no_main]
#![feature(abi_efiapi)]

use core::panic::PanicInfo;
use uefi_lemola_os::utils::loop_with_hlt;
use uefi_lemola_os::{mem_desc, println};
use uefi_lemola_os::{uefi::*, uefi_utils::*};

use utf16_literal::utf16;
#[no_mangle]
pub extern "C" fn efi_main(_image_handle: EfiHandle, system_table: &'static mut EfiSystemTable) {
    let output_protocol = &system_table.con_out;
    unsafe {
        WRITER.output_protocol.set(Some(output_protocol));
    }
    (output_protocol.reset)(output_protocol, true);
    output_protocol.enable_cursor(true);
    (output_protocol.output_string)(output_protocol, utf16!("Hello World from Rust").as_ptr());
    println!("{}", utf16!("Hello World from Rust").len());
    println!("Is this ok?");
    unsafe {
        WRITER
            .output_protocol
            .get()
            .unwrap()
            .output_string("Hello from WRITER");
        WRITER.output_protocol.get().unwrap().output_string("????");
    }
    use heapless::consts::U128;
    use heapless::*;
    println!(
        "{}",
        utf16!("\n")[0] == "\n".encode_utf16().collect::<Vec<u16, U128>>()[0]
    );
    println!("{}", utf16!("\n")[0]);
    println!("Hello World from macro");

    let boot_services = system_table.get_boot_services();

    let mem_desc_array = mem_desc!(boot_services);

    let mut i = 0;
    while let Some(mem_desc) = mem_desc_array.get(i) {
        println!("{}", mem_desc);
        i += 1;
    }

    loop_with_hlt();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{:?}", info);
    loop_with_hlt()
}
