use core::ffi::c_void;
use core::fmt::Error;

use crate::dyn_utf16_ptr;
use crate::guid::*;
use crate::println;
use crate::protocols::EfiGraphicsOutputProtocol;
use crate::uefi_utils::MemoryDescriptorArray;
use crate::uefi_utils::MemoryMap;
use crate::unwrap_success;

type CHAR16 = u16;
pub type EfiStatus = usize;
// *void
pub type EfiHandle = *mut c_void;

#[repr(C)]
#[derive(Debug)]
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

impl EfiSystemTable {
    pub fn get_boot_services(&self) -> &EfiBootServices {
        unsafe { self.boot_services.as_ref().unwrap() }
    }

    pub fn output_protocol(&self) -> &EfiSimpleTextOutputProtocol {
        unsafe { self.con_out.as_ref().unwrap() }
    }
}

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
    allocate_pages: extern "efiapi" fn(
        type_: u32,
        memory_type: u32,
        pages: u64,
        memory: *const u64,
    ) -> EfiStatus,
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
    // Image Services
    load_image: FnPtr,
    start_image: FnPtr,
    exit: FnPtr,
    unload_image: FnPtr,
    exit_boot_services: extern "efiapi" fn(image_handle: EfiHandle, map_key: usize) -> EfiStatus,
    // Miscellaneous Services
    get_next_monotonic_count: FnPtr,
    stall: FnPtr,
    set_watchdog_timer: FnPtr,
    // DriverSupport Services
    connect_controller: FnPtr,
    disconnect_controller: FnPtr,
    // Open and Close Protocol Services
    open_protocol: FnPtr,
    close_protocol: FnPtr,
    open_protocol_information: FnPtr,
    // Library Services
    protocols_per_handle: FnPtr,
    locate_handle_buffer: FnPtr,
    locate_protocol: extern "efiapi" fn(
        protocol: &EfiGuid,
        registration: *const c_void,
        interface: &*const c_void,
    ) -> EfiStatus,
    install_multiple_protocol_interfaces: FnPtr,
    uninstall_multiple_protocol_interfaces: FnPtr,
    // 32-bit CRC Services
    calculate_crc32: FnPtr,
    // Miscellaneous Services
    copy_mem: FnPtr,
    set_mem: FnPtr,
    create_event_ex: FnPtr,
}

#[derive(Copy, Clone, Debug)]
pub enum EfiAllocateType {
    AllocateAnyPages = 0,
    AllocateMaxAddress = 1,
    AllocateAddress = 2,
    MaxAllocateType = 3,
}

impl Into<u32> for EfiAllocateType {
    fn into(self) -> u32 {
        match self {
            EfiAllocateType::AllocateAnyPages => 0,
            EfiAllocateType::AllocateMaxAddress => 1,
            EfiAllocateType::AllocateAddress => 2,
            EfiAllocateType::MaxAllocateType => 3,
        }
    }
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
    clear_screen: extern "efiapi" fn(&Self) -> EfiStatus,
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
    pub fn get_memory_map(&self, size: usize, map: &mut MemoryMap) -> Result<EfiStatusCode, &str> {
        map.memory_map_size = size;
        let status = (self.get_memory_map)(
            &mut map.memory_map_size,
            map.memory_map,
            &mut map.map_key,
            &mut map.descriptor_size,
            &mut map.descriptor_version,
        );
        if EfiStatusCode::try_from(status).unwrap() == EfiStatusCode::EfiBufferTooSmall {
            println!("buffer too small");
        }
        Ok(status.try_into().unwrap())
    }

    pub fn allocate_pages(
        &self,
        type_: EfiAllocateType,
        memory_type: MemoryType,
        pages: u64,
        memory: *const u64,
    ) -> EfiStatusCode {
        let status: EfiStatusCode =
            (self.allocate_pages)(type_.into(), memory_type.into(), pages, memory)
                .try_into()
                .unwrap();
        println!("{:?}", status);
        assert!(status.is_success());
        status
    }

    pub fn get_memory_descriptor_array<T>(
        &self,
        memmap_buf_ptr: *mut T,
        size: usize,
    ) -> MemoryDescriptorArray {
        let mut map = MemoryMap::new(memmap_buf_ptr, size);
        let status = self.get_memory_map(size, &mut map).unwrap();
        unwrap_success!(status);

        MemoryDescriptorArray::new(
            memmap_buf_ptr,
            map.descriptor_size,
            map.memory_map_size,
            map.map_key,
        )
    }

    pub fn exit_boot_services(
        &self,
        image_handle: EfiHandle,
        map_key: usize,
    ) -> Result<EfiStatusCode, Error> {
        let status = (self.exit_boot_services)(image_handle, map_key);
        let status: EfiStatusCode = status.try_into().unwrap();
        assert!(status.is_success());
        Ok(status)
    }

    pub fn graphics_output_protocol(&self) -> &EfiGraphicsOutputProtocol {
        let ptr = core::ptr::null();
        (self.locate_protocol)(&EFI_GRAPHICS_OUTPUT_PROTOCOL_GUID, core::ptr::null(), &ptr);
        unsafe {
            ptr.cast::<EfiGraphicsOutputProtocol>()
                .as_ref()
                .expect("provided pointer was null")
        }
    }

    pub fn locate_protocol<T: HasGuid>(&self) -> &T {
        let ptr = core::ptr::null();
        println!("{:X?}", T::get_guid());
        (self.locate_protocol)(T::get_guid(), core::ptr::null(), &ptr);
        unsafe { ptr.cast::<T>().as_ref().expect("provided pointer was null") }
    }
}

impl EfiSimpleTextOutputProtocol {
    pub fn output_string(&self, msg: &str) -> EfiStatusCode {
        let status = (self.output_string)(
            self,
            // msg.encode_utf16().collect::<Vec<u16, U4096>>().as_ptr(),
            dyn_utf16_ptr!(msg),
        );
        status.try_into().unwrap()
    }

    pub fn output_string_utf16(&self, msg_ptr: *const u16) -> EfiStatusCode {
        let status: EfiStatusCode = (self.output_string)(self, msg_ptr).try_into().unwrap();
        assert!(status.is_success());
        status
    }

    pub fn reset(&self, b: bool) -> EfiStatusCode {
        let status = (self.reset)(self, b);
        println!("reset: {:?}", EfiStatusCode::try_from(status));
        status.try_into().unwrap()
    }

    pub fn clear_screen(&self) -> EfiStatusCode {
        let status = (self.clear_screen)(&self);
        status.try_into().unwrap()
    }

    pub fn enable_cursor(&self, b: bool) {
        (self.enable_cursor)(self, b);
    }

    pub fn change_column(&self) {
        let column;
        let row;
        unsafe {
            column = (*(self.mode)).cursor_column as usize;
            row = (*(self.mode)).cursor_row as usize;
        }
        (self.set_cursor_position)(self, column + 1, row);
    }
}

#[repr(C)]
pub struct EfiInputKey {
    pub scan_code: u16,
    pub unicode_char: CHAR16,
}

#[repr(C)]
#[derive(Debug)]
pub struct EfiTime {
    year: u16,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
    second: u8,
    pad1: u8,
    nanosecond: u32,
    time_zone: u16,
    day_light: u8,
    pad2: u8,
}

#[derive(Debug, PartialEq)]
pub enum EfiStatusCode {
    EfiSuccess,
    EfiLoadError,
    EfiInvalidParameter,
    EfiUnsupported,
    EfiBadBufferSize,
    EfiBufferTooSmall,
    EfiNotReady,
    EfiDeviceError,
    EfiWriteProtected,
    EfiOutOfResources,
    EfiVolumeCorrupted,
    EfiVolumeFull,
    EfiNoMedia,
    EfiMediaChanged,
    EfiNotFound,
    EfiAccessDenied,
    EfiNoResponse,
    EfiNoMapping,
    EfiTimeout,
    EfiNotStarted,
    EfiAlreadyStarted,
    EfiAborted,
    EfiIcmpError,
    EfiTftpError,
    EfiProtocolError,
    EfiIncompatibleVersion,
    EfiSecurityViolation,
    EfiCrcError,
    EfiEndOfMedia,
    EfiEndOfFile,
    EfiInvalidLanguage,
    EfiCompromisedData,
    EfiIpAddressConflict,
    EfiHttpError,
    EfiWarnUnknownGlyph,
    EfiWarnDeleteFailure,
    EfiWarnWriteFailure,
    EfiWarnBufferTooSmall,
    EfiWarnStaleData,
    EfiWarnFileSystem,
    EfiWarnResetRequired,
}

impl EfiStatusCode {
    pub fn is_err(&self) -> bool {
        use EfiStatusCode::*;
        match self {
            EfiSuccess => false,
            EfiLoadError => true,
            EfiInvalidParameter => true,
            EfiUnsupported => true,
            EfiBadBufferSize => true,
            EfiBufferTooSmall => true,
            EfiNotReady => true,
            EfiDeviceError => true,
            EfiWriteProtected => true,
            EfiOutOfResources => true,
            EfiVolumeCorrupted => true,
            EfiVolumeFull => true,
            EfiNoMedia => true,
            EfiMediaChanged => true,
            EfiNotFound => true,
            EfiAccessDenied => true,
            EfiNoResponse => true,
            EfiNoMapping => true,
            EfiTimeout => true,
            EfiNotStarted => true,
            EfiAlreadyStarted => true,
            EfiAborted => true,
            EfiIcmpError => true,
            EfiTftpError => true,
            EfiProtocolError => true,
            EfiIncompatibleVersion => true,
            EfiSecurityViolation => true,
            EfiCrcError => true,
            EfiEndOfMedia => true,
            EfiEndOfFile => true,
            EfiInvalidLanguage => true,
            EfiCompromisedData => true,
            EfiIpAddressConflict => true,
            EfiHttpError => true,
            EfiWarnUnknownGlyph => false,
            EfiWarnDeleteFailure => false,
            EfiWarnWriteFailure => false,
            EfiWarnBufferTooSmall => false,
            EfiWarnStaleData => false,
            EfiWarnFileSystem => false,
            EfiWarnResetRequired => false,
        }
    }

    pub fn is_success(&self) -> bool {
        matches!(self, &EfiStatusCode::EfiSuccess)
    }

    pub fn unwrap_success(&self) -> Self {
        if let Self::EfiSuccess = *self {
            Self::EfiSuccess
        } else {
            panic!("EfiStatus: {:?}", self);
        }
    }
}

impl TryFrom<EfiStatus> for EfiStatusCode {
    type Error = Error;

    fn try_from(value: EfiStatus) -> Result<Self, Self::Error> {
        use EfiStatusCode::*;
        let status = if value << 8 != 0 {
            match (value << 8) >> 8 {
                1 => EfiLoadError,
                2 => EfiInvalidParameter,
                3 => EfiUnsupported,
                4 => EfiBadBufferSize,
                5 => EfiBufferTooSmall,
                6 => EfiNotReady,
                7 => EfiDeviceError,
                8 => EfiWriteProtected,
                9 => EfiOutOfResources,
                10 => EfiVolumeCorrupted,
                11 => EfiVolumeFull,
                12 => EfiNoMedia,
                13 => EfiMediaChanged,
                14 => EfiNotFound,
                15 => EfiAccessDenied,
                16 => EfiNoResponse,
                17 => EfiNoMapping,
                18 => EfiTimeout,
                19 => EfiNotStarted,
                20 => EfiAlreadyStarted,
                21 => EfiAborted,
                22 => EfiIcmpError,
                23 => EfiTftpError,
                24 => EfiProtocolError,
                25 => EfiIncompatibleVersion,
                26 => EfiSecurityViolation,
                27 => EfiCrcError,
                28 => EfiEndOfMedia,
                31 => EfiEndOfFile,
                32 => EfiInvalidLanguage,
                33 => EfiCompromisedData,
                34 => EfiIpAddressConflict,
                35 => EfiHttpError,
                _ => return Err(Error),
            }
        } else {
            match value {
                0 => EfiSuccess,
                1 => EfiWarnUnknownGlyph,
                2 => EfiWarnDeleteFailure,
                3 => EfiWarnWriteFailure,
                4 => EfiBufferTooSmall,
                5 => EfiWarnStaleData,
                6 => EfiWarnFileSystem,
                7 => EfiWarnResetRequired,
                _ => return Err(Error),
            }
        };

        Ok(status)
    }
}

#[derive(Debug, PartialEq)]
pub enum MemoryType {
    EfiReservedMemoryType,
    EfiLoaderCode,
    EfiLoaderData,
    EfiBootServicesCode,
    EfiBootServicesData,
    EfiRuntimeServicesCode,
    EfiRuntimeServicesData,
    EfiConventionalMemory,
    EfiUnusableMemory,
    EfiACPIRecaimMemory,
    EfiACPIMemoryNVS,
    EfiMemoryMappedIO,
    EfiMemoryMappedIOPortSpace,
    EfiPalCode,
    EfiPersistentMemory,
    EfiUnacceptedMemoryType,
    EfiMaxMemoryType,
}

impl TryFrom<u32> for MemoryType {
    type Error = Error;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        use crate::uefi::MemoryType::*;
        let mem_type = match value {
            0 => EfiReservedMemoryType,
            1 => EfiLoaderCode,
            2 => EfiLoaderData,
            3 => EfiBootServicesCode,
            4 => EfiBootServicesData,
            5 => EfiRuntimeServicesCode,
            6 => EfiRuntimeServicesData,
            7 => EfiConventionalMemory,
            8 => EfiUnusableMemory,
            9 => EfiACPIRecaimMemory,
            10 => EfiACPIMemoryNVS,
            11 => EfiMemoryMappedIO,
            12 => EfiMemoryMappedIOPortSpace,
            13 => EfiPalCode,
            14 => EfiPersistentMemory,
            15 => EfiUnacceptedMemoryType,
            16 => EfiMaxMemoryType,
            _ => return Err(Error),
        };
        Ok(mem_type)
    }
}

impl Into<u32> for MemoryType {
    fn into(self) -> u32 {
        match self {
            MemoryType::EfiReservedMemoryType => 0,
            MemoryType::EfiLoaderCode => 1,
            MemoryType::EfiLoaderData => 2,
            MemoryType::EfiBootServicesCode => 3,
            MemoryType::EfiBootServicesData => 4,
            MemoryType::EfiRuntimeServicesCode => 5,
            MemoryType::EfiRuntimeServicesData => 6,
            MemoryType::EfiConventionalMemory => 7,
            MemoryType::EfiUnusableMemory => 8,
            MemoryType::EfiACPIRecaimMemory => 9,
            MemoryType::EfiACPIMemoryNVS => 10,
            MemoryType::EfiMemoryMappedIO => 11,
            MemoryType::EfiMemoryMappedIOPortSpace => 12,
            MemoryType::EfiPalCode => 13,
            MemoryType::EfiPersistentMemory => 14,
            MemoryType::EfiUnacceptedMemoryType => 15,
            MemoryType::EfiMaxMemoryType => 16,
        }
    }
}
