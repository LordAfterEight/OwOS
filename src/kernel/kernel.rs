#![allow(dead_code)]

use crate::os::{self, input};

use uefi::println;
use alloc::format;
use embedded_graphics::pixelcolor::Rgb888;
use uefi::proto::console::text::{Input, Key, ScanCode};
use uefi::{boot, Char16, Result, ResultExt};

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

        let mut counter = 0;

        uefi::boot::set_watchdog_timer(0, 0x10000, None).unwrap();
        let version = format!("v{}", env!("CARGO_PKG_VERSION"));

        println!("[i] OwOS:kernel => Booting OwOS {}...\n", version);

        let mut display = os::display::Display::new();
        let (resx,resy) = display.resolution;

        display.draw_rect(0,34,resx as u32,1,Rgb888::new(70,70,85)); // Seperator

        display.print_at_position(
            &version,
            (resx-50) as i32,
            (resy-10) as i32,
            display.colors.classic.grey
        );

        display.print_title();

        display.cursor_y += 32;
        self.os_info(&mut display);
        display.print("    OwOS:input   <= ");
        println!("[i] OwOS:kernel => Booted OwOS {}\n", version);

        let mut cursor = "_";

        loop {

            if counter == 50000000 {
                display.print(cursor);
                display.cursor_x -= 8;
            }

            if counter > 100000000 {
                counter = 0;
                display.draw_rect(
                    display.cursor_x as i32,
                    display.cursor_y as i32 - 12,
                    8, 16,
                    display.colors.classic.black
                );
            }

            counter += 1;
        }
    }
}


pub fn shutdown() {
    uefi::runtime::reset(uefi::runtime::ResetType::SHUTDOWN, uefi::Status::SUCCESS, None);
}
