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
        let version = format!("v{}", env!("CARGO_PKG_VERSION"));

        self.pause(500);

        display.print_at_position("OwOS", (resx/2-14) as i32, (resy/2) as i32);
        display.draw_rect(100,(resy/2+20) as i32, (resx-200) as u32, 5, display.colors.bg_header);

        for i in 0..(resx-200)/2 {
            display.draw_rect((100+i*2) as i32,(resy/2+20) as i32, 2, 5, Rgb888::new(255,(255-i/7) as u8,255));
        }

        self.pause(500);
        display.clear(display.colors.bg);
        display.draw_rect(0,0,resx as u32,35,display.colors.bg_header);
        display.draw_rect(0,35,resx as u32,(resy-35) as u32,display.colors.bg);

        display.print_at_position(&version, (resx-50) as i32, (resy-10) as i32);

        for i in 15..255 {
            display.colors.fg_header = Rgb888::new(i/4,i/2,i);
            display.print_title("[i] OwOS:os => Welcome to OwOS! :3");
        }

        display.cursor_y += 30;
        self.os_info(&mut display);

        display.print("    OwOS <= # ");

        loop {}
    }
}


pub fn shutdown() {
    uefi::runtime::reset(uefi::runtime::ResetType::SHUTDOWN, uefi::Status::SUCCESS, None);
}
