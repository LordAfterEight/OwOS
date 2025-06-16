#![feature(abi_x86_interrupt)]
#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]

pub mod serial;
pub mod vga_buffer;
pub mod interrupts;
pub mod gdt;
pub mod memory;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn init() {
    interrupts::init_idt();
    gdt::init();
    unsafe { interrupts::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

pub fn halt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}


pub struct InputBuffer {
    pub content: [char;17],
    index: usize
}

impl InputBuffer {
    pub fn init(&self) -> InputBuffer {
        let return_value = InputBuffer {
            content: [' ';17],
            index: 0
        };
        return return_value
    }

    pub fn write_into(&mut self, content: char) {
        self.content[self.index] = content;
        self.increase_index();
    }

    fn increase_index(&mut self) {
        if self.index < 16 {
            self.index += 1;
        } else {
            self.index = 0;
        }
    }
}
