#![allow(dead_code)]

use crate::os;

use uefi::println;
use alloc::format;
use uefi::proto::pi::mp;
use uefi::proto::console::text::{Input, Key, ScanCode};
use embedded_graphics::pixelcolor::Rgb888;
use embedded_graphics::pixelcolor::WebColors;


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

    pub fn os_info(&mut self, display: &mut os::display::Display) {
        let (resx,resy) = display.resolution;
        let os_info = format!(
            "[i] OwOS:display => Resolution: {}x{} | {}",
            resx,
            resy,
            format!("{} v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"))
        );
        display.println(&os_info as &str);
    }

    /// Pauses for a given time in milliseconds
    pub fn pause(&mut self, time: usize) {
        uefi::boot::stall(time * 1000);
    }


    /// Executes the kernel code
    pub fn run(&mut self) -> ! {
        uefi::boot::set_watchdog_timer(0, 0x10000, None).unwrap();

        println!("[i] OwOS:kernel => Booting OwOS v{}...\n", env!("CARGO_PKG_VERSION"));

        let mut display = os::display::Display::new();
        let (resx,resy) = display.resolution;
        let end = resx - 100;

        display.clear(Rgb888::CSS_BLACK);

        for i in 0..255 {
            display.color = Rgb888::new(i,i,i);
            display.print_title("[i] OwOS:os => Welcome to OwOS! :3");
        }

        display.cursor_y += 20;
        self.os_info(&mut display);

        display.print("OwOS <= # ");

        loop {}
    }
}


pub fn shutdown() {
    uefi::runtime::reset(uefi::runtime::ResetType::SHUTDOWN, uefi::Status::SUCCESS, None);
}
