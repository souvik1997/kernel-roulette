# `kernel-roulette`: play Russian Roulette with the Linux kernel

`kernel-roulette` is a simple Linux kernel module written in Rust and C that implements a character device driver for a virtual device. 
When this virtual device is read, there is a chance that the system will crash with a kernel panic <sup>*</sup>.

More importantly, it demonstrates how to build a kernel module with Rust. The rust code uses `#![no_std]` to disable the standard library, but we can still use the following:
- the `core` crate
- the `alloc` crate, which includes data structures such as `Vec` that use dynamic memory allocation.
- macros like `println!()` to print to the `dmesg` buffer.


<sup>*</sup>: It actually uses the `BUG()` macro to show a stack trace in `dmesg`, which doesn't crash the system but leaves it in a somewhat inconsistent state.
If you _really_ want it to panic replace `abort()` with `panic_c()` in `src/lang.rs:31`.

## Dependencies
- Linux kernel headers and `build-essential` (`gcc`, `make`, etc.)
- Nightly Rust (install from https://rustup.rs)
- Xargo and `rust-src`
  
  Download the Rust library source with `rustup component add rust-src`
  
  Install `xargo` with `cargo install xargo`
  
 - `sudo` access
 
## Building
 - Run `make` from the project directory
 - Run `sudo insmod build/roulette.ko`
 - Run `dmesg | tail -n 10` and verify that the kernel module was loaded. You should see something like:
 ```
[   35.735871] IPv6: ADDRCONF(NETDEV_UP): enp4s0: link is not ready
[   39.123353] r8169 0000:04:00.0 enp4s0: link up
[   39.123364] IPv6: ADDRCONF(NETDEV_CHANGE): enp4s0: link becomes ready
[  792.965067] roulette: loading out-of-tree module taints kernel.
[  792.965070] roulette: module license 'unspecified' taints kernel.
[  792.965070] Disabling lock debugging due to kernel taint
[  792.965236] roulette: module verification failed: signature and/or required key missing - tainting kernel
[  792.966321] Registered kernel-roulette with major device number 243
[  792.966322] Run /bin/mknod /dev/kernel-roulette c 243 0
[  793.477624] Panic probability: 10/100
```
 - Follow the instructions in `dmesg`: in this case you would run `sudo /bin/mknod /dev/kernel-roulette c 243 0`
 - Run `cat /dev/kernel-roulette`
 - Unload the module with `sudo rmmod build/roulette.ko`
 
 
 ## Acknowledgements
 - [rust.ko](https://github.com/tsgates/rust.ko)
 - [kmod](https://github.com/saschagrunert/kmod)
 - [The Linux Kernel Module Programming Guide](https://www.tldp.org/LDP/lkmpg/2.6/html/index.html)
 - https://elixir.bootlin.com/
 
 ## License
 GPL 3
