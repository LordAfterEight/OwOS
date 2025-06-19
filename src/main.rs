#![no_std]
#![no_main]

extern crate alloc;

use core::panic::PanicInfo;
use bootloader::{BootInfo, entry_point};
use alloc::boxed::Box;
mod vga_buffer;
mod serial;

entry_point!(memory_check);

#[unsafe(no_mangle)]
fn kernel_main() -> ! {

    print!("\n OwOS <= # ");

    owos::halt_loop();
}

#[unsafe(no_mangle)]
fn memory_check(boot_info: &'static BootInfo) -> ! {
    use owos::allocator;
    use owos::memory::{self, BootInfoFrameAllocator};
    use owos::vga_buffer::{ColorCode, COLORS, Color};

    owos::init();

    unsafe {
        COLORS = ColorCode::new(Color::Green, Color::Black);
        println!("^ [i] OwOS => Welcome to OwOS v{} :3\n ", env!("CARGO_PKG_VERSION"));
    }
    unsafe {COLORS = ColorCode::new(Color::White, Color::Black);}
    serial_println!("Booted kernel");

    let phys_mem_offset = x86_64::VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_map)
    };

    allocator::init_heap(&mut mapper, &mut frame_allocator)
        .expect("heap initialization failed");


    let x = Box::new(41);

    kernel_main();
}


#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("\n [X] OwOS:Kernel => {}", info);
    kernel_main()
}
