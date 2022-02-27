#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(noob::test_runner)]
#![reexport_test_harness_main = "test_main"]

use noob::println;
use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();
    
    loop {}
}

#[test_case]
fn test_println() {
    println!("test_println output");
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    noob::test_panic_handler(info);
}