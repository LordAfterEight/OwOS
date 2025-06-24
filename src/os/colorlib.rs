#![allow(dead_code)]
use embedded_graphics::pixelcolor::{Rgb888, WebColors};

pub struct Colors{
    pub fg: Rgb888,
    pub bg: Rgb888,
    pub fg_header: Rgb888,
    pub bg_header: Rgb888,
    pub classic: ClassicColors,
}

impl Colors {
    pub fn init() -> Self {
        Colors {
            fg: Rgb888::new(255,255,255),
            bg: Rgb888::new(7,7,7),
            fg_header: Rgb888::new(255,0,200),
            bg_header: Rgb888::new(17,17,17),
            classic: ClassicColors::init()
        }
    }
}

pub struct ClassicColors {
    pub magenta: Rgb888,
    pub cyan: Rgb888,
    pub blue: Rgb888,
    pub green: Rgb888,
    pub red: Rgb888,
    pub yellow: Rgb888,
    pub orange: Rgb888,
    pub dark_grey: Rgb888,
}

impl ClassicColors {
    pub fn init() -> Self {
        ClassicColors {
            magenta: Rgb888::CSS_MAGENTA,
            cyan: Rgb888::CSS_CYAN,
            blue: Rgb888::CSS_BLUE,
            green: Rgb888 :: CSS_GREEN,
            red: Rgb888::CSS_RED,
            yellow: Rgb888::CSS_YELLOW,
            orange: Rgb888::CSS_ORANGE,
            dark_grey: Rgb888::new(60,60,60),
        }
    }
}
