use crate::uefi::*;
use core::cell::Cell;
use core::fmt::Error;

pub static mut WRITER: Writer = Writer {
    output_protocol: Cell::new(None),
};

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
        Ok(())
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::uefi_utils::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\r\n"));
    ($($arg:tt)*) => ($crate::print!("{}\r\n", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! mem_desc {
    ($boot_services:expr) => {{
        const SIZE: usize = 4096 * 4;
        use core::mem::MaybeUninit;
        let mut memmap_buf: MaybeUninit<[u8; SIZE]> = MaybeUninit::uninit();
        let mem_desc_array = $boot_services.get_memory_descriptor_array(
            memmap_buf.as_mut_ptr(),
            core::mem::size_of_val(&memmap_buf),
        );
        mem_desc_array
    }};
}

#[macro_export]
macro_rules! mem_map {
    ($boot_services:expr) => {{
        const SIZE: usize = 4096 * 4;
        use core::mem::MaybeUninit;
        let mut memmap_buf: MaybeUninit<[u8; SIZE]> = MaybeUninit::uninit();
        let mut map = MemoryMap::new(&mut memmap_buf, SIZE);
        let status = $boot_services.get_memory_map(SIZE, &mut map).unwrap();
        unwrap_success!(status);
        map
    }};
}

trait BitOr {
    fn bit_or(&mut self, other: u64);
}

// impl BitOr for u64 {
//     fn bit_or(&mut self, other: u64) {
//         self = (*self) | other;
//     }
// }

#[doc(hidden)]
pub fn _print(args: core::fmt::Arguments) {
    use core::fmt::Write;
    unsafe {
        WRITER.write_fmt(args).unwrap();
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct MemoryMap {
    pub memory_map_size: usize,
    pub memory_map: *mut EfiMemoryDescriptor,
    pub map_key: usize,
    pub descriptor_size: usize,
    pub descriptor_version: u32,
}

impl MemoryMap {
    pub fn _new<T>(memmap_buf_ptr: *mut T, size: usize) -> Self {
        // let mut memmap_buf = [0u8; 4096 * 4];
        Self {
            memory_map_size: size,
            memory_map: memmap_buf_ptr as *mut EfiMemoryDescriptor,
            map_key: 0,
            descriptor_size: 0,
            descriptor_version: 0,
        }
    }

    pub fn new<T>(memmap_buf: &mut T, size: usize) -> Self {
        MemoryMap::_new(memmap_buf as *mut T, size)
    }

    pub fn iter(self) -> MemoryDescriptorIterator {
        self.array().iter()
    }

    pub fn array(&self) -> MemoryDescriptorArray {
        MemoryDescriptorArray::new(
            self.memory_map,
            self.descriptor_size,
            self.memory_map_size,
            self.map_key,
        )
    }
}

pub struct MemoryDescriptorArray {
    mem_desc_head: *const EfiMemoryDescriptor,
    mem_desc_size: usize,
    mem_map_size: usize,
    map_key: usize,
}

impl MemoryDescriptorArray {
    pub fn get<'a>(&self, index: usize) -> Option<&'a EfiMemoryDescriptor> {
        if self.mem_map_size <= index * self.mem_desc_size {
            // End of MemoryMap; Out of Index
            return None;
        }
        unsafe {
            self.mem_desc_head
                .cast::<u8>()
                .add(index * self.mem_desc_size)
                .cast::<EfiMemoryDescriptor>()
                .as_ref()
        }
    }

    pub fn new<T>(
        mem_desc_head: *const T,
        mem_desc_size: usize,
        mem_map_size: usize,
        map_key: usize,
    ) -> MemoryDescriptorArray {
        MemoryDescriptorArray {
            mem_desc_head: mem_desc_head.cast::<EfiMemoryDescriptor>(),
            mem_desc_size,
            mem_map_size,
            map_key,
        }
    }

    pub fn map_key(&self) -> usize {
        self.map_key
    }

    pub fn iter(self) -> MemoryDescriptorIterator {
        MemoryDescriptorIterator {
            mem_desc_array: self,
            index: 0,
        }
    }
}

pub struct MemoryDescriptorIterator {
    mem_desc_array: MemoryDescriptorArray,
    index: usize,
}

impl Iterator for MemoryDescriptorIterator {
    type Item = &'static EfiMemoryDescriptor;

    fn next(&mut self) -> Option<Self::Item> {
        self.index += 1;

        self.mem_desc_array.get(self.index + 1)
    }
}
