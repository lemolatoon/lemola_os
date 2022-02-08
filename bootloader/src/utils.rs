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
