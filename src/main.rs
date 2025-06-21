#![no_std]
#![no_main]


use crate::kernel::kernel::Kernel;
use uefi::prelude::*;

use core::panic;

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
    panic!("{}", info);
}
