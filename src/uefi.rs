use core::ffi::c_void;

type CHAR16 = u16;
pub type EfiStatus = usize;
// *void
pub type EfiHandle = *mut c_void;

#[repr(C)]
pub struct EfiTableHeader {
    pub signature: u64,
    pub revision: u32,
    pub header_size: u32,
    pub crc32: u32,
    reserved: u32,
}

#[repr(C)]
pub struct EfiSystemTable {
    pub hdr: EfiTableHeader,
    pub firmware_vendor: *mut CHAR16,
    pub firmware_revision: u32,
    pub console_in_handle: EfiHandle,
    pub con_in: *mut EfiSimpleTextInputProtocol,
    pub console_out_handle: EfiHandle,
    pub con_out: *mut EfiSimpleTextOutputProtocol,
    pub standerd_error_handle: EfiHandle,
    pub std_err: *mut EfiSimpleTextOutputProtocol,
    pub runtime_services: *mut EfiRuntimeServices,
    pub boot_services: *mut EfiBootServices,
    pub number_of_table_entries: usize,
    pub configuration_table: *mut EfiConfigurationTable,
}

type EfiGuid = u128;

#[repr(C)]
pub struct EfiConfigurationTable {
    vendor_guid: EfiGuid,
    pub vendor_table: *mut c_void,
}

type FnPtr = u64;

#[repr(C)]
pub struct EfiRuntimeServices {
    pub hdr: EfiTableHeader,
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
pub struct EfiBootServices {
    pub hdr: EfiTableHeader,
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
pub struct EfiSimpleTextOutputProtocol {
    pub reset: extern "efiapi" fn(&Self, bool) -> EfiStatus,
    pub output_string: extern "efiapi" fn(&Self, *const CHAR16) -> EfiStatus,
    query_mode: FnPtr,
    set_mode: FnPtr,
    set_attribute: FnPtr,
    pub clear_screen: extern "efiapi" fn(&Self) -> EfiStatus,
    set_cursor_position: FnPtr,
    enable_cursor: FnPtr,
    pub mode: *mut SimpleTextOutputMode,
}

#[repr(C)]
pub struct SimpleTextOutputMode {
    pub max_mode: i32,
    // current setting
    pub mode: i32,
    pub attribute: i32,
    pub cursor_column: i32,
    pub cursor_row: i32,
    pub cursor_visible: bool,
}

#[repr(C)]
pub struct EfiSimpleTextInputProtocol {
    pub reset: extern "efiapi" fn(&Self, bool) -> EfiStatus,
    pub read_key_stroke: extern "efiapi" fn(&Self, *mut EfiInputKey) -> EfiStatus,
    pub wait_for_key: *mut c_void,
}

impl EfiSimpleTextOutputProtocol {
    pub fn output_string(&self, msg: &str) {
        use heapless::*;
        use heapless::consts::*;
        (self.output_string)(self, msg.encode_utf16().collect::<Vec<u16, U128>>().as_ptr());
    }
}

#[repr(C)]
pub struct EfiInputKey {
    pub scan_code: u16,
    pub unicode_char: CHAR16,
}

use core::cell::Cell;
use core::fmt::Error;

pub static mut WRITER: Writer = Writer {output_protocol: Cell::new(None)};

pub struct Writer {
    pub output_protocol: Cell<Option<&'static EfiSimpleTextOutputProtocol>>,
}

unsafe impl Sync for Writer {}

impl core::fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        if let Some(output_protcol) = self.output_protocol.get() {
            output_protcol.output_string(s);
            return Ok(());
        }
        Err(Error)
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::uefi::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: core::fmt::Arguments) {
    use core::fmt::Write;
    unsafe {
        WRITER.write_fmt(args).unwrap();
    }
}
