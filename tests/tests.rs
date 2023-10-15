
use colorthis::*;

#[derive(Debug, PartialEq, Eq)]
pub struct Color3 {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color3 {
    pub fn new(r: u8, g: u8, b: u8) -> Self{
        Color3 {
            r, g, b
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self{
        Color {
            r, g, b, a
        }
    }
}


#[derive(Debug, PartialEq)]
pub struct Color3F {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Color3F {
    pub fn new(r: f32, g: f32, b: f32) -> Self{
        Color3F {
            r, g, b
        }
    }
}


#[derive(Debug, PartialEq)]
pub struct ColorF {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl ColorF {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self{
        ColorF {
            r, g, b, a
        }
    }
}
#[test]
pub fn test_int3(){
    assert_eq!(rgb!(Color3::new, [1, 2, 3]), Color3::new(1, 2, 3));
    assert_eq!(rgb!(Color3::new, (2+3, 4*5, 6u8.pow(2))), Color3::new(5, 20, 36));
    assert_eq!(rgb!(Color3::new, [0.0, 1.0, 1.0]), Color3::new(0, 255, 255));
    assert_eq!(rgb!(Color3::new, 0x010203), Color3::new(1, 2, 3));
    assert_eq!(rgb!(Color3::new, "010203"), Color3::new(1, 2, 3));
    assert_eq!(rgb!(Color3::new, "#010203"), Color3::new(1, 2, 3));
    assert_eq!(rgb!(Color3::new, [75; 3]), Color3::new(75, 75, 75));
    assert_eq!(rgb!(Color3::new, Transparent), Color3::new(0, 0, 0));
    assert_eq!(rgb!(Color3::new, Black), Color3::new(0, 0, 0));
    assert_eq!(rgb!(Color3::new, White), Color3::new(255, 255, 255));
    assert_eq!(rgb!(Color3::new, Red), Color3::new(255, 0, 0));
    assert_eq!(rgb!(Color3::new, Green), Color3::new(0, 128, 0));
    assert_eq!(rgb!(Color3::new, Cyan), Color3::new(0, 255, 255));
    assert_eq!(rgb!(Color3::new, Gray700), Color3::new(55, 65, 81));
    assert_eq!(rgb!(Color3::new, Indigo50), Color3::new(238, 242, 255));
    assert_eq!(rgb!(Color3::new, Fuchsia300), Color3::new(240, 171, 252));
    assert_eq!(rgb!(Color3::new, Lime950), Color3::new(26, 46, 5));
    assert_eq!(rgb!(Color3::new, Amber500), Color3::new(245, 158, 11));
}


#[test]
pub fn test_int(){
    assert_eq!(rgba!(Color::new, [1, 2, 3]), Color::new(1, 2, 3, 255));
    assert_eq!(rgba!(Color::new, [1, 2, 3, 4]), Color::new(1, 2, 3, 4));
    assert_eq!(rgba!(Color::new, (2+3, 4*5, 6u8.pow(2))), Color::new(5, 20, 36, 255));
    assert_eq!(rgba!(Color::new, (2+3, 4*5, 6u8.pow(2), u8::MIN)), Color::new(5, 20, 36, 0));
    assert_eq!(rgba!(Color::new, [0.0, 1.0, 1.0]), Color::new(0, 255, 255, 255));
    assert_eq!(rgba!(Color::new, [0.0, 1.0, 1.0, 0.0]), Color::new(0, 255, 255, 0));
    assert_eq!(rgba!(Color::new, 0x010203), Color::new(1, 2, 3, 255));
    assert_eq!(rgba!(Color::new, 0x01020304), Color::new(1, 2, 3, 4));
    assert_eq!(rgba!(Color::new, "010203"), Color::new(1, 2, 3, 255));
    assert_eq!(rgba!(Color::new, "01020304"), Color::new(1, 2, 3, 4));
    assert_eq!(rgba!(Color::new, "#010203"), Color::new(1, 2, 3, 255));
    assert_eq!(rgba!(Color::new, "#01020304"), Color::new(1, 2, 3, 4));
    assert_eq!(rgba!(Color::new, [4; 3]), Color::new(4, 4, 4, 255));
    assert_eq!(rgba!(Color::new, [4; 3, 12]), Color::new(4, 4, 4, 12));
    assert_eq!(rgba!(Color::new, [4; 4]), Color::new(4, 4, 4, 4));
    assert_eq!(rgba!(Color::new, Transparent), Color::new(0, 0, 0, 0));
    assert_eq!(rgba!(Color::new, Black), Color::new(0, 0, 0, 255));
    assert_eq!(rgba!(Color::new, White), Color::new(255, 255, 255, 255));
    assert_eq!(rgba!(Color::new, Red), Color::new(255, 0, 0, 255));
    assert_eq!(rgba!(Color::new, Green), Color::new(0, 128, 0, 255));
    assert_eq!(rgba!(Color::new, Cyan), Color::new(0, 255, 255, 255));
    assert_eq!(rgba!(Color::new, Gray700), Color::new(55, 65, 81, 255));
    assert_eq!(rgba!(Color::new, Indigo50), Color::new(238, 242, 255, 255));
    assert_eq!(rgba!(Color::new, Fuchsia300), Color::new(240, 171, 252, 255));
    assert_eq!(rgba!(Color::new, Lime950), Color::new(26, 46, 5, 255));
    assert_eq!(rgba!(Color::new, Amber500), Color::new(245, 158, 11, 255));
}

const F1: f32 = 1.0 / 255.0;
const F2: f32 = 2.0 / 255.0;
const F3: f32 = 3.0 / 255.0;
const F4: f32 = 4.0 / 255.0;


#[test]
pub fn test_float3(){
    assert_eq!(rgbf!(Color3F::new, [1, 2, 3]), Color3F::new(F1, F2, F3));
    assert_eq!(rgbf!(Color3F::new, [0.14, 0.56, 0.66]), Color3F::new(0.14, 0.56, 0.66));
    assert_eq!(rgbf!(Color3F::new, (0.0f32.sin(), 0.0f32.cos(), 0.0f32.signum())), Color3F::new(0.0, 1.0, 1.0));
    assert_eq!(rgbf!(Color3F::new, 0x010203), Color3F::new(F1, F2, F3));
    assert_eq!(rgbf!(Color3F::new, "010203"), Color3F::new(F1, F2, F3));
    assert_eq!(rgbf!(Color3F::new, "#010203"), Color3F::new(F1, F2, F3));
    assert_eq!(rgbf!(Color3F::new, [0.15; 3]), Color3F::new(0.15, 0.15, 0.15));
    assert_eq!(rgbf!(Color3F::new, Transparent), Color3F::new(0.0, 0.0, 0.0));
    assert_eq!(rgbf!(Color3F::new, Black), Color3F::new(0.0, 0.0, 0.0));
    assert_eq!(rgbf!(Color3F::new, White), Color3F::new(1.0, 1.0, 1.0));
}


#[test]
pub fn test_float(){
    assert_eq!(rgbaf!(ColorF::new, [1, 2, 3]), ColorF::new(F1, F2, F3, 1.0));
    assert_eq!(rgbaf!(ColorF::new, [1, 2, 3, 4]), ColorF::new(F1, F2, F3, F4));
    assert_eq!(rgbaf!(ColorF::new, (0.5f32.powi(2), 2.0 / 4.0, 0.4)), ColorF::new(0.25, 0.5, 0.4, 1.0));
    assert_eq!(rgbaf!(ColorF::new, (0.5f32.powi(2), 2.0 / 4.0, 0.4, 0.1)), ColorF::new(0.25, 0.5, 0.4, 0.1));
    assert_eq!(rgbaf!(ColorF::new, [0.14, 0.56, 0.66]), ColorF::new(0.14, 0.56, 0.66, 1.0));
    assert_eq!(rgbaf!(ColorF::new, [0.43, 0.12, 0, 1]), ColorF::new(0.43, 0.12, 0.0, 1.0));
    assert_eq!(rgbaf!(ColorF::new, 0x010203), ColorF::new(F1, F2, F3, 1.0));
    assert_eq!(rgbaf!(ColorF::new, 0x01020304), ColorF::new(F1, F2, F3, F4));
    assert_eq!(rgbaf!(ColorF::new, "010203"), ColorF::new(F1, F2, F3, 1.0));
    assert_eq!(rgbaf!(ColorF::new, "01020304"), ColorF::new(F1, F2, F3, F4));
    assert_eq!(rgbaf!(ColorF::new, "#010203"), ColorF::new(F1, F2, F3, 1.0));
    assert_eq!(rgbaf!(ColorF::new, "#01020304"), ColorF::new(F1, F2, F3, F4));
    assert_eq!(rgbaf!(ColorF::new, [0.15; 3]), ColorF::new(0.15, 0.15, 0.15, 1.0));
    assert_eq!(rgbaf!(ColorF::new, [0.75; 3, 0.13]), ColorF::new(0.75, 0.75, 0.75, 0.13));
    assert_eq!(rgbaf!(ColorF::new, [2; 4]), ColorF::new(F2, F2, F2, F2));
    assert_eq!(rgbaf!(ColorF::new, Transparent), ColorF::new(0.0, 0.0, 0.0, 0.0));
    assert_eq!(rgbaf!(ColorF::new, Black), ColorF::new(0.0, 0.0, 0.0, 1.0));
    assert_eq!(rgbaf!(ColorF::new, White), ColorF::new(1.0, 1.0, 1.0, 1.0));
}

#[test]
#[cfg(feature="clamp")]
pub fn test_clamp(){
    assert_eq!(rgba!(Color::new, [1000, -1000, 500, 244]), Color::new(255, 0, 255, 244));
    assert_eq!(rgbaf!(ColorF::new, [-1, 2.0, 3.14, 0.5]), ColorF::new(0.0, 1.0, 1.0, 0.5));
}


#[test]
#[cfg(feature="compact")]
pub fn test_compact(){
    assert_eq!(rgba!(Color::new, 0x123), Color::new(0x11, 0x22, 0x33, 255));
    assert_eq!(rgba!(Color::new, 0x1234), Color::new(0x11, 0x22, 0x33, 0x44));
    assert_eq!(rgba!(Color::new, "123"), Color::new(0x11, 0x22, 0x33, 255));
    assert_eq!(rgba!(Color::new, "1234"), Color::new(0x11, 0x22, 0x33, 0x44));
    assert_eq!(rgba!(Color::new, "#123"), Color::new(0x11, 0x22, 0x33, 255));
    assert_eq!(rgba!(Color::new, "#1234"), Color::new(0x11, 0x22, 0x33, 0x44));
}