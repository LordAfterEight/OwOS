use embedded_graphics::geometry::Point;
use embedded_graphics::mono_font::ascii::*;
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::pixelcolor::Rgb888;
use embedded_graphics::text::Text;
use embedded_graphics::primitives::StyledDrawable;
use embedded_graphics::Drawable;
use embedded_graphics::prelude::*;
use uefi::prelude::*;
use uefi::proto::console::gop::GraphicsOutput;
use uefi_graphics2::UefiDisplay;
use alloc::string::String;

pub struct Display {
    pub cursor: String,
    pub cursor_blink: bool,
    pub cursor_y: i32,
    pub cursor_x: i32,
    pub text_offset: u8,
    pub mode: uefi::proto::console::gop::ModeInfo,
    pub display: UefiDisplay,
    pub colors: crate::os::colorlib::Colors,
    pub resolution: (usize,usize)
}

impl Display {
    pub fn new() -> Self {
        let gop_handle = boot::get_handle_for_protocol::<GraphicsOutput>().unwrap();

        let mut gopr = match boot::open_protocol_exclusive::<GraphicsOutput>(gop_handle) {
            Ok(gopr) => {
                gopr
            },
            Err(error) => {
                panic!("{}",error)
            }
        };

        let mode_info = gopr.current_mode_info();

        Display {
            cursor: String::from("_"),
            cursor_blink: true,
            cursor_y: 23,
            cursor_x: 10,
            text_offset: 16,
            mode: mode_info,
            display: UefiDisplay::new(gopr.frame_buffer(), mode_info).unwrap(),
            colors: crate::os::colorlib::Colors::init(),
            resolution: mode_info.resolution()
        }
    }

    pub fn clear(&mut self, color: Rgb888) {
        self.display.clear(color).unwrap();
        self.display.flush();
    }

    pub fn draw_rect(&mut self, x: i32, y: i32, w: u32, h: u32, color: Rgb888) {
        let style = embedded_graphics::primitives::PrimitiveStyleBuilder::new()
            .fill_color(color)
            .build();
        _ = embedded_graphics::primitives::Rectangle::new(Point::new(x as i32, y as i32),Size::new(w as u32, h as u32))
            .draw_styled(&style, &mut self.display);
        self.display.flush();
    }

    pub fn print_title(&mut self) {
        let title_text = &alloc::format!(
            "Welcome to {} v{}!",
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION"),
        ) as &str;

        let (resx,_resy) = self.mode.resolution();
        let title = Text::new(
            title_text,
            Point { x: ((resx / 2) - (title_text.len()*5) -15) as i32, y: self.cursor_y },
            MonoTextStyle::new(&FONT_10X20, self.colors.fg_header)
        );
        title.draw(&mut self.display).unwrap();
        self.display.flush();
    }

    pub fn print(&mut self, x: &str) {
        let text = Text::new(
            &x as &str,
            Point {x: self.cursor_x, y: self.cursor_y},
            MonoTextStyle::new(&FONT_7X14, self.colors.fg)
        );
        text.draw(&mut self.display).unwrap();
        self.display.flush();
        self.cursor_x += (x.len() as i32 * 7) +1;
    }

    pub fn print_colored(&mut self, x: &str, color: Rgb888) {
        let text = Text::new(
            &x as &str,
            Point {x: self.cursor_x, y: self.cursor_y},
            MonoTextStyle::new(&FONT_7X14, color)
        );
        text.draw(&mut self.display).unwrap();
        self.display.flush();
        self.cursor_x += (x.len() as i32 * 7) +1;
    }

    pub fn print_at_position(&mut self, x: &str, pos_x: i32, pos_y: i32, color: Rgb888) {
        let text = Text::new(
            &x as &str,
            Point {x: pos_x, y: pos_y},
            MonoTextStyle::new(&FONT_7X14, color)
        );
        text.draw(&mut self.display).unwrap();
        self.display.flush();
    }

    pub fn println(&mut self, text: &str) {
        let text = Text::new(
            &text as &str,
            Point {x: 10, y: self.cursor_y},
            MonoTextStyle::new(&FONT_7X14, self.colors.fg)
        );
        text.draw(&mut self.display).unwrap();
        self.display.flush();
        self.new_line();
    }

    pub fn new_line(&mut self) {
        self.cursor_y += 16;
        self.cursor_x = 10;
    }
}
