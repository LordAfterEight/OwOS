#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
mod vga_buffer;


#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("OwOS => Hello World!");

    owos::init();
    x86_64::instructions::interrupts::int3();

    #[cfg(test)]
    test_main();

    loop {}
}


#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop{}
}
