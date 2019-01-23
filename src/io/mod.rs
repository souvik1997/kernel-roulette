use alloc::*;
use alloc::vec::Vec;
use core::fmt;
use spin::Mutex;

lazy_static! {
    /// Used by the `print!` and `println!` macros.
    pub static ref PRINTK_WRITER: Mutex<KernelDebugWriter> = Mutex::new(KernelDebugWriter::default());
}

#[derive(Default)]
pub struct KernelDebugWriter {
    buffer: Vec<u8>,
}

extern "C" {
    fn puts_c(len: u64, c: *const u8);
}

impl KernelDebugWriter {
    fn flush(&mut self) {
        // Call c_puts with the string length and a pointer to its contents
        unsafe { puts_c(self.buffer.len() as u64, self.buffer.as_ptr() as *const u8) };
        self.buffer.clear();
    }
}

impl fmt::Write for KernelDebugWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        // Add the bytes from `s` to the buffer
        self.buffer.extend(s.bytes());
        Ok(())
    }
}

/// Like the `print!` macro in the standard library, but calls printk
#[allow(unused_macros)]
macro_rules! print {
    ($($arg:tt)*) => ($crate::io::print(format_args!($($arg)*)));
}

/// Like the `print!` macro in the standard library, but calls printk
#[allow(unused_macros)]
macro_rules! println {
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}

pub fn print(args: fmt::Arguments) {
    use core::fmt::Write;
    let mut writer = PRINTK_WRITER.lock();
    writer.write_fmt(args).unwrap();
    writer.flush();
}
