//This tells the compiler to not link with the 
//standard library
#![no_std]
//This tell the compiler that we have no main 
//function
#![no_main]

use core::panic::PanicInfo;

//Ensures the rusts compiler outputs the functions
//name _start
#[no_mangle]
pub extern "C" fn _start() -> ! {
    //This is the entry point, since the
    //linker looks for a function named _start
    //by default
    loop{}
}

//This function is called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> !{
    loop{}
}