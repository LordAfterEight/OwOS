use x86_64::structures::paging::{OffsetPageTable, PageTable};
use x86_64::{
    PhysAddr,
    VirtAddr,
    structures::paging::{PhysFrame, Size4KiB, FrameAllocator}
};
use core::alloc::Layout;
use bootloader::bootinfo::MemoryMap;
use bootloader::bootinfo::MemoryRegionType;
use crate::serial_println;
use crate::println;


pub unsafe fn translate_addr(addr: VirtAddr, physical_memory_offset: VirtAddr)
    -> Option<PhysAddr>
{
    translate_addr_inner(addr, physical_memory_offset)
}


pub unsafe fn active_level_4_table(physical_memory_offset: VirtAddr)
    -> &'static mut PageTable
{
    use x86_64::registers::control::Cr3;

    let (level_4_table_frame, _) = Cr3::read();

    let phys = level_4_table_frame.start_address();
    let virt = physical_memory_offset + phys.as_u64();
    let page_table_ptr: *mut PageTable = virt.as_mut_ptr();

    unsafe { &mut *page_table_ptr }
}

fn translate_addr_inner(addr: VirtAddr, physical_memory_offset: VirtAddr)
    -> Option<PhysAddr>
{
    use x86_64::structures::paging::page_table::FrameError;
    use x86_64::registers::control::Cr3;

    // read the active level 4 frame from the CR3 register
    let (level_4_table_frame, _) = Cr3::read();

    let table_indexes = [
        addr.p4_index(), addr.p3_index(), addr.p2_index(), addr.p1_index()
    ];
    let mut frame = level_4_table_frame;

    // traverse the multi-level page table
    for &index in &table_indexes {
        // convert the frame into a page table reference
        let virt = physical_memory_offset + frame.start_address().as_u64();
        let table_ptr: *const PageTable = virt.as_ptr();
        let table = unsafe {&*table_ptr};

        // read the page table entry and update `frame`
        let entry = &table[index];
        frame = match entry.frame() {
            Ok(frame) => frame,
            Err(FrameError::FrameNotPresent) => return None,
            Err(FrameError::HugeFrame) => panic!("huge pages not supported"),
        };
    }

    // calculate the physical address by adding the page offset
    Some(frame.start_address() + u64::from(addr.page_offset()))
}

pub unsafe fn init(physical_memory_offset: VirtAddr) -> OffsetPageTable<'static> {
    unsafe {
        let level_4_table = active_level_4_table(physical_memory_offset);
        OffsetPageTable::new(level_4_table, physical_memory_offset)
    }
}


pub struct EmptyFrameAllocator;

unsafe impl FrameAllocator<Size4KiB> for BootInfoFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame> {
        let frame = self.usable_frames().nth(self.next);
        self.next += 1;
        frame
    }
}

pub struct BootInfoFrameAllocator {
    memory_map: &'static MemoryMap,
    next: usize,
}

impl BootInfoFrameAllocator {
    pub unsafe fn init(memory_map: &'static MemoryMap) -> Self {
        BootInfoFrameAllocator {
            memory_map,
            next: 0,
        }
    }

    fn usable_frames(&self) -> impl Iterator<Item = PhysFrame> {
        // get usable regions from memory map
        let regions = self.memory_map.iter();
        let usable_regions = regions
            .filter(|r| r.region_type == MemoryRegionType::Usable);
        // map each region to its address range
        let addr_ranges = usable_regions
            .map(|r| r.range.start_addr()..r.range.end_addr());
        // transform to an iterator of frame start addresses
        let frame_addresses = addr_ranges.flat_map(|r| r.step_by(4096));
        // create `PhysFrame` types from the start addresses
        frame_addresses.map(|addr| PhysFrame::containing_address(PhysAddr::new(addr)))
    }
}

pub unsafe fn raw_write(address: u16, value: u8) {
    let ptr = address as *mut u16;
    unsafe { *ptr = value as u16; }
}

pub unsafe fn raw_read(address: u16) -> u8 {
    let ptr = address as *mut u16;
    let x = *ptr as u8;
    return x
}

pub struct InputBuffer {
    pub content: [char;17],
    pub index: usize
}

impl InputBuffer {
    pub fn insert(&mut self, character: char) {
        self.content[self.index] = character;
        //println!(" [i] InputBuffer@OwOS => Saved value '{}' at index {}", self.content[self.index], self.index);
        if self.index == 16 {
            self.index = 0;
            println!(" [!] OwOS => Input buffer overflow!\n Reset index to 0 and cleared buffer\n ");
        }
        unsafe {
            raw_write(0xF000, character as u8);
            let x = char::from(raw_read(0xF000));
            if x == character {
                for i in 0..self.content.len() {
                    serial_println!("V: {} | IDX: {}", self.content[i], i);
                }
            }
        }
        self.index += 1;
    }
}


pub struct Global;

unsafe extern "Rust" {
    static __rust_no_alloc_shim_is_unstable: u8;

    #[rustc_allocator]
    #[rustc_nounwind]
    fn __rust_alloc(size: usize, align: usize) -> *mut u8;

    #[rustc_deallocator]
    #[rustc_nounwind]
    fn __rust_dealloc(ptr: *mut u8, size: usize, align: usize);

    #[rustc_reallocator]
    #[rustc_nounwind]
    fn __rust_realloc(ptr: *mut u8, old_size: usize, align: usize, new_size: usize) -> *mut u8;

    #[rustc_allocator_zeroed]
    #[rustc_nounwind]
    fn __rust_alloc_zeroed(size: usize, align: usize) -> *mut u8;
}

pub unsafe fn realloc(ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
    unsafe { __rust_realloc(ptr, layout.size(), layout.align(), new_size) }
}

pub unsafe fn dealloc(ptr: *mut u8, layout: Layout) {
    unsafe { __rust_dealloc(ptr, layout.size(), layout.align()) }
}

pub unsafe fn alloc(layout: core::alloc::Layout) -> *mut u8 {
    unsafe {
        // Make sure we don't accidentally allow omitting the allocator shim in
        // stable code until it is actually stabilized.
        core::ptr::read_volatile(&__rust_no_alloc_shim_is_unstable);

        __rust_alloc(layout.size(), layout.align())
    }
}


unsafe extern "Rust" {
    // This is the magic symbol to call the global alloc error handler. rustc generates
    // it to call `__rg_oom` if there is a `#[alloc_error_handler]`, or to call the
    // default implementations below (`__rdl_oom`) otherwise.
    fn __rust_alloc_error_handler(size: usize, align: usize) -> !;
}

pub const fn handle_alloc_error(layout: Layout) -> ! {
    const fn ct_error(_: Layout) -> ! {
        panic!("allocation failed");
    }

    #[inline]
    fn rt_error(layout: Layout) -> ! {
        unsafe {
            __rust_alloc_error_handler(layout.size(), layout.align());
        }
    }

    {
        core::intrinsics::const_eval_select((layout,), ct_error, rt_error)
    }

    ct_error(layout)
}
