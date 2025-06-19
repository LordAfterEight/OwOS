extern crate alloc;

use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use crate::println;
use crate::gdt;
use crate::memory;
use crate::print;
use crate::serial_println;
//use crate::format;
use spin;
use core::iter::iter;
use alloc::boxed::Box;
use pic8259::ChainedPics;
use lazy_static::lazy_static;
use x86_64::structures::idt::PageFaultErrorCode;
use crate::halt_loop;
use crate::InputBuffer;

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

pub static PICS: spin::Mutex<ChainedPics> =
    spin::Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
    Timer = PIC_1_OFFSET,
    Keyboard,
}

impl InterruptIndex {
    fn as_u8(self) -> u8 {
        self as u8
    }

    fn as_usize(self) -> usize {
        usize::from(self.as_u8())
    }
}

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        unsafe {
            idt.double_fault.set_handler_fn(double_fault_handler)
                .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX); // new
        }

        idt[InterruptIndex::Timer.as_u8()]
            .set_handler_fn(timer_interrupt_handler);

        idt[InterruptIndex::Keyboard.as_u8()]
            .set_handler_fn(keyboard_interrupt_handler);

        idt.page_fault.set_handler_fn(page_fault_handler);

        idt
    };
}

pub fn init_idt() {
    IDT.load();
}


extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn double_fault_handler( stack_frame: InterruptStackFrame, _error_code: u64) -> ! {
    panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: InterruptStackFrame) {
    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Timer.as_u8());
    }
}

extern "x86-interrupt" fn page_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: PageFaultErrorCode,
) {
    use x86_64::registers::control::Cr2;

    println!(" [X] OwOS => EXCEPTION: PAGE FAULT");
    println!("  -> OwOS => Accessed Address: {:?}", Cr2::read());
    println!("  -> OwOS => Error Code: {:?}", error_code);
    println!(" {:#?}\n ", stack_frame);
}

extern "x86-interrupt" fn keyboard_interrupt_handler(
    _stack_frame: InterruptStackFrame)
{
    use pc_keyboard::{layouts, HandleControl, Keyboard, ScancodeSet1};
    use spin::Mutex;
    use x86_64::instructions::port::Port;

    lazy_static! {
        static ref KEYBOARD: Mutex<Keyboard<layouts::De105Key, ScancodeSet1>> =
            Mutex::new(Keyboard::new(ScancodeSet1::new(),
                layouts::De105Key, HandleControl::Ignore)
            );
    }

    let mut keyboard = KEYBOARD.lock();
    let mut port = Port::new(0x60);
    let scancode: u8 = unsafe { port.read() };
    if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
        if let Some(key) = keyboard.process_keyevent(key_event) {
            let mut in_buffer = crate::memory::InputBuffer {
                content: [' ';17],
                index: 0
            };
            unsafe {
                handle_keyboard_input(key, &raw mut input_buffer);
            }
        }
    }

    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Keyboard.as_u8());
    }
}

static mut input_buffer: memory::InputBuffer = memory::InputBuffer {
    content: [' ';17],
    index: 0
};

fn handle_keyboard_input(key: pc_keyboard::DecodedKey, buffer: *mut crate::memory::InputBuffer) {
    match key {
        pc_keyboard::DecodedKey::Unicode(mut character) => match character {
            '\n' => unsafe {
                unsafe {
                    println!("\n [i] OwOS:InputBuffer => Building &str...");
                    let mut buf = [0u8;17];
                    let x: &str = chars_to_str(&(*buffer).content, &mut buf);
                    for i in 0..(*buffer).content.len() {
                        print!("\n [i] OwOS:LineBuffer => {}:{}",
                            (*buffer).content[i],
                            i
                        );
                    }
                    println!("\n [i] OwOS:LineBuffer => {}", x);
                    match x {
                        "help"  => {
                            print!("{}\n", character);
                            print!("{}{}{}{}",
                                "Commands:\n",
                                "  h : Show this help message\n",
                                "  q : Enter halt loop\n",
                                "More commands will be supported soon! :3\n"
                            )
                        },
                        "quit" => {
                            print!("^System stopped :3");
                            serial_println!("Received system stop command");
                            halt_loop();
                        }
                        _ => {
                            print!("{}\n [!] OwOS => Invalid input: {}\n", character, character);
                        }
                    }
                    for i in 0..(*buffer).content.len() {
                        (*buffer).content[i] = ' ';
                    }
                }
                (*buffer).index = 0;
                print!("\n OwOS <= # ");
            }
            '^' => print!("^ OwOS <= # "),
            '`' => print!("`"),
            _ => unsafe {
                if character == ' ' {
                    character = '_';
                }
                print!("{}", character);
                (*buffer).insert(character);
            }
        },
        pc_keyboard::DecodedKey::RawKey(key) => {},
    }
}

use core::str;
use crate::interrupts::alloc::string::ToString;

/*fn build_str_from_chars<'a>(input: &[char; 17], output: &'a mut [u8]) -> &'a str {
    let mut len = 0;
    for (i, c) in input.iter().enumerate() {
        if let Some(byte) = (*c).to_string().as_bytes().first() {
            output[i] = *byte;
            len += 1;
        }
    }
    unsafe { core::str::from_utf8_unchecked(&output[..len]) }
}*/

fn build_str_from_chars(input: &[char; 17]) -> &'static str {
    let mut len = 0;
    unsafe {
        for (i, c) in input.iter().enumerate() {
            if let Some(byte) = (*c).to_string().as_bytes().first() {
                crate::memory::INPUT_BUFFER[i] = *byte;
                len += 1;
            }
        }
        core::str::from_utf8_unchecked(&crate::memory::INPUT_BUFFER[..len])
    }
}

fn chars_to_str<'a>(chars: &[char], output: &'a mut [u8]) -> &'a str {
    let mut pos = 0;

    for &ch in chars {
        let mut buf = [0u8; 4];
        let encoded = ch.encode_utf8(&mut buf);
        let bytes = encoded.as_bytes();
        let len = bytes.len();

        output[pos..pos + len].copy_from_slice(bytes);
        pos += len;
    }

    // SAFETY: We only wrote valid UTF-8
    unsafe { core::str::from_utf8_unchecked(&output[..pos]) }
}
