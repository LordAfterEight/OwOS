#![allow(dead_code)]

use core::ptr::NonNull;

use uefi::println;
use uefi_raw::table::system::SystemTable;
use uefi::Handle;


pub struct Kernel {
    system_table: SystemTable,
    handler: Handle
}

impl Kernel {

    /// Creates a new instance of a kernel
    pub fn new() -> Self {
        let ptr = 0xFFFF0000 as *mut _;
        unsafe {
            Kernel {
                system_table: SystemTable::default(),
                handler: Handle::new(NonNull::new(ptr).unwrap())
            }
        }
    }

    /*
    /// Pauses the kernel for a given time in milliseconds
    pub fn pause(&mut self, time: usize) {
        self.system_table.boot_services.stall(time*1000);
    }
    */

    pub fn screen_info(&mut self) {
    }

    /// Clears the screen
    pub fn clear_screen(&mut self) {
    }

    /// Executes the kernel code
    pub fn run(&mut self) -> ! {
        println!("[i] OwOS => Welcome to OwOS v{} :3", env!("CARGO_PKG_VERSION"));
        loop {}
    }
}
