extern crate alloc;

use embedded_graphics::geometry::Point;
use embedded_graphics::mono_font::ascii::FONT_6X10;
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::pixelcolor::Rgb888;
use embedded_graphics::text::Text;
use embedded_graphics::Drawable;
use embedded_graphics::prelude::*;
use uefi::prelude::*;
use uefi::proto::console::gop::GraphicsOutput;
use uefi_graphics2::UefiDisplay;
use uefi::{println,print};
use crate::kernel::kernel;

pub fn draw(kernel: &mut kernel::Kernel) {
    // Disable the watchdog timer
    boot::set_watchdog_timer(0, 0x10000, None).unwrap();
    println!("[i] OwOS:display => Set watchdog timer...");
    kernel.pause(100);

    println!("[i] OwOS:display => Initializing GOP handle...");
    kernel.pause(100);


    // Get gop
    let gop_handle = boot::get_handle_for_protocol::<GraphicsOutput>().unwrap();
    println!("[i] OwOS:display => Got GOP handle...");
    kernel.pause(100);

    let mut gop = match boot::open_protocol_exclusive::<GraphicsOutput>(gop_handle) {
        Ok(gop) => {
            println!("[i] OwOS:display => Opened GOP protocol...");
            kernel.pause(100);
            gop
        },
        Err(error) => {
            println!("[X] OwOS => Encountered error while opening GOP protocol!");
            kernel.pause(10000);
            panic!("{}",error)
        }
    };


    // Create UefiDisplay
    println!("[i] OwOS:display => Fetching current mode info...");
    let mode = gop.current_mode_info();
    println!("[i] OwOS:display => Fetched current mode info...");
    kernel.pause(1000);
    let mut display = UefiDisplay::new(gop.frame_buffer(), mode);
    println!("[i] OwOS:display => Created new UEFI display...");
    kernel.pause(500);

    // Create a new character style
    let style = MonoTextStyle::new(&FONT_6X10, Rgb888::WHITE);

    // Create a new text
    let text = Text::new("Hello World!", Point { x: 30, y: 100 }, style);

    // Draw the text on the display
    //text.draw(&mut display).unwrap();

    // Flush everything
    display.flush();

    // wait 10000000 microseconds (10 seconds)
    println!("[i] OwOS => Reached end! :3");
    kernel.pause(5000);
    print!("Shutting down in ");
    for i in 0..4 {
        print!("{}",i);
        kernel.pause(250);
        for i in 0..3 {
            print!(".");
            kernel.pause(250);
        }
    }
    kernel::shutdown();
}
