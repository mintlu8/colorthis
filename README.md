# colorthis [![Latest Version]][crates.io] [Docs]

[crates.io]: https://crates.io/crates/colorthis
[Docs]: (https://docs.rs/colorthis/latest/colorthis/)

Meta macros that aid macro authors to create colors from a generalized syntax.

## General Syntax

```rust
rgb!($path: path, $color_syntax: tt [=> {$($fields: ident)*}])
```

Where path is a function or a struct constructor

If fields are not specified,

```rust
rgba!(Cmyk, [123, 155, 224, 155]);
```

produces

```rust
Cmyk(123, 155, 224, 155);
```

If fields are specified,

```rust
rgba!(Cmyk, [123, 155, 224, 155] => {c, m, y, k});
```

produces

```rust
Cmyk {
    c: 123,
    m: 155,
    y: 224,
    k: 155,
};
```

Populating a larger struct is allowed as long as the struct implements Default.

```rust
# use colorthis::rgba;
#[derive(Default)]
struct PixelData {
    r: u8, g: u8, b: u8, a: u8,
    z: u8, brightness: u8, magic_number: u8
}
rgba!(PixelData, [1, 2, 3, 4] => {r, g, b, a, _});
```

produces

```rust
PixelData {
    r: 1,
    g: 2,
    b: 3,
    a: 4,
    ..Default::default()
};
```

## Example usage

```rust
macro_rules! bevy_rgba {
    ($color: tt) => {
        ::colorthis::rgbaf!(::bevy::prelude::Color::Rgba, $color => {red, green, blue, alpha})
    };
    ($($colors: literal),* $(,)?) => {
        ::colorthis::rgbaf!(::bevy::prelude::Color::Rgba, [$($colors),*] => {red, green, blue, alpha})
    };
}
bevy_rgba!("00EEFFFF");
bevy_rgba!("#AAEE22");
bevy_rgba!(0xAABBCC);
bevy_rgba!(72, 184, 255, 255);
bevy_rgba!(0.23, 0.44, 0.89, 0.50);
bevy_rgba!(Red);
bevy_rgba!(Violet800);
```

## Color Syntax

The color is always a TokenTree `tt`.

* Bracketed numbers: `[0.3, 0.72, 0.98]`, `[124, 54, 87, 255]`
* Parenthesised expressions: `(0.3, 0.72, 0.98)`, `(r, g, b + g, a + 0.5)`
* Splat syntax: `[0.3; 3]`, `[0.3; 3, 0.8]`, `[0.7; 4]`
* Hex strings: `"AABBCC"`, `"AABBCCFF"`, `"#AABBCC"`, `"#AABBCCFF"`
* Hex number literals: `0xAABBCC`, `0xAABBCCFF`
* CSS color names: `Red`, `Blue`
* TailwindCSS color names: `Red100`, `Sky400`

## Details

### Bracketed Numbers

If all values are integers,
they are considered to be in range `0..=255`.

```rust
assert_eq!(rgba!(Color, [32, 144, 220, 125]), Color(32,144,220,125));
```

If any value is a float,
values are considered to be in range `0.0..=1.0`.

Therefore this fails to compile since numbers like 144 overflows this bounds.

```rust
assert_eq!(rgba!(Color, [0.5, 144, 220, 125]), Color(32, 144, 220, 125));
```

However if all values are `0`s or `1`s, and the macro invoked
is `rgbf!()` or `rgbaf!()` `1`s are treated as `1.0`s.

```rust
assert_eq!(rgbaf!(ColorF, [255, 255, 255, 255]), ColorF(1.0, 1.0, 1.0, 1.0));
assert_eq!(rgbaf!(ColorF, [1.0, 0.5, 1, 0]), ColorF(1.0, 0.5, 1.0, 0.0));
assert_eq!(rgbaf!(ColorF, [1, 0, 1, 0]), ColorF(1.0, 0.0, 1.0, 0.0));
```

### Parenthesised Expressions

Currently we do not modify the expression or provide type conversion,
aside from validating the number of expressions,
and providing default alpha value if needed.

### Splat Syntax

* `[v; 3]` means `[v, v, v, 255]`
* `[v; 3, a]` means `[v, v, v, a]`
* `[v; 4]` means `[v, v, v, v]`

### Color Names

We relies on a [crate](https://docs.rs/parse-color/0.1.0/parse_color/)
to parse and generate these data at compile time. No external support required.

## Feature Flags

### `unchecked` and `clamp`

By default, we assert integers are in `0..=255`, floats are in `0.0..=1.0`

if unchecked is not specified, this fails

```rust
rgba!(color, [1000, 255, 128, 0]); // 1000 is not in 0..=255
```

With `unchecked`, this *might* compile, assuming color accepts `1000` as an input.

```rust
rgba!(color, [1000, 255, 128, 0]);
```

With `clamp`, this compiles and clamps `1000` into `255`

```rust
rgba!(color, [1000, 255, 128, 0]); // produces color(255, 255, 128, 0)
```

## `compact`

Compact allows 3 or 4 letter compact colors to be compiled.

```rust
rgba!(color, 0xABC); // compiles to 0xAABBCC
rgba!(color, "1234"); // compiles to "11223344"
rgba!(color, "#FFF"); // compiles to "FFFFFF"
```
