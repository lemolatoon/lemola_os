use core::arch::asm;

pub fn loop_with_hlt() -> ! {
    loop {
        unsafe {
            asm! {"hlt"};
        }
    }
}

#[macro_export]
macro_rules! dbg {
    ($val:expr $(,)?) => {
        match $val {
            tmp => {
                $crate::println!("{} = {:?}", core::stringify!($val), &tmp);
                tmp
            }
        }
    };
}

#[macro_export]
macro_rules! dyn_utf16 {
    ($e:expr) => {
        use heapless::Vec;
        let size = $e.len();
        if size < 16 {
            use heapless::consts::U16;
            &($e.encode_utf16().collect::<Vec<u16, U16>>())[..]
        } else if size < 512 {
            use heapless::consts::U512;
            &($e.encode_utf16().collect::<Vec<u16, U512>>())[..]
        } else if size < 2048 {
            use heapless::consts::U2048;
            &($e.encode_utf16().collect::<Vec<u16, U2048>>())[..]
        } else if size < 8192 {
            use heapless::consts::U8192;
            &($e.encode_utf16().collect::<Vec<u16, U8192>>())[..]
        } else {
            use heapless::consts::U131072;
            &($e.encode_utf16().collect::<Vec<u16, U131072>>())[..]
        }
    };
}

#[macro_export]
macro_rules! dyn_utf16_ptr {
    ($e:expr) => {
        {
            use heapless::Vec;
            let size = $e.len();
            let ptr = if size < 16 {
                use heapless::consts::U16;
                ($e.encode_utf16().collect::<Vec<u16, U16>>()).as_ptr()
            } else if size < 512 {
                use heapless::consts::U512;
                ($e.encode_utf16().collect::<Vec<u16, U512>>()).as_ptr()
            } else if size < 2048 {
                use heapless::consts::U2048;
                ($e.encode_utf16().collect::<Vec<u16, U2048>>()).as_ptr()
            } else if size < 8192 {
                use heapless::consts::U8192;
                ($e.encode_utf16().collect::<Vec<u16, U8192>>()).as_ptr()
            } else {
                use heapless::consts::U131072;
                ($e.encode_utf16().collect::<Vec<u16, U131072>>()).as_ptr()
            };
            ptr
        }
    };
}
