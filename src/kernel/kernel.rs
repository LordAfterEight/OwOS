#![allow(dead_code)]

use crate::os;

use uefi::println;


pub struct Kernel {
}

impl Kernel {

    /// Creates a new instance of a kernel
    pub fn new() -> Self {
        Kernel {}
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

    /// Pauses for a given time in milliseconds
    pub fn pause(&mut self, time: usize) {
        uefi::boot::stall(time * 1000);
    }


    /// Executes the kernel code
    pub fn run(&mut self) -> ! {
        println!("[i] OwOS:kernel => Booting OwOS v{}...\n", env!("CARGO_PKG_VERSION"));
        self.pause(3000);
        os::display::draw(self);
        loop {}
    }
}


pub fn shutdown() {
    uefi::runtime::reset(uefi::runtime::ResetType::SHUTDOWN, uefi::Status::SUCCESS, None);
}
