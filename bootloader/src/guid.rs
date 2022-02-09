pub const EFI_GRAPHICS_OUTPUT_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x9042a9de, 0x23dc, 0x4a38, 0x96, 0xfb, 0x7a, 0xde, 0xd0, 0x80, 0x51, 0x6a,
);

#[repr(C)]
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
}
