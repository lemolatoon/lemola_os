use core::ffi::c_void;

use crate::uefi_utils::MemoryDescriptorArray;
use crate::uefi_utils::MemoryType;
use crate::{print, uefi_utils::MemoryMap};

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
    pub con_out: &'static mut EfiSimpleTextOutputProtocol,
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
    pub get_memory_map: extern "efiapi" fn(
        memory_map_size: &mut usize,
        memory_map: *mut EfiMemoryDescriptor,
        map_key: &mut usize,
        descriptor_size: &mut usize,
        descriptor_version: &mut u32,
    ) -> EfiStatus,
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
#[derive(Debug)]
pub struct EfiMemoryDescriptor {
    pub type_: u32,
    pub physical_start: u64,
    pub virtual_start: u64,
    pub number_of_pages: u64,
    pub attribute: u64,
}

impl core::fmt::Display for EfiMemoryDescriptor {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_fmt(format_args!(
            "{{ addr: [ {:#010x} - {:#010x} ], memory_type: {:?} }}",
            self.physical_start,
            self.physical_start + self.number_of_pages * 4 * 1024 - 1,
            MemoryType::try_from(self.type_)?
        ))?;
        Ok(())
    }
}

#[repr(C)]
pub struct EfiSimpleTextOutputProtocol {
    pub reset: extern "efiapi" fn(&Self, bool) -> EfiStatus,
    pub output_string: extern "efiapi" fn(&Self, *const CHAR16) -> EfiStatus,
    query_mode: extern "efiapi" fn(&Self, usize, *mut usize, *mut usize) -> EfiStatus,
    set_mode: FnPtr,
    set_attribute: FnPtr,
    pub clear_screen: extern "efiapi" fn(&Self) -> EfiStatus,
    pub set_cursor_position: extern "efiapi" fn(&Self, usize, usize) -> EfiStatus,
    enable_cursor: extern "efiapi" fn(&Self, bool) -> EfiStatus,
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

impl EfiBootServices {
    pub fn get_memory_map(&self, size: usize, map: &mut MemoryMap) -> Result<EfiStatus, &str> {
        if size == 0 {
            return Err("TOO SMALL BUFFER SIZE");
        }
        map.memory_map_size = size;
        let status = (&self.get_memory_map)(
            &mut map.memory_map_size,
            map.memory_map,
            &mut map.map_key,
            &mut map.descriptor_size,
            &mut map.descriptor_version,
        );
        Ok(status)
    }

    pub fn get_memory_descriptor_array<T>(
        &self,
        memmap_buf_ptr: *mut T,
        size: usize,
    ) -> MemoryDescriptorArray {
        let mut map = MemoryMap::new(memmap_buf_ptr, size);
        self.get_memory_map(size, &mut map).unwrap();

        MemoryDescriptorArray::new(memmap_buf_ptr, map.descriptor_size, map.memory_map_size)
    }
}

impl EfiSimpleTextOutputProtocol {
    pub fn output_string(&self, msg: &str) {
        use heapless::consts::*;
        use heapless::*;
        (self.output_string)(
            self,
            msg.encode_utf16().collect::<Vec<u16, U4096>>().as_ptr(),
        );
    }

    pub fn enable_cursor(&self, b: bool) {
        (self.enable_cursor)(self, b);
    }

    pub fn change_column(&self) {
        let column;
        let row;
        unsafe {
            column = (*(self.mode)).cursor_column;
            row = (*(self.mode)).cursor_row;
            print!("column: {}, row: {}", column, row);
        }
        // (self.set_cursor_position)(self, column + 1, row);
    }
}

#[repr(C)]
pub struct EfiInputKey {
    pub scan_code: u16,
    pub unicode_char: CHAR16,
}
