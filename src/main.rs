#![no_std]
#![no_main]


use crate::kernel::kernel::Kernel;
use uefi::prelude::*;
use uefi::helpers;
use core::panic::PanicInfo;

mod kernel;
mod os;


#[entry]
fn efi_main(handler: Handle, mut system_table: SystemTable<Boot>) -> Status {
    helpers::init(&mut system_table).unwrap();


    let mut kernel = Kernel::new(&mut system_table);
    kernel.run();


    #[allow(unreachable_code)]
    return Status::SUCCESS;
}
