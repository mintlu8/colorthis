//! Meta macros that aid macro authors to create colors from a generalized syntax.
//!
//! # General Syntax
//!
//! ```
//! # /*
//! rgb!($path: path, $color_syntax: tt [=> {$($fields: ident)*}])
//! # */
//! ```
//!
//! Where path is a function or a struct constructor
//!
//! If fields are not specified,
//! ```
//! # use colorthis::rgba;
//! # struct Cmyk(u8, u8, u8, u8);
//! rgba!(Cmyk, [123, 155, 224, 155]);
//! ```
//!
//! produces
//!
//! ```
//! # struct Cmyk(u8, u8, u8, u8);
//! Cmyk(123, 155, 224, 155);
//! ```
//!
//! If fields are specified,
//! ```
//! # use colorthis::rgba;
//! # struct Cmyk{c: u8, m: u8, y: u8, k: u8};
//! rgba!(Cmyk, [123, 155, 224, 155] => {c, m, y, k});
//! ```
//!
//! produces
//!
//! ```
//! # struct Cmyk{c: u8, m: u8, y: u8, k: u8};
//! Cmyk {
//!     c: 123,
//!     m: 155,
//!     y: 224,
//!     k: 155,
//! };
//! ```
//!
//! Populating a larger struct is allowed as long as the struct implements Default.
//!
//! ```
//! # use colorthis::rgba;
//! #[derive(Default)]
//! struct PixelData {
//!     r: u8, g: u8, b: u8, a: u8,
//!     z: u8, brightness: u8, magic_number: u8
//! }
//! rgba!(PixelData, [1, 2, 3, 4] => {r, g, b, a, _});
//! ```
//!
//! produces
//!
//! ```
//! # #[derive(Default)] struct PixelData { r: u8, g: u8, b: u8, a: u8, z: u8, brightness: u8, magic_number: u8 }
//! PixelData {
//!     r: 1,
//!     g: 2,
//!     b: 3,
//!     a: 4,
//!     ..Default::default()
//! };
//! ```
//! 
//! # Example usage
//! ```
//! # mod bevy {pub mod prelude {
//! #     pub enum Color { Rgba { red: f32, green: f32, blue: f32, alpha: f32 }, Hsla,  }
//! # }}
//! # use colorthis::rgba;
//! macro_rules! bevy_rgba {
//!     ($color: tt) => {
//!         ::colorthis::rgbaf!(bevy::prelude::Color::Rgba, $color => {red, green, blue, alpha})
//!     };
//!     ($($colors: literal),* $(,)?) => {
//!         ::colorthis::rgbaf!(bevy::prelude::Color::Rgba, [$($colors),*] => {red, green, blue, alpha})
//!     };
//! }
//! bevy_rgba!("00EEFFFF");
//! bevy_rgba!("#AAEE22");
//! bevy_rgba!(0xAABBCC);
//! bevy_rgba!(72, 184, 255, 255);
//! bevy_rgba!(0.23, 0.44, 0.89, 0.50);
//! bevy_rgba!(Red);
//! bevy_rgba!(Violet800);
//! ```
//!
//! # Color Syntax
//!
//! The color is always a TokenTree `tt`.
//!
//! * Bracketed numbers: `[0.3, 0.72, 0.98]`, `[124, 54, 87, 255]`
//! * Parenthesised expressions: `(0.3, 0.72, 0.98)`, `(r, g, b + g, a + 0.5)`
//! * Splat syntax: `[0.3; 3]`, `[0.3; 3, 0.8]`, `[0.7; 4]`
//! * Hex strings: `"AABBCC"`, `"AABBCCFF"`, `"#AABBCC"`, `"#AABBCCFF"`
//! * Hex number literals: `0xAABBCC`, `0xAABBCCFF`
//! * CSS color names: `Red`, `Blue`
//! * TailwindCSS color names: `Red100`, `Sky400`
//!
//! # Details
//!
//! ### Bracketed Numbers
//! If all values are integers,
//! they are considered to be in range `0..=255`.
//!
//! ```
//! # use colorthis::rgba;
//! # #[derive(Debug, PartialEq)] struct Color(u8,u8,u8,u8);
//! assert_eq!(rgba!(Color, [32, 144, 220, 125]), Color(32,144,220,125));
//! ```
//!
//! If any value is a float,
//! values are considered to be in range `0.0..=1.0`.
//!
//! Therefore this fails to compile since numbers like 144 overflows this bounds.
//!
//! ```compile_fail
//! # use colorthis::rgba;
//! rgba!(Color, [0.5, 144, 220, 125]);
//! ```
//!
//! If **all** values are `0`s or `1`s, and the macro invoked
//! is `rgbf!()` or `rgbaf!()` `1`s are treated as `1.0`s.
//!
//! ```
//! # use colorthis::rgbaf;
//! # #[derive(Debug, PartialEq)] struct ColorF(f32,f32,f32,f32);
//! assert_eq!(rgbaf!(ColorF, [255, 255, 255, 255]), ColorF(1.0, 1.0, 1.0, 1.0));
//! assert_eq!(rgbaf!(ColorF, [1.0, 0.5, 1, 0]), ColorF(1.0, 0.5, 1.0, 0.0));
//! assert_eq!(rgbaf!(ColorF, [1, 0, 1, 0]), ColorF(1.0, 0.0, 1.0, 0.0));
//! ```
//!
//! ### Parenthesised Expressions
//! Currently we do not modify the expression or provide type conversion,
//! aside from validating the number of expressions,
//! and providing default alpha value if needed.
//!
//! ### Splat Syntax
//! * `[v; 3]` means `[v, v, v, 255]`
//! * `[v; 3, a]` means `[v, v, v, a]`
//! * `[v; 4]` means `[v, v, v, v]`
//! 
//! ### Color Names
//! We relies on a [crate](https://docs.rs/parse-color/latest/parse_color/) 
//! to parse and generate these data at compile time. No external support required.
//! 
//! # Feature Flags
//!
//! ## `unchecked` and `clamp`
//! By default, we assert integers are in `0..=255`, floats are in `0.0..=1.0`
//!
//! if unchecked is not specified, this fails
//! ```compile_fail
//! # use colorthis::rgba;
//! # fn color(r: u8, g: u8, b: u8, a: u8) {}
//! rgba!(color, [1000, 255, 128, 0]); // 1000 is not in 0..=255
//! # #[cfg(feature="unchecked")] compile_error!();
//! ```
//!
//! With `unchecked`, this *might* compile, assuming color accepts `1000` as an input.
//! ```
//! # /*
//! rgba!(color, [1000, 255, 128, 0]);
//! # */
//! ```
//!
//! With `clamp`, this compiles and clamps `1000` into `255`
//! ```
//! # /*
//! rgba!(color, [1000, 255, 128, 0]); // produces color(255, 255, 128, 0)
//! # */
//! ```
//!
//! ## `compact`
//!
//! Compact allows 3 or 4 letter compact colors to be compiled.
//! ```
//! # /*
//! rgba!(color, 0xABC); // compiles to 0xAABBCC
//! rgba!(color, "1234"); // compiles to "11223344"
//! rgba!(color, "#FFF"); // compiles to "FFFFFF"
//! # */
//! ```
//!
use proc_macro_error::proc_macro_error;
use quote::quote;
mod convert;
use convert::Convert;
mod path;
use path::extract_path;
mod parse;
use parse::{ColorResult, parse_color, extract_idents};


/// Converts color-like tokens into a function call or a struct constructor that receives 3 integers
///
/// Syntax:
/// ```
/// # /*
/// rgb!(path, color_syntax [=> {fields}])
/// # */
/// ```
#[proc_macro]
#[proc_macro_error]
pub fn rgb(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let (path, color, args) = extract_path(tokens.into());
    let mut rgba = parse_color(color);
    if let Some(args) = args {
        rgba = rgba.with_idents(extract_idents(args))
    }
    match rgba {
        ColorResult::Integers(ints) => {
            let items = ints.to_int3();
            quote!(#path (#(#items),*))
        },
        ColorResult::Floats(floats) => {
            let items = floats.to_int3();
            quote!(#path (#(#items),*))
        },
        ColorResult::Exprs(streams) => {
            let streams = streams.into_iter().take(3)
                .map(|x| x.unwrap_or(quote!(255)));
            quote!(#path (#(#streams),*))
        },
        ColorResult::IntFields(ints, idents) => {
            let items = ints.to_int3();
            if idents.len() > 3 {
                let idents = &idents[..3];
                quote!(#path {#(#idents: #items),*, ..Default::default()})
            } else {
                quote!(#path {#(#idents: #items),*})
            }
        },
        ColorResult::FloatFields(floats, idents) => {
            let items = floats.to_int3();
            if idents.len() > 3 {
                let idents = &idents[..3];
                quote!(#path {#(#idents: #items),*, ..Default::default()})
            } else {
                quote!(#path {#(#idents: #items),*})
            }
        },
        ColorResult::ExprFields(streams, idents) => {
            let streams = streams.into_iter().take(3)
                .map(|x| x.unwrap_or(quote!(255)));
            if idents.len() > 3 {
                let idents = &idents[..3];
                quote!(#path {#(#idents: #streams),*, ..Default::default()})
            } else {
                quote!(#path {#(#idents: #streams),*})
            }
        }
    }.into()
}

/// Converts color-like tokens into a function call or a struct constructor that receives 4 integers
///
/// Syntax:
/// ```
/// # /*
/// rgba!(path, color_syntax [=> {fields}])
/// # */
/// ```
#[proc_macro]
#[proc_macro_error]
pub fn rgba(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let (path, color, args) = extract_path(tokens.into());
    let mut rgba = parse_color(color);
    if let Some(args) = args {
        rgba = rgba.with_idents(extract_idents(args))
    }
    match rgba {
        ColorResult::Integers(ints) => {
            let items = ints.to_int4();
            quote!(#path (#(#items),*))
        },
        ColorResult::Floats(floats) => {
            let items = floats.to_int4();
            quote!(#path (#(#items),*))
        },
        ColorResult::Exprs(streams) => {
            let streams = streams.into_iter()
                .map(|x| x.unwrap_or(quote!(255)));
            quote!(#path (#(#streams),*))
        },
        ColorResult::IntFields(ints, idents) => {
            let items = ints.to_int4();
            if idents.len() > 4 {
                let idents = &idents[..4];
                quote!(#path {#(#idents: #items),*, ..Default::default()})
            } else {
                quote!(#path {#(#idents: #items),*})
            }
        },
        ColorResult::FloatFields(floats, idents) =>  {
            let items = floats.to_int4();
            if idents.len() > 4 {
                let idents = &idents[..4];
                quote!(#path {#(#idents: #items),*, ..Default::default()})
            } else {
                quote!(#path {#(#idents: #items),*})
            }
        },
        ColorResult::ExprFields(streams, idents) => {
            let streams = streams.into_iter()
                .map(|x| x.unwrap_or(quote!(255)));
            if idents.len() > 4 {
                let idents = &idents[..4];
                quote!(#path {#(#idents: #streams),*, ..Default::default()})
            } else {
                quote!(#path {#(#idents: #streams),*})
            }
        }
    }.into()
}

/// Converts color-like tokens into a function call or a struct constructor that receives 3 flaoting point numbers
///
/// Syntax:
/// ```
/// # /*
/// rgbf!(path, color_syntax [=> {fields}])
/// # */
/// ```
#[proc_macro]
#[proc_macro_error]
pub fn rgbf(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let (path, color, args) = extract_path(tokens.into());
    let mut rgba = parse_color(color);
    if let Some(args) = args {
        rgba = rgba.with_idents(extract_idents(args))
    }
    match rgba {
        ColorResult::Integers(ints) => {
            let items = ints.to_float3();
            quote!(#path (#(#items),*))
        },
        ColorResult::Floats(floats) => {
            let items = floats.to_float3();
            quote!(#path (#(#items),*))
        },
        ColorResult::Exprs(streams) => {
            let streams = streams.into_iter().take(3)
                .map(|x| x.unwrap_or(quote!(1.0)));            
            quote!(#path (#(#streams),*))
        },
        ColorResult::IntFields(ints, idents) => {
            let items = ints.to_float3();
            if idents.len() > 3 {
                let idents = &idents[..3];
                quote!(#path {#(#idents: #items),*, ..Default::default()})
            } else {
                quote!(#path {#(#idents: #items),*})
            }
        },
        ColorResult::FloatFields(floats, idents) => {
            let items = floats.to_float3();
            if idents.len() > 3 {
                let idents = &idents[..3];
                quote!(#path {#(#idents: #items),*, ..Default::default()})
            } else {
                quote!(#path {#(#idents: #items),*})
            }
        },
        ColorResult::ExprFields(streams, idents) => {
            let streams = streams.into_iter().take(3)
                .map(|x| x.unwrap_or(quote!(1.0)));
            if idents.len() > 3 {
                let idents = &idents[..3];
                quote!(#path {#(#idents: #streams),*, ..Default::default()})
            } else {
                quote!(#path {#(#idents: #streams),*})
            }
        },
    }.into()
}

/// Converts color-like tokens into a function call or a struct constructor that receives 4 flaoting point numbers
///
/// Syntax:
/// ```
/// # /*
/// rgbaf!(path, color_syntax [=> {fields}])
/// # */
/// ```
#[proc_macro]
#[proc_macro_error]
pub fn rgbaf(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let (path, color, args) = extract_path(tokens.into());
    let mut rgba = parse_color(color);
    if let Some(args) = args {
        rgba = rgba.with_idents(extract_idents(args))
    }
    match rgba {
        ColorResult::Integers(ints) => {
            let items = ints.to_float4();
            quote!(#path (#(#items),*))
        },
        ColorResult::Floats(floats) => {
            let items = floats.to_float4();
            quote!(#path (#(#items),*))
        },
        ColorResult::Exprs(streams) => {
            let streams = streams.into_iter()
                .map(|x| x.unwrap_or(quote!(1.0)));
            quote!(#path (#(#streams),*))
        },
        ColorResult::IntFields(ints, idents) => {
            let items = ints.to_float4();
            if idents.len() > 4 {
                let idents = &idents[..4];
                quote!(#path {#(#idents: #items),*, ..Default::default()})
            } else {
                quote!(#path {#(#idents: #items),*})
            }
        },
        ColorResult::FloatFields(floats, idents) =>  {
            let items = floats.to_float4();
            if idents.len() > 4 {
                let idents = &idents[..4];
                quote!(#path {#(#idents: #items),*, ..Default::default()})
            } else {
                quote!(#path {#(#idents: #items),*})
            }
        },
        ColorResult::ExprFields(streams, idents) => {
            let streams = streams.into_iter()
                .map(|x| x.unwrap_or(quote!(1.0)));
            if idents.len() > 4 {
                let idents = &idents[..4];
                quote!(#path {#(#idents: #streams),*, ..Default::default()})
            } else {
                quote!(#path {#(#idents: #streams),*})
            }
        }
    }.into()
}
