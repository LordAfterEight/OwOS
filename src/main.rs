#![no_std]
#![no_main]

use core::panic::PanicInfo;
use bootloader::{BootInfo, entry_point};
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
    use owos::memory;
    use owos::memory::BootInfoFrameAllocator;
    use owos::memory::active_level_4_table;
    use x86_64::{structures::paging::{Page, Translate}, VirtAddr};

    println!("^ [i] OwOS => Welcome to OwOS v{} :3\n ", env!("CARGO_PKG_VERSION"));
    serial_println!("Booted kernel");

    owos::init();

    let mut frame_allocator = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_map)
    };

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };


    let addresses = [
        // the identity-mapped vga buffer page
        0xb8000,
        // some code page
        0x201008,
        // some stack page
        0x0100_0020_1a10,
        // virtual address mapped to physical address 0
        boot_info.physical_memory_offset,
    ];

    for &address in &addresses {
        let virt = VirtAddr::new(address);
        // new: use the `mapper.translate_addr` method
        let phys = mapper.translate_addr(virt);
        println!(" virt: {:?}\n phys: {:?}\n\n ", virt, phys);
    }

    unsafe {
        println!("\n [i] OwOS => Memory check done and successful :3\n ");
        serial_println!("\nMemory checks successful");
        kernel_main();
    }
}


#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!(" [X] Kernel@OwOS => {:?}\n [i] OwOS => Press Enter", info);
    owos::halt_loop()
}
