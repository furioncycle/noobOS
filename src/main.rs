//This tells the compiler to not link with the 
//standard library
#![no_std]
//This tell the compiler that we have no main 
//function
#![no_main]
mod vga_buffer;

use core::panic::PanicInfo;

static HELLO: &[u8] = b"Hello World!";

//Ensures the rusts compiler outputs the functions
//name _start
#[no_mangle]
pub extern "C" fn _start() -> ! {
    //This is the entry point, since the
    //linker looks for a function named _start
    //by default
    println!("Hello Again{}","!");
    panic!("Panic at some message");
    loop{}
}

//This function is called on panic
#[panic_handler]
fn panic(info: &PanicInfo) -> !{
    println!("{}",info);
    loop{}
}

