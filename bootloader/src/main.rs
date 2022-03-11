#![no_std]
#![no_main]
#![feature(abi_efiapi)]
#![feature(maybe_uninit_uninit_array)]

use core::arch::asm;
use core::mem::size_of;
use core::mem::MaybeUninit;
use core::panic::PanicInfo;
use uefi_lemola_os::dbg;
use uefi_lemola_os::protocols::*;
use uefi_lemola_os::root_dir;
use uefi_lemola_os::unwrap_success;
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

    let status = system_table.output_protocol().reset(true);
    unwrap_success!(status);

    // start booting kernel
    let protocol = boot_services.locate_protocol::<EfiSimpleFileSystemProtocol>();
    let root_dir = MaybeUninit::<&EfiFileProtocol>::uninit();
    protocol.root_dir(&root_dir);
    let root_dir = unsafe { root_dir.assume_init() };
    println!("{:p}", root_dir);
    root_dir!(protocol, root_dir);

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
    println!("=======================================");
    let kernel_file = MaybeUninit::<&EfiFileProtocol>::uninit();
    const FILE_NAME: &str = "\\kernel.elf\0";
    let status = root_dir.open(
        &kernel_file,
        FILE_NAME,
        OpenMode::EfiFileModeRead,
        FileAttributes::EfiFileReadOnly,
    );
    println!("{:?}", status);
    unwrap_success!(status);
    let kernel_file = unsafe { kernel_file.assume_init() };
    assert_ne!(
        root_dir as *const EfiFileProtocol,
        kernel_file as *const EfiFileProtocol
    );
    unsafe {
        for i in 0..15 {
            print!(
                "{:X}, ",
                (kernel_file as *const EfiFileProtocol)
                    .cast::<u64>()
                    .add(i)
                    .as_ref()
                    .unwrap()
            );
        }
    }
    // const FILE_NAME: &str = "\\a.txt\0";
    const FILE_NAME_LEN: usize = FILE_NAME.len();
    assert_eq!(FILE_NAME_LEN, 12);
    const FILE_INFO_SIZE: usize = size_of::<EfiFileInfo>() + size_of::<u16>() * FILE_NAME_LEN;
    let file_info_buffer: [MaybeUninit<u8>; FILE_INFO_SIZE] = MaybeUninit::uninit_array();
    let mut buffer_size = FILE_INFO_SIZE;
    let status = kernel_file.get_info(&mut buffer_size, &file_info_buffer);
    unwrap_success!(status);
    let file_info = unsafe {
        file_info_buffer
            .as_ptr()
            .cast::<EfiFileInfo>()
            .as_ref()
            .expect("FileInfo was null")
    };

    println!("{:X?}", file_info);
    loop_with_hlt();
    // TODO: utf8でencodeされてしまっているから、utf16でget_info関数にデータを渡せるように修正する
    let c = file_info.filename;
    // let str: [u16; 12] = unsafe { *((c as *const u16).cast::<[u16; 12]>()) };
    // println!("{:X?}", str);
    let size = file_info.size;
    println!(
        "file_info.size: {}, size_of::<EfiFileInfo>: {}",
        file_info.size,
        size_of::<EfiFileInfo>()
    );

    let kernel_file_size = file_info.file_size;

    let status = system_table.output_protocol().reset(true);
    unwrap_success!(status);
    //
    let protocol = system_table
        .get_boot_services()
        .locate_protocol::<EfiGraphicsOutputProtocol>();
    let base_addr = protocol.mode.frame_buffer_base;
    let size = protocol.mode.frame_buffer_size;
    println!("base: {:X}, size: {:X}", base_addr, size);
    /*
    for i in 0..size / 4 {
        unsafe {
            *((base_addr as *mut u8).add(i)) = 0xff;
        }
    }
    */

    let kernel_base_addr: u64 = 0x100000;
    let kernel_entry_addr: u64 = 0x101120;
    let status = boot_services.allocate_pages(
        EfiAllocateType::AllocateAddress,
        EfiLoaderData,
        (kernel_file_size + 0xfff) / 0x1000,
        &kernel_base_addr,
    );
    unwrap_success!(status);
    let kernel_file_size = kernel_file_size as usize;
    dbg!("before read");
    let status = (kernel_file.read)(
        kernel_file,
        &kernel_file_size,
        (kernel_base_addr as *const u8).cast(),
    );
    dbg!("after read");
    let status: EfiStatusCode = status.try_into().unwrap();
    unwrap_success!(status);

    let kernel_entry_addr: u64 = kernel_base_addr + 24;
    let kernel_entry_addr = 0x101130;
    let f: extern "C" fn() = unsafe { core::mem::transmute(kernel_entry_addr as *const u8) };
    println!("{:p}", f as *const u8);
    println!("\n{}", base_addr);
    println!("{}", size);
    dbg!("before jump");
    let mem_desc_array = mem_desc!(boot_services);
    for desc in mem_desc_array.iter() {
        if desc.physical_start + desc.number_of_pages * 1024 > 0x72a401c {
            break;
        }
        println!("{}", desc);
    }
    let mem_desc_array = mem_desc!(boot_services);
    let map_key = mem_desc_array.map_key();
    // There must be no stdout between get_memorymap and exit_boot_services
    let _status = boot_services.exit_boot_services(image_handle, map_key);
    //let res = f();
    loop {
        unsafe { asm!("add rax, 1") }
    }
    dbg!("after jump");
    // assert_eq!(res, 32);

    let mem_desc_array = mem_desc!(boot_services);
    let map_key = mem_desc_array.map_key();
    // There must be no stdout between get_memorymap and exit_boot_services
    let _status = boot_services
        .exit_boot_services(image_handle, map_key)
        .unwrap();

    loop {}
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
