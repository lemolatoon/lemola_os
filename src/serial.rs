use core::cell::Cell;

use uart_16550::SerialPort;

pub static SERIAL: Cell<SerialPort> = {
    let mut serial_port = unsafe { SerialPort::new(0x3F8) };
    serial_port.init();
    serial_port
};

#[doc(hidden)]
pub fn _print(args: core::fmt::Arguments) {
    use core::fmt::Write;
    unsafe {
        SERIAL.write_fmt(args).expect("serial port output failed");
    }
}