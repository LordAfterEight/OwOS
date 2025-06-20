use x86_64::structures::paging::{OffsetPageTable, PageTable};
use x86_64::{
    PhysAddr,
    VirtAddr,
    structures::paging::{PhysFrame, Size4KiB, FrameAllocator}
};
use core::alloc::{Layout, AllocError};
use core::ptr::NonNull;
use crate::kernel::vga_buffer;
use crate::serial_print;
use crate::{println, print};
use crate::kernel::vga_buffer::{COLORS, ColorCode, Color};


pub unsafe fn raw_write(address: u16, value: u8) {
    let ptr = address as *mut u16;
    unsafe { *ptr = value as u16; }
}

pub unsafe fn raw_read(address: u16) -> u8 {
    let ptr = address as *mut u16;
    unsafe {
        let x = *ptr as u8;
        return x
    }
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
        self.index += 1;
        if self.index == 17 {
            self.index = 0;
            print!("\n[!] OwOS => Input buffer overflow!\nCleared buffer and reset index to 0\n\nOwOS <= # ");
            for i in 0..self.content.len() {
                self.content[i] = ' ';
            }
        }
    }
}


pub fn memcheck() {
    print!("\n[i] OwOS:memcheck => Not yet implemented!\n");
}
