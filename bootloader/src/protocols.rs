use crate::guid::*;
use core::ffi::c_void;

use crate::uefi::*;

type FnPtr = u64;
type CHAR16 = u16;

use crate::impl_guid;
impl_guid!(EfiGraphicsOutputProtocol<'_>, EFI_GRAPHICS_OUTPUT_PROTOCOL_GUID);
impl_guid!(EfiSimpleFileSystemProtocol, EFI_SIMPLE_FILE_SYSTEM_PROTOCOL_GUID);

#[repr(C)]
#[derive(Debug)]
pub struct EfiGraphicsOutputProtocol<'a> {
    pub query_mode: FnPtr,
    pub set_mode: FnPtr,
    pub blt: FnPtr,
    pub mode: &'a EfiGraphicsOutputProtocolMode<'a>,
}

type EfiPhysicalAddress = u64;

#[repr(C)]
#[derive(Debug)]
pub struct EfiGraphicsOutputProtocolMode<'a> {
    pub max_mode: u32,
    pub mode: u32,
    pub info: &'a EfiGraphicsOutputModeInformation,
    pub size_of_info: usize,
    pub frame_buffer_base: EfiPhysicalAddress,
    pub frame_buffer_size: usize,
}

#[repr(C)]
#[derive(Debug)]
pub struct EfiGraphicsOutputModeInformation {
    pub version: u32,
    pub horizontal_resolution: u32,
    pub vertical_resolution: u32,
    pub pixel_format: EfiGraphicsPixelFormat,
    pub pixel_information: EfiPixelBitmask,
    pub pixels_per_scan_line: u32,
}

#[repr(C)]
#[derive(Debug)]
pub enum EfiGraphicsPixelFormat {
    PixelRedGreenBlueReserved8BitPerColor,
    PixelBlueGreenRedReserved8BitPerColor,
    PixelBitMask,
    PixelBltOnly,
    PixelFormatMax,
}

#[repr(C)]
#[derive(Debug)]
pub struct EfiPixelBitmask {
    pub red_mask: u32,
    pub green_mask: u32,
    pub blue_mask: u32,
    pub reserved_mask: u32,
}

#[repr(C)]
pub struct EfiSimpleFileSystemProtocol {
    revision: u64,
    open_volume: extern "efiapi" fn(
        this: &EfiSimpleFileSystemProtocol,
        root: &&EfiFileProtocol,
    ) -> EfiStatus,
}

#[repr(C)]
pub struct EfiFileProtocol {
    revision: u64,
    open: extern "efiapi" fn(
        this: &EfiFileProtocol,
        new_handle: &&EfiFileProtocol,
        file_name: &CHAR16,
        open_mode: u64,
        attributes: u64,
    ) -> EfiStatus,
    close: FnPtr,
    delete: FnPtr,
    read: FnPtr,
    write: FnPtr,
    get_position: FnPtr,
    set_position: FnPtr,
    get_info: extern "efiapi" fn(
        this: &EfiFileProtocol,
        information_type: &EfiGuid,
        buffer_size: &usize,
        buffer: *const c_void,
    ) -> EfiStatus,
    set_info: FnPtr,
    flush: FnPtr,
    open_ex: FnPtr,
    read_ex: FnPtr,
    write_ex: FnPtr,
    flush_ex: FnPtr,
}

#[repr(C)]
pub struct EfiFileInfo {
    size: u64,
    file_size: u64,
    physical_size: u64,
    create_time: EfiTime,
    last_access_time: EfiTime,
    modification_time: EfiTime,
    attribute: u64,
    filename: CHAR16,
}
