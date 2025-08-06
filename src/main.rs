#![no_std]
#![no_main]
#![feature(ptr_as_uninit)]
#![allow(static_mut_refs)]

extern crate alloc;

use crate::kernel::kernel::Kernel;
use uefi::prelude::*;
use uefi::allocator::Allocator;

mod kernel;
mod os;

#[entry]
fn main() -> Status {
    uefi::helpers::init().unwrap();


    let mut kernel = Kernel::new();

    kernel.run();

    #[allow(unreachable_code)]
    return Status::SUCCESS;
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    uefi::println!("[X] OwOS:kernel => Error occured! {}", info);
    kernel::kernel::Kernel::new().run();
    loop {} // Should never reach here
}


#[global_allocator]
static ALLOCATOR: Allocator = Allocator;
