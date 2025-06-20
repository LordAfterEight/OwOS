#![no_std]
#![no_main]
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
#![cfg_attr(test, no_main)]
#![allow(internal_features)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(static_mut_refs)]

extern crate alloc;

pub mod kernel;

use core::panic::PanicInfo;
use bootloader::{BootInfo, entry_point};

entry_point!(memory_check);

#[unsafe(no_mangle)]
fn kernel_main() -> ! {

    print!("\nOwOS <= # ");
    loop {}
}

#[unsafe(no_mangle)]
fn memory_check(boot_info: &'static BootInfo) -> ! {
    use kernel::allocator;
    use kernel::memory::{self, BootInfoFrameAllocator};
    use kernel::vga_buffer::{ColorCode, COLORS, Color};
    use alloc::{boxed::Box, vec, vec::Vec, rc::Rc};

    init();

    unsafe {
        COLORS = ColorCode::new(Color::LightBlue, Color::Black);
        println!("^ [i] OwOS => Welcome to OwOS v{} :3\n ", env!("CARGO_PKG_VERSION"));
    }
    unsafe {COLORS = ColorCode::new(Color::White, Color::Black);}
    serial_println!("Booted kernel");

    /*
    let phys_mem_offset = x86_64::VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_map)
    };

    allocator::init_heap(&mut mapper, &mut frame_allocator)
        .expect("[X] OwOS:Allocator => Heap initialization failed");


    let heap_value = Box::new(41);
    println!("[i] OwOS:Allocator => Heap_value at {:p}", heap_value);

    // create a dynamically sized vector
    let mut vec = Vec::new();
    for i in 0..500 {
        vec.push(i);
    }
    println!("[i] OwOS:Allocator => vec at {:p}", vec.as_slice());

    // create a reference counted vector -> will be freed when count reaches 0
    let reference_counted = Rc::new(vec![1, 2, 3]);
    let cloned_reference = reference_counted.clone();
    println!("[i] OwOS:Allocator => current reference count is {}", Rc::strong_count(&cloned_reference));
    core::mem::drop(reference_counted);
    println!("[i] OwOS:Allocator => reference count is {} now", Rc::strong_count(&cloned_reference));
    */

    kernel_main();
}


#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("\n[X] OwOS:kernel => {}", info);
    println!("[i] OwOS:kernel => Please restart the computer");
    halt_loop()
}

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
