use core;

// These functions and traits are used by the compiler, but not
// for a bare-bones hello world. These are normally
// provided by libstd.
#[lang = "eh_personality"]
#[no_mangle]
pub extern "C" fn eh_personality() {}

#[lang = "eh_unwind_resume"]
#[no_mangle]
pub extern "C" fn eh_unwind_resume(_a: *const u8) {}

#[allow(dead_code)]
extern "C" {
    fn abort();
    fn panic_c();
}

#[lang = "panic_fmt"]
#[no_mangle]
pub extern "C" fn rust_begin_panic(msg: core::fmt::Arguments, file: &'static str, line: u32) -> ! {
    use super::io;
    // Print the file and line number
    println!("Rust panic @ {}:{}", file, line);

    // Print the message and a newline
    io::print(msg);
    println!("");

    unsafe {
        // In a real kernel module, we should use abort() instead of panic()
        abort(); // replace with panic_c() if you want
    }
    loop {}
}
