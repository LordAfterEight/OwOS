#![allow(dead_code)]

use core::fmt::Debug;

use crate::os::{self, input};

use uefi::boot::ScopedProtocol;
use uefi::println;
use alloc::format;
use embedded_graphics::pixelcolor::Rgb888;
use uefi::proto::console::text::{Input, Key, ScanCode};
use uefi::{boot, Char16, Result, ResultExt};

use uefi::{
    Handle as UefiHandle,
    boot::{get_handle_for_protocol, open_protocol_exclusive},
    proto::console::text::Input as InputProtocol,
};

pub struct Kernel {
    stdin: ScopedProtocol<Input>
}

impl Kernel {

    /// Creates a new instance of a kernel
    pub fn new() -> Self {
        Kernel {
            stdin: input::init_input_protocol(),
        }
    }

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
    pub fn run(&mut self) -> uefi::Status {

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
        println!("[i] OwOS:kernel => Booted OwOS {}\n", version);

        let mut cursor = "_";
        display.print("    OwOS:input   <= ");

        let mut input_buffer = alloc::string::String::new();
        let return_key = Char16::try_from('\r').unwrap();

        loop {

            if counter == 250_000 {
                display.print(cursor);
                display.cursor_x -= 8;
            }

            if counter > 500_000 {
                counter = 0;
                display.draw_rect(
                    display.cursor_x as i32,
                    display.cursor_y as i32 - 12,
                    8, 16,
                    display.colors.classic.black
                );
            }

            let input = self.stdin.read_key().expect("Failed to read key");
            if let Some(Key::Printable(key)) = input {
                if key == return_key {
                    display.draw_rect(
                        display.cursor_x as i32,
                        display.cursor_y as i32 - 12,
                        8, 16,
                        display.colors.classic.black
                    );
                    display.new_line();
                    match input_buffer.as_str() {
                        "shutdown" => {
                            display.println("[i] OwOS:kernel => Shutting down...");
                            self.pause(1000);
                            shutdown();
                        },
                        "reboot" => {
                            display.println("[i] OwOS:kernel => Rebooting...");
                            self.pause(1000);
                            reboot();
                        },
                        "clear" => {
                            display.clear(display.colors.classic.black);
                            display.cursor_y = 12;
                            display.cursor_x = 10;

                        },
                        "help" => {
                            display.println_colored(
                                "[i] commands: shutdown, reboot, clear, help, time",
                                display.colors.classic.yellow
                            );
                        },
                        "time" => {
                            let time = uefi::runtime::get_time().expect("Failed to get time");
                            display.println_colored(
                                &format!("[i] OwOS:time    => {}", time),
                                display.colors.classic.cyan
                            );
                        },
                        "" => {
                        },
                        _ => {
                            display.println_colored(&format!("[i] Invalid command: {}", input_buffer), display.colors.classic.orange);
                            input_buffer.clear();
                        }
                    }
                    input_buffer.clear();
                    display.print("    OwOS:input   <= ");
                } else {
                    display.draw_rect(
                        display.cursor_x as i32,
                        display.cursor_y as i32 - 12,
                        8, 16,
                        display.colors.classic.black
                    );
                    display.print(&format!("{}", key));
                    input_buffer.push(key.into());
                }
            }

            counter += 1;
        }
        return uefi::Status::SUCCESS;
    }
}


pub fn shutdown() {
    uefi::runtime::reset(uefi::runtime::ResetType::SHUTDOWN, uefi::Status::SUCCESS, None);
}

pub fn reboot() {
    uefi::runtime::reset(uefi::runtime::ResetType::COLD, uefi::Status::SUCCESS, None);
}
