#![no_std]
#![no_main]
#![feature(abi_efiapi)]

use core::panic::PanicInfo;
use uefi_lemola_os::utils::loop_with_hlt;
use uefi_lemola_os::{mem_desc, println};
use uefi_lemola_os::{uefi::*, uefi_utils::*};

#[no_mangle]
pub extern "C" fn efi_main(image_handle: EfiHandle, system_table: &'static EfiSystemTable) {
    init(system_table);
    println!("Hello World from macro");

    let boot_services = system_table.get_boot_services();

    let mem_desc_array = mem_desc!(boot_services);

    let mut i = 0;
    while let Some(mem_desc) = mem_desc_array.get(i) {
        println!("{}", mem_desc);
        i += 1;
    }

    let status = boot_services
        .exit_boot_services(image_handle, mem_desc_array.map_key())
        .unwrap();

    println!("{:?}", status);

    loop_with_hlt();
}

fn init(system_table: &'static EfiSystemTable) {
    let output_protocol = system_table.output_protocol();
    unsafe {
        WRITER.output_protocol.set(Some(output_protocol));
    }
    output_protocol.reset(true);
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{:?}", info);
    loop_with_hlt()
}
