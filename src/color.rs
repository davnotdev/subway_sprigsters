//  Adapted from ROOM: https://github.com/davnotdev/Room
use super::*;

//  These correspond with the sprig palette.
#[repr(u8)]
#[derive(Clone, Copy)]
pub enum Color {
    Gray0 = 0,
    GrayL,
    Gray1,
    Gray2,

    Red3,
    RedC,

    Blue7,
    Blue5,

    Yellow6,
    YellowF,

    Green4,
    GreenD,

    Pink8,
    PurpleH,

    Orange9,
}

impl From<u8> for Color {
    fn from(value: u8) -> Self {
        assert!(value <= Color::Orange9 as u8);
        unsafe { core::mem::transmute(value) }
    }
}

impl From<Color> for Rgb565 {
    fn from(value: Color) -> Self {
        match value {
            Color::Gray0 => Rgb565::from(Rgb888::new(0, 0, 0)),
            Color::GrayL => Rgb565::from(Rgb888::new(73, 80, 87)),
            Color::Gray1 => Rgb565::from(Rgb888::new(145, 151, 156)),
            Color::Gray2 => Rgb565::from(Rgb888::new(248, 249, 250)),
            Color::Red3 => Rgb565::from(Rgb888::new(235, 44, 71)),
            Color::RedC => Rgb565::from(Rgb888::new(139, 65, 46)),
            Color::Blue7 => Rgb565::from(Rgb888::new(25, 177, 248)),
            Color::Blue5 => Rgb565::from(Rgb888::new(19, 21, 224)),
            Color::Yellow6 => Rgb565::from(Rgb888::new(254, 230, 16)),
            Color::YellowF => Rgb565::from(Rgb888::new(149, 140, 50)),
            Color::Green4 => Rgb565::from(Rgb888::new(45, 225, 62)),
            Color::GreenD => Rgb565::from(Rgb888::new(29, 148, 16)),
            Color::Pink8 => Rgb565::from(Rgb888::new(245, 109, 187)),
            Color::PurpleH => Rgb565::from(Rgb888::new(170, 58, 197)),
            Color::Orange9 => Rgb565::from(Rgb888::new(245, 113, 23)),
        }
    }
}
