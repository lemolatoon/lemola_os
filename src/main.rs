#![no_std]
#![no_main]
#![feature(abi_efiapi)]

use core::panic::PanicInfo;
use core::ptr;
use uefi_lemola_os::{uefi::*, uefi_utils::{self, *}};
use uefi_lemola_os::println;
use uefi_lemola_os::print;


use utf16_literal::utf16;
#[no_mangle]
pub extern "C" fn efi_main(_image_handle: EfiHandle, system_table: *mut  EfiSystemTable) {
    let output_protocol = unsafe { &*(*system_table).con_out };
    unsafe {
        WRITER.output_protocol.set(Some(output_protocol));
    }
    (output_protocol.reset)(output_protocol, true);
    output_protocol.enable_cursor(true);
    (output_protocol.output_string)(output_protocol, utf16!("Hello World from Rust").as_ptr());
    output_protocol.change_column();
    println!("{}", utf16!("Hello World from Rust").len());
    println!("Is this ok?");
    println!("改行されてますか？");
    unsafe {
        WRITER.output_protocol.get().unwrap().output_string("Hello from WRITER");
        WRITER.output_protocol.get().unwrap().output_string("????");
    }
    use heapless::*;
    use heapless::consts::U128;
    println!("{}", utf16!("\n")[0] == "\n".encode_utf16().collect::<Vec<u16, U128>>()[0]);
    println!("{}", utf16!("\n")[0]);
    println!("Hello World from macro");

    let mut map;
    const SIZE: usize = 4096 * 4;
    use core::mem::MaybeUninit;
    let mut memmap_buf: MaybeUninit<[u8; SIZE]> = MaybeUninit::uninit(); 
    let boot_services = unsafe {system_table.as_ref().unwrap().boot_services.as_ref().unwrap() };
    let mut memmap_buf_inited;
    unsafe {
        memmap_buf_inited = memmap_buf.assume_init();
        map = MemoryMap::new(&mut memmap_buf_inited);
        boot_services.get_memory_map(&mut memmap_buf_inited, &mut map).unwrap();
    }
    println!("{:?}", map);
    let mut i = 0;
    unsafe {
        let mut mem_desc = map.memory_map.as_ref().unwrap();
        while mem_desc.number_of_pages != 0 {
            i = i + 1;
            mem_desc = map.memory_map.cast::<u8>().add(map.descriptor_size as usize * i).cast::<EfiMemoryDescriptor>().as_ref().unwrap();
            use uefi_utils::MemoryType::*;
            match MemoryType::try_from(mem_desc.type_).unwrap() {
                EfiConventionalMemory => println!("{}", mem_desc),
                _ => ()
            }
        }
    }

    loop {}
}

fn get_memory_map(map: &mut MemoryMap ,get_map: extern "efiapi" fn(&mut usize, *mut EfiMemoryDescriptor, &mut usize, &mut usize, &mut u32) -> EfiStatus) -> Result<EfiStatus, ()> {
    let status;
    status = get_map(&mut map.memory_map_size, map.memory_map, &mut map.map_key, &mut map.descriptor_size, &mut map.descriptor_version);
    Ok(status)
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{:?}", info);
    loop {}
}
