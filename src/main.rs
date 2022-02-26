//This tells the compiler to not link with the 
//standard library
#![no_std]
//This tell the compiler that we have no main 
//function
#![no_main]

use core::panic::PanicInfo;

static HELLO: &[u8] = b"Hello World!";

//Ensures the rusts compiler outputs the functions
//name _start
#[no_mangle]
pub extern "C" fn _start() -> ! {
    //This is the entry point, since the
    //linker looks for a function named _start
    //by default
    let vga_buffer = 0xb8000 as *mut u8;
    
    for(i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb; //Color byte cyan
        }
    }
    
    loop{}
}

//This function is called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> !{
    loop{}
}