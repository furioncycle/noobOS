//This tells the compiler to not link with the 
//standard library
#![no_std]
//This tell the compiler that we have no main 
//function
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(noob::test_runner)]
#![reexport_test_harness_main = "test_main"]


use core::panic::PanicInfo;
use noob::println;

//Ensures the rusts compiler outputs the functions
//name _start
#[no_mangle]
pub extern "C" fn _start() -> ! {
    //This is the entry point, since the
    //linker looks for a function named _start
    //by default
    println!("Hello Again{}","!");
    
    #[cfg(test)]
    test_main();

    loop{}
}


#[test_case]
fn trivial_assertion() {
    assert_eq!(1,1);
}




#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
        println!("{}", info);
        loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
        noob::test_panic_handler(info)
}
