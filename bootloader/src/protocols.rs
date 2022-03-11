use crate::dyn_utf16_ptr;
use crate::guid::*;
use crate::println;
use crate::unwrap_success;
use core::ffi::c_void;
use core::mem::MaybeUninit;

use crate::uefi::*;

type FnPtr = u64;
type CHAR16 = u16;

use crate::impl_guid;
impl_guid!(
    EfiGraphicsOutputProtocol<'_>,
    EFI_GRAPHICS_OUTPUT_PROTOCOL_GUID
);
impl_guid!(
    EfiSimpleFileSystemProtocol,
    EFI_SIMPLE_FILE_SYSTEM_PROTOCOL_GUID
);

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
        root: *const &EfiFileProtocol,
    ) -> EfiStatus,
}

impl EfiSimpleFileSystemProtocol {
    pub fn root_dir(&self, root_dir: &MaybeUninit<&EfiFileProtocol>) -> EfiStatusCode {
        let root_dir = root_dir.as_ptr();
        let status = (self.open_volume)(self, root_dir);
        let status = EfiStatusCode::try_from(status).unwrap();
        if !status.is_success() {
            println!("{:?}", status);
            panic!("open_volume failed");
        }
        status
    }
}

#[repr(C)]
pub struct EfiFileProtocol {
    pub revision: u64,
    open: extern "efiapi" fn(
        this: &EfiFileProtocol,
        new_handle: *const &EfiFileProtocol,
        file_name: *const CHAR16,
        open_mode: u64,
        attributes: u64,
    ) -> EfiStatus,
    close: FnPtr,
    delete: FnPtr,
    pub read: extern "efiapi" fn(
        this: &EfiFileProtocol,
        buffer_size: &usize,
        buffer: *const c_void,
    ) -> EfiStatus,
    // TODO: change to c_void
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

impl EfiFileProtocol {
    pub fn open(
        &self,
        new_handle: &MaybeUninit<&EfiFileProtocol>,
        file_name: &str,
        open_mode: OpenMode,
        attribute: FileAttributes,
    ) -> EfiStatusCode {
        let status: EfiStatusCode = (self.open)(
            self,
            new_handle.as_ptr(),
            dyn_utf16_ptr!(file_name),
            open_mode.into(),
            attribute.into(),
        )
        .try_into()
        .unwrap();
        unwrap_success!(status);
        status
    }

    pub fn get_info(&self, buffer_size: &mut usize, buffer: &[MaybeUninit<u8>]) -> EfiStatusCode {
        let status: EfiStatusCode = (self.get_info)(
            self,
            &EFI_FILE_INFO_GUID,
            buffer_size,
            buffer.as_ptr().cast(),
        )
        .try_into()
        .unwrap();
        if !status.is_success() {
            println!("{:?}", status);
            panic!("get_info failed");
        }
        status
    }
}

#[derive(Debug)]
pub enum OpenMode {
    EfiFileModeRead,
    EfiFileModeWrite,
    EfiFiileModeCreate,
}

impl Into<u64> for OpenMode {
    fn into(self) -> u64 {
        match self {
            OpenMode::EfiFileModeRead => 0x0000000000000001,
            OpenMode::EfiFileModeWrite => 0x0000000000000002,
            OpenMode::EfiFiileModeCreate => 0x8000000000000000,
        }
    }
}

#[derive(Debug)]
pub enum FileAttributes {
    EfiFileReadOnly,
    EfiFileHidden,
    EfiFileSystem,
    EfiFileReserved,
    EfiFileDirectory,
    EfiFileArchive,
    EfiFileValidAttr,
}

impl Into<u64> for FileAttributes {
    fn into(self) -> u64 {
        match self {
            FileAttributes::EfiFileReadOnly => 0x0000000000000001,
            FileAttributes::EfiFileHidden => 0x0000000000000001,
            FileAttributes::EfiFileSystem => 0x0000000000000004,
            FileAttributes::EfiFileReserved => 0x0000000000000008,
            FileAttributes::EfiFileDirectory => 0x0000000000000010,
            FileAttributes::EfiFileArchive => 0x0000000000000020,
            FileAttributes::EfiFileValidAttr => 0x0000000000000037,
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct EfiFileInfo {
    pub size: u64,
    pub file_size: u64,
    physical_size: u64,
    create_time: EfiTime,
    last_access_time: EfiTime,
    modification_time: EfiTime,
    attribute: u64,
    pub filename: [CHAR16; 12],
}
