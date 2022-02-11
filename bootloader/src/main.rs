#![no_std]
#![no_main]
#![feature(abi_efiapi)]

use core::panic::PanicInfo;
use uefi_lemola_os::dbg;
use uefi_lemola_os::protocols::*;
use uefi_lemola_os::utils::loop_with_hlt;
use uefi_lemola_os::{mem_desc, println};
use uefi_lemola_os::{uefi::*, uefi_utils::*};

#[no_mangle]
pub extern "C" fn efi_main(image_handle: EfiHandle, system_table: &'static EfiSystemTable) {
    init(system_table);
    println!("Hello World from macro");

    let boot_services = system_table.get_boot_services();

    let mem_desc_array = mem_desc!(boot_services);

    use uefi_lemola_os::uefi::MemoryType::*;
    let iter = mem_desc_array
        .iter()
        .filter(|desc| MemoryType::try_from(desc.type_).unwrap() == EfiConventionalMemory);

    for desc in iter {
        println!("{}", desc);
    }

    let protocol = boot_services.locate_protocol::<EfiGraphicsOutputProtocol>();
    // let protocol = locate_protocol!(boot_services, get_guid!(EfiGraphicsOutputProtocol));
    println!("{:?}", protocol);
    let base_addr = protocol.mode.frame_buffer_base;
    let size = protocol.mode.frame_buffer_size;
    for i in 0..size / 4 {
        unsafe {
            *((base_addr as *mut u8).add(i)) = 0xff;
        }
    }
    dbg!(base_addr);

    let status = system_table.output_protocol().clear_screen();
    println!("{:?}", status);
    let status = system_table.output_protocol().reset(true);
    println!("{:?}", status);

    let protocol = boot_services.locate_protocol::<EfiSimpleFileSystemProtocol>();
    let root_dir = protocol.root_dir();
    use uefi_lemola_os::print;
    unsafe {
        for i in 0..15 {
            print!(
                "{:X}, ",
                (root_dir as *const EfiFileProtocol)
                    .cast::<u64>()
                    .add(i)
                    .as_ref()
                    .unwrap()
            );
        }
    }
    println!();
    let protocol = root_dir.open(
        "\\kernel.elf",
        OpenMode::EfiFileModeRead,
        FileAttributes::EfiFileReadOnly,
    );
    unsafe {
        for i in 0..15 {
            print!(
                "{:X}, ",
                (protocol as *const EfiFileProtocol)
                    .cast::<u64>()
                    .add(i)
                    .as_ref()
                    .unwrap()
            );
        }
    }

    for i in size / 4..size / 5 {
        unsafe {
            *((base_addr as *mut u8).add(i)) = 0xfa;
        }
    }

    let mem_desc_array = mem_desc!(boot_services);
    let map_key = mem_desc_array.map_key();
    // There must be no stdout between get_memorymap and exit_boot_services
    let _status = boot_services
        .exit_boot_services(image_handle, map_key)
        .unwrap();

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
