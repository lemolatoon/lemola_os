type FnPtr = u64;

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
