use x86_64::structures::paging::{OffsetPageTable, PageTable};
use x86_64::{
    PhysAddr,
    VirtAddr,
    structures::paging::{PhysFrame, Size4KiB, FrameAllocator}
};
use core::alloc::{Layout, AllocError};
use core::ptr::NonNull;
use bootloader::bootinfo::MemoryMap;
use bootloader::bootinfo::MemoryRegionType;
use crate::serial_println;
use crate::{println, print};


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

pub static mut INPUT_BUFFER: [u8;17] = [0u8;17];

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


pub fn memcheck() {
    print!("\n [i] OwOS:memcheck => Not yet implemented!");
}
