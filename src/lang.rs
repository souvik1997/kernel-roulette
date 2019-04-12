use core::panic::PanicInfo;

#[lang = "eh_personality"]
#[no_mangle]
pub extern "C" fn eh_personality() {}

#[allow(dead_code)]
extern "C" {
    fn abort() -> !;
    fn panic_c();
}

#[panic_handler]
#[no_mangle]
pub fn rust_begin_panic(info: &PanicInfo) -> ! {
    // Print the file and line number
    if let Some(location) = info.location() {
        println!("Rust panic @ {}:{}",
            location.file(), location.line());
    }

    // Print the message and a newline
    if let Some(message) = info.message() {
        println!("{}", message);
    }

    unsafe {
        // In a real kernel module, we should use abort() instead of panic()
        abort() // replace with panic_c() if you want
    }
}

use core::alloc::Layout;
#[cfg(not(test))]
#[alloc_error_handler]
pub fn alloc_error(_: Layout) -> ! {
    unsafe { abort() }
}