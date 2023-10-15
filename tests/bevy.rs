// This simulates for bevy intergration

/// A 1-1 copy of Bevy's Color.
#[derive(Debug, PartialEq, Clone, Copy)]
enum Color {
    Rgba {
        red: f32,
        green: f32,
        blue: f32,
        alpha: f32,
    },
    RgbaLinear {
        red: f32,
        green: f32,
        blue: f32,
        alpha: f32,
    },
    Hsla {
        hue: f32,
        saturation: f32,
        lightness: f32,
        alpha: f32,
    },
    Lcha {
        lightness: f32,
        chroma: f32,
        hue: f32,
        alpha: f32,
    },
}

impl Color {
    pub const fn rgba(r: f32, g: f32, b: f32, a: f32) -> Color {
        Color::Rgba { red: r, green: g, blue: b, alpha: a }
    }
}


macro_rules! rgba {
    ($color: tt) => {
        ::colorthis::rgbaf!(Color::Rgba, $color => {red, green, blue, alpha})
    };
    ($($colors: literal),* $(,)?) => {
        ::colorthis::rgbaf!(Color::Rgba, [$($colors),*] => {red, green, blue, alpha})
    };
}

const F1: f32 = 1.0 / 255.0;
const F2: f32 = 2.0 / 255.0;
const F3: f32 = 3.0 / 255.0;
const F4: f32 = 4.0 / 255.0;

#[test]
pub fn test_bevy(){
    assert_eq!(rgba!(1, 2, 3), Color::rgba(F1, F2, F3, 1.0));
    assert_eq!(rgba!(1, 2, 3, 4), Color::rgba(F1, F2, F3, F4));
    assert_eq!(rgba!(0.14, 0.56, 0.66), Color::rgba(0.14, 0.56, 0.66, 1.0));
    assert_eq!(rgba!(0.43, 0.12, 0, 1), Color::rgba(0.43, 0.12, 0.0, 1.0));
    assert_eq!(rgba!(0x010203), Color::rgba(F1, F2, F3, 1.0));
    assert_eq!(rgba!(0x01020304), Color::rgba(F1, F2, F3, F4));
    assert_eq!(rgba!("010203"), Color::rgba(F1, F2, F3, 1.0));
    assert_eq!(rgba!("01020304"), Color::rgba(F1, F2, F3, F4));
    assert_eq!(rgba!("#010203"), Color::rgba(F1, F2, F3, 1.0));
    assert_eq!(rgba!("#01020304"), Color::rgba(F1, F2, F3, F4));
    assert_eq!(rgba!([0.15; 3]), Color::rgba(0.15, 0.15, 0.15, 1.0));
    assert_eq!(rgba!([0.75; 3, 0.13]), Color::rgba(0.75, 0.75, 0.75, 0.13));
    assert_eq!(rgba!([2; 4]), Color::rgba(F2, F2, F2, F2));
    assert_eq!(rgba!(Transparent), Color::rgba(0.0, 0.0, 0.0, 0.0));
    assert_eq!(rgba!(Black), Color::rgba(0.0, 0.0, 0.0, 1.0));
    assert_eq!(rgba!(White), Color::rgba(1.0, 1.0, 1.0, 1.0));
}