#![allow(dead_code)]

use crate::os;

use uefi::println;
use alloc::format;
use embedded_graphics::pixelcolor::Rgb888;

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
        let version = format!("v{}", env!("CARGO_PKG_VERSION"));

        println!("[i] OwOS:kernel => Booting OwOS v{}...\n", version);

        let mut display = os::display::Display::new();
        let (resx,resy) = display.resolution;

        self.pause(500);

        display.print_at_position(
            "OwOS",
            (resx/2-14) as i32,
            (resy/2) as i32,
            display.colors.fg
        );

        display.draw_rect(100,(resy/2+20) as i32, (resx-200) as u32, 5, display.colors.bg_header);

        for i in 0..(resx-200)/3 {
            display.draw_rect(
                (100+i*3) as i32,
                (resy/2+20) as i32,
                3,
                5,
                Rgb888::new((150+(i/7*2)) as u8,(255-(i/7*2)) as u8,255)
            );
        }

        self.pause(500);
        display.clear(display.colors.bg);
        display.draw_rect(0,0,resx as u32,35,display.colors.bg_header); // Header
        display.draw_rect(0,34,resx as u32,2,Rgb888::new(70,70,85)); // Seperator
        display.draw_rect(0,36,resx as u32,(resy-36) as u32,display.colors.bg); // Background

        display.print_at_position(
            &version,
            (resx-50) as i32,
            (resy-10) as i32,
            display.colors.classic.grey
        );

        for i in 20..255 {
            display.colors.fg_header = Rgb888::new(i/2,i/2,i);
            display.print_title("[i] OwOS:os => Welcome to OwOS! :3");
        }

        display.cursor_y += 32;
        self.os_info(&mut display);
        println!("[i] OwOS:kernel => Booted OwOS v{}\n", version);

        loop {}
    }
}


pub fn shutdown() {
    uefi::runtime::reset(uefi::runtime::ResetType::SHUTDOWN, uefi::Status::SUCCESS, None);
}
