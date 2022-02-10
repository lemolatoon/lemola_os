pub const EFI_GRAPHICS_OUTPUT_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x9042a9de, 0x23dc, 0x4a38, 0x96, 0xfb, 0x7a, 0xde, 0xd0, 0x80, 0x51, 0x6a,
);

pub const EFI_SIMPLE_FILE_SYSTEM_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x964e5b22, 0x6459, 0x11d2, 0x8e, 0x39, 0x00, 0xa0, 0xc9, 0x69, 0x72, 0x3b,
);

#[repr(C)]
#[derive(Debug)]
pub struct EfiGuid {
    a: u32,
    b: u16,
    c: u16,
    d: u8,
    e: u8,
    f: u8,
    g: u8,
    h: u8,
    i: u8,
    j: u8,
    k: u8,
}

#[macro_export]
macro_rules! impl_guid {
    ($t:ty, $e:expr) => {
        impl HasGuid for $t {
            fn get_guid() -> &'static EfiGuid {
                &$e
            }
        }
    } 
}

pub trait HasGuid {
    fn get_guid() -> &'static EfiGuid;
}

impl EfiGuid {

    const fn new(
        a: u32,
        b: u16,
        c: u16,
        d: u8,
        e: u8,
        f: u8,
        g: u8,
        h: u8,
        i: u8,
        j: u8,
        k: u8,
    ) -> Self {
        Self {
            a,
            b,
            c,
            d,
            e,
            f,
            g,
            h,
            i,
            j,
            k,
        }
    }

    pub fn null() -> Self {
        Self::new(0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
    }
}
