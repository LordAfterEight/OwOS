use embedded_graphics::pixelcolor::Rgb888;

pub struct Colors{
    pub fg: Rgb888,
    pub bg: Rgb888,
    pub fg_header: Rgb888,
    pub bg_header: Rgb888
}

impl Colors {
    pub fn init() -> Self {
        Colors {
            fg: Rgb888::new(255,255,255),
            bg: Rgb888::new(5,5,5),
            fg_header: Rgb888::new(255,0,200),
            bg_header: Rgb888::new(15,15,15)
        }
    }
}
