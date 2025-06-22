use embedded_graphics::geometry::Point;
use embedded_graphics::mono_font::ascii::*;
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
    kernel.pause(10);

    println!("[i] OwOS:display => Initializing GOP handle...");
    kernel.pause(10);


    // Get gop
    let gop_handle = boot::get_handle_for_protocol::<GraphicsOutput>().unwrap();
    println!("[i] OwOS:display => Got GOP handle...");
    kernel.pause(10);

    let mut gopr = match boot::open_protocol_exclusive::<GraphicsOutput>(gop_handle) {
        Ok(gopr) => {
            println!("[i] OwOS:display => Opened GOP protocol...");
            kernel.pause(10);
            gopr
        },
        Err(error) => {
            println!("[X] OwOS => Encountered error while opening GOP protocol!");
            kernel.pause(10000);
            panic!("{}",error)
        }
    };


    println!("[i] OwOS:display => Fetching current mode info...");
    let mode = gopr.current_mode_info();

    println!("[i] OwOS:display => Fetched current mode info...");
    kernel.pause(10);

    let mut display = UefiDisplay::new(gopr.frame_buffer(), mode);

    println!("[i] OwOS:display => Created new UEFI display...");
    kernel.pause(50);

    let style = MonoTextStyle::new(&FONT_10X20, Rgb888::CSS_CYAN);

    println!("[i] OwOS:display => Created new style");
    kernel.pause(25);

    let title_text = &alloc::format!(
        "Welcome to {} v{}!",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
    ) as &str;

    let (resx,resy) = mode.resolution();

    let title = Text::new(
        title_text,
        Point { x: ((resx / 2) - (title_text.len()*5) -10) as i32, y: 20 }, style
    );

    let res_text_text = &alloc::format!("[i] OwOS:display => Current resolution: {}x{}", resx, resy) as &str;
    let mut res_text = Text::new(
        res_text_text,
        Point { x: 10, y: 50},
        MonoTextStyle::new(&FONT_7X14, Rgb888::CSS_WHITE)
    );

    let mut text = Text::new(
        "[i] OwOS:os => You will soon be able to input commands :3",
        Point { x: 10, y: 70},
        MonoTextStyle::new(&FONT_7X14, Rgb888::CSS_WHITE)
    );

    println!("[i] OwOS:display => Created new Text");
    kernel.pause(25);


    title.draw(&mut display).unwrap();
    res_text.draw(&mut display).unwrap();
    text.draw(&mut display).unwrap();

    println!("[i] OwOS:display => Drew text on the UEFI display");
    kernel.pause(25);

    println!("[i] OwOS:display => Flushing everything now...");
    kernel.pause(25);

    display.flush();
    println!("[i] OwOS => Reached end! :3");
}
