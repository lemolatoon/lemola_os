#![no_std]
#![no_main]
#![feature(abi_efiapi)]

use core::panic::PanicInfo;
use core::ffi::c_void;

type CHAR16 = u16;
type EfiStatus = usize;
// *void
type EfiHandle = *mut c_void;

#[repr(C)]
struct EfiTableHeader {
    signature: u64,
    revision: u32,
    header_size: u32,
    crc32: u32,
    reserved: u32,
}

#[repr(C)]
pub struct EfiSystemTable {
    hdr: EfiTableHeader,
    firmware_vendor: *mut CHAR16,
    firmware_revision: u32,
    console_in_handle: EfiHandle,
    con_in: *mut EfiSimpleTextInputProtocol,
    console_out_handle: EfiHandle,
    con_out: *mut EfiSimpleTextOutputProtocol,
    standerd_error_handle: EfiHandle,
    std_err: *mut EfiSimpleTextOutputProtocol,
    runtime_services: *mut EfiRuntimeServices,
    boot_services: *mut EfiBootServices,
    number_of_table_entries: usize,
    configuration_table: *mut EfiConfigurationTable,
}

type EfiGuid = u128;

#[repr(C)]
struct EfiConfigurationTable {
    vendor_guid: EfiGuid,
    vendor_table: *mut c_void,
}

type FnPtr = u64;

#[repr(C)]
struct EfiRuntimeServices {
    hdr: EfiTableHeader,
    // Time Services
    get_time: FnPtr,
    set_time: FnPtr,
    get_wakeup_time: FnPtr,
    set_wakeup_time: FnPtr,
    // Virtual Memory Services
    set_virtual_address_map: FnPtr,
    convert_pointer: FnPtr,
    // Variable Services
    get_variable: FnPtr,
    get_next_variable_name: FnPtr,
    set_variable: FnPtr,
    // Miscellaneous Services
    get_next_high_monotonic_count: FnPtr,
    reset_system: FnPtr,
    // UEFI 2.0 Capsule Services
    update_capsule: FnPtr,
    query_capsule_capabilities: FnPtr,
    // Miscellaneous UEFI 2.0 Services
    query_variable_info: FnPtr,
}

#[repr(C)]
struct EfiBootServices {
    hdr: EfiTableHeader,
    // Task Priority Services
    raise_tpl: FnPtr,
    restore_tpl: FnPtr,
    // Memory Services
    allocate_pages: FnPtr,
    free_pages: FnPtr,
    get_memory_map: FnPtr,
    allocate_pool: FnPtr,
    free_pool: FnPtr,
    // Event & Timer Services
    create_event: FnPtr,
    set_timer: FnPtr,
    wait_for_event: FnPtr,
    signal_event: FnPtr,
    close_event: FnPtr,
    check_event: FnPtr,
    // Protocol Handler Services
    install_protocol_interface: FnPtr,
    reinstall_protocol_interface: FnPtr,
    uninstall_protocol_interface: FnPtr,
    handle_protocol: FnPtr,
    reserved: FnPtr,
    register_protocol_notify: FnPtr,
    locate_handle: FnPtr,
    locate_device_path: FnPtr,
    install_configuration_table: FnPtr,
    load_image: FnPtr,
    start_image: FnPtr,
    exit: FnPtr,
    unload_image: FnPtr,
    exit_boot_services: FnPtr,
    get_next_monotonic_count: FnPtr,
    stall: FnPtr,
    set_watchdog_timer: FnPtr,
    connect_controller: FnPtr,
    disconnect_controller: FnPtr,
    open_protocol: FnPtr,
    close_protocol: FnPtr,
    open_protocol_information: FnPtr,
    protocols_per_handle: FnPtr,
    locate_handle_buffer: FnPtr,
    locate_protocol: FnPtr,
    install_multiple_protocol_interfaces: FnPtr,
    uninstall_multiple_protocol_interfaces: FnPtr,
    calculate_crc32: FnPtr,
    copy_mem: FnPtr,
    set_mem: FnPtr,
    create_event_ex: FnPtr,
}

#[repr(C)]
struct EfiSimpleTextOutputProtocol {
    reset: extern "efiapi" fn(*mut EfiSimpleTextOutputProtocol, bool) -> EfiStatus,
    output_string: extern "efiapi" fn(*mut EfiSimpleTextOutputProtocol, *const CHAR16),
    query_mode: FnPtr,
    set_mode: FnPtr,
    set_attribute: FnPtr,
    clear_screen: FnPtr,
    set_cursor_position: FnPtr,
    enable_cursor: FnPtr,
    mode: *mut SimpleTextOutputMode,
}

#[repr(C)]
struct SimpleTextOutputMode {
    max_mode: i32,
    // current setting
    mode: i32,
    attribute: i32,
    cursor_column: i32,
    cursor_row: i32,
    cursor_visible: bool,
}

#[repr(C)]
struct EfiSimpleTextInputProtocol {
    reset: extern "efiapi" fn(*mut EfiSimpleTextInputProtocol, bool) -> EfiStatus,
    read_key_stroke: extern "efiapi" fn(*mut EfiSimpleTextInputProtocol, *mut EfiInputKey) -> EfiStatus,
    wait_for_key: *mut c_void,
}

#[repr(C)]
struct EfiInputKey {
    scan_code: u16,
    unicode_char: CHAR16,
}


use utf16_literal::utf16;
#[no_mangle]
pub extern "C" fn efi_main(image_handle: EfiHandle, system_table: *mut  EfiSystemTable) {
    unsafe {
        let output_protocol = (*system_table).con_out;
        ((*output_protocol).output_string)(output_protocol, utf16!("Hello World from Rust!\n").as_ptr());
    }
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop{}
}
