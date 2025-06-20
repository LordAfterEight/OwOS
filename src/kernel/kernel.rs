use uefi::prelude::*;
use core::fmt::Write;


/// Represents the core of the operating system.
pub struct Kernel<'a> {
    system_table: &'a mut SystemTable<Boot>,
}

impl<'a> Kernel<'a> {

    /// Creates a new instance of the Kernel.
    pub fn new(system_table: &'a mut SystemTable<Boot>) -> Self {
        Kernel {
            system_table: system_table,
        }
    }

    /// Starts the kernel execution loop.
    /// It initializes the display and enters an infinite loop.
    pub fn run(&mut self) -> ! {
        self.system_table.stdout().reset(false).unwrap();
        writeln!(self.system_table.stdout(), "[i] OwOS => Welcome to OwOS v1.0.0 :3").unwrap();
        loop {}
    }
}
