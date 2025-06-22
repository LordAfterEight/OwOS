#![no_std]
#![no_main]

extern crate alloc;

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

#[global_allocator]
static ALLOCATOR: Dummy = Dummy;

use alloc::alloc::{GlobalAlloc, Layout};
use core::ptr::null_mut;

pub struct Dummy;

unsafe impl GlobalAlloc for Dummy {
    unsafe fn alloc(&self, _layout: Layout) -> *mut u8 {
        null_mut()
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        panic!("dealloc should be never called")
    }
}
