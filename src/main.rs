#![no_std]
#![no_main]

extern crate alloc;

use core::panic::PanicInfo;
use bootloader::{BootInfo, entry_point};
mod vga_buffer;
mod serial;

entry_point!(memory_check);

#[unsafe(no_mangle)]
fn kernel_main() -> ! {

    print!("\nOwOS <= # ");

    owos::halt_loop();
}

#[unsafe(no_mangle)]
fn memory_check(boot_info: &'static BootInfo) -> ! {
    use owos::allocator;
    use owos::memory::{self, BootInfoFrameAllocator};
    use owos::vga_buffer::{ColorCode, COLORS, Color};
    use alloc::{boxed::Box, vec, vec::Vec, rc::Rc};

    owos::init();

    unsafe {
        COLORS = ColorCode::new(Color::Green, Color::Black);
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
    println!("\n[X] OwOS:Kernel => {}", info);
    kernel_main()
}
