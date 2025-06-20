#![no_std]
#![cfg_attr(test, no_main)]
#![allow(internal_features)]
#![feature(abi_x86_interrupt)]
#![feature(custom_test_frameworks)]
#![feature(fmt_internals)]
#![feature(allocator_api)]
#![feature(const_eval_select)]
#![feature(core_intrinsics)]
#![feature(rustc_attrs)]
#![feature(ptr_internals)]
#![feature(trusted_len)]
#![feature(slice_range)]
#![feature(extend_one_unchecked)]
#![feature(extend_one)]
#![feature(deref_pure_trait)]
#![feature(sized_type_properties)]
#![feature(min_specialization)]
#![feature(ptr_alignment_type)]
#![feature(temporary_niche_types)]
#![feature(nonnull_provenance)]
#![feature(alloc_layout_extra)]
#![feature(std_internals)]
#![feature(slice_ptr_get)]
#![feature(iter_macro)]

#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(static_mut_refs)]


pub mod kernel;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn init() {
    kernel::interrupts::init_idt();
    kernel::gdt::init();
    unsafe { kernel::interrupts::PICS.lock().initialize() };
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
