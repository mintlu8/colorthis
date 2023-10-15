use proc_macro2::{TokenStream, TokenTree, Span, Delimiter, Ident, Literal};
use proc_macro_error::abort;

#[derive(Debug)]
pub(crate) enum ColorResult {
    Integers([i64; 4]),
    Floats([f64; 4]),
    Exprs([Option<TokenStream>; 4]),
    IntFields([i64; 4], Vec<Ident>),
    FloatFields([f64; 4], Vec<Ident>),
    ExprFields([Option<TokenStream>; 4], Vec<Ident>),
}

impl ColorResult {
    pub fn with_idents(self, idents: Vec<Ident>) -> Self{
        match self {
            ColorResult::Integers(x) => ColorResult::IntFields(x, idents),
            ColorResult::Floats(x) => ColorResult::FloatFields(x, idents),
            ColorResult::Exprs(x) => ColorResult::ExprFields(x, idents),
            _ => abort!(Span::call_site(), "Probably a bug: Idents already filled.")
        }
    }

    pub fn check(&mut self, span: Span) {
        #[cfg(not(feature="unchecked"))]
        if !match self {
            ColorResult::Integers(i) => i.into_iter().all(|x| (0..=255).contains(x)),
            ColorResult::Floats(f) => f.into_iter().all(|x| (0.0..=1.0).contains(x)),
            ColorResult::Exprs(_) => true,
            ColorResult::IntFields(i, _) => i.into_iter().all(|x| (0..=255).contains(x)),
            ColorResult::FloatFields(f, _) => f.into_iter().all(|x| (0.0..=1.0).contains(x)),
            ColorResult::ExprFields(_, _) => true,
        } {
            abort!(span, "Color out of bounds: {:?}", self)
        }
        #[cfg(feature="clamp")]
        match self {
            ColorResult::Integers(i) => i.iter_mut().for_each(|x| *x = (*x).clamp(0, 255)),
            ColorResult::Floats(f) => f.iter_mut().for_each(|x| *x = x.clamp(0.0, 1.0)),
            ColorResult::Exprs(..) => (),
            ColorResult::IntFields(i, _) => i.iter_mut().for_each(|x| *x = (*x).clamp(0, 255)),
            ColorResult::FloatFields(f, _) => f.iter_mut().for_each(|x| *x = x.clamp(0.0, 1.0)),
            ColorResult::ExprFields(..) => (),
        }
    }
}

pub(crate) fn extract_idents(stream: TokenStream) -> Vec<Ident> {
    let mut idents = Vec::new();
    let mut iter = stream.into_iter();
    while let Some(item) = iter.next() {
        if idents.len() > 4 { abort!(item, "Expecting 3 or 4 items, found more.") }
        let span = item.span();
        match item {
            TokenTree::Ident(ident) =>  idents.push(ident),
            _ => abort!(span, "Expected Ident.")
        }
        match iter.next() {
            Some(TokenTree::Punct(p)) if p.as_char() == ',' => (),
            None => break,
            Some(item) => abort!(item.span(), "Expecting comma or end of list."),
        }
    }
    idents
}

fn hex(a: u8, span: Span) -> i64 {
    (match a {
        b'0'..= b'9' => a - b'0',
        b'a'..= b'z' => a - b'a' + 10,
        b'A'..= b'Z' => a - b'A' + 10,
        _ => abort!(span, "Not a valid hexadecial number.")
    } as i64)
}

fn hex2(a: u8, b: u8, span: Span) -> i64 {
    (hex(a, span) << 4) + hex(b, span)
}

fn is_end_of_stream(mut stream: proc_macro2::token_stream::IntoIter) -> bool {
    match stream.next() {
        Some(TokenTree::Punct(p)) if p.as_char() == ',' => {
            stream.next().is_none()
        },
        Some(_) => false,
        None => true,
    }
}

fn parse_slice(lit: &[u8], span: Span) -> ColorResult{
    match lit.len() {
        #[cfg(feature="compact")]
        3 => ColorResult::Integers([
            hex(lit[0], span) * 17,
            hex(lit[1], span) * 17,
            hex(lit[2], span) * 17,
            255
        ]),
        #[cfg(feature="compact")]
        4 => ColorResult::Integers([
            hex(lit[0], span) * 17,
            hex(lit[1], span) * 17,
            hex(lit[2], span) * 17,
            hex(lit[3], span) * 17,
        ]),
        6 => ColorResult::Integers([
            hex2(lit[0], lit[1], span),
            hex2(lit[2], lit[3], span),
            hex2(lit[4], lit[5], span),
            255
        ]),
        8 => ColorResult::Integers([
            hex2(lit[0], lit[1], span),
            hex2(lit[2], lit[3], span),
            hex2(lit[4], lit[5], span),
            hex2(lit[6], lit[7], span),
        ]),
        _ => abort!(span, "Invalid color syntax, must be of length 6 or 8."),
    }
}

/// Separate a `TokenStream` by comma
fn parse_arguments(tokens: TokenStream) -> Vec<TokenStream> {
    let mut result = Vec::new();
    let mut buffer = Vec::new();

    for tt in tokens.into_iter() {
        match tt {
            TokenTree::Punct(p) if p.as_char() == ',' => {
                result.push(buffer.drain(..).collect())
            },
            tt => buffer.push(tt),
        }
    }
    if !buffer.is_empty() {
        result.push(buffer.drain(..).collect())
    }
    result
}

#[derive(Debug, Clone, PartialEq)]
enum KnownToken{
    Int(i64),
    Float(f64),
    Ident(String),
    Comma,
    Neg,
    SemiColon,
}

impl KnownToken {
    fn cast(&self) -> f64 {
        match self {
            Self::Int(i) => *i as f64,
            Self::Float(f) => *f,
            _ => abort!(Span::call_site(), "Expected a number."),
        }
    }

    fn parse_f64(&self) -> f64 {
        match self {
            Self::Int(i) => *i as f64 / 255.0,
            Self::Float(f) => *f,
            _ => abort!(Span::call_site(), "Expected a number."),
        }
    }

    fn make_neg(&mut self, span: Span) {
        match self {
            Self::Int(i) => *i = -*i,
            Self::Float(f) => *f = -*f,
            _ => abort!(span, "Invalid token: -."),
        }
    }
}

fn parse_tt(tt: TokenTree) -> KnownToken{
    let span = tt.span();
    let lit = match tt {
        TokenTree::Literal(lit) => lit,
        TokenTree::Group(g) if g.delimiter() == Delimiter::None => {
            let mut iter = g.stream().into_iter();
            match (iter.next(), iter.next()) {
                (Some(tt), None) => return parse_tt(tt),
                _ => abort!(span, "Expected number literal, found {}.", g)
            }
        },
        TokenTree::Punct(p) if p.as_char() == ',' => return KnownToken::Comma,
        TokenTree::Punct(p) if p.as_char() == ';' => return KnownToken::SemiColon,
        TokenTree::Punct(p) if p.as_char() == '-' => return KnownToken::Neg,
        _ => abort!(span, "Expected number literal, found {}.", tt)
    };
    if let Ok(lit) = litrs::IntegerLit::try_from(lit.clone()) {
        if let Some(i) = lit.value::<i64>() {
            #[cfg(not(feature="unchecked"))]
            if !(0..=255).contains(&i) {
                abort!(span, "Integer has to be in range 0..=255.")
            }
            return KnownToken::Int(i);
        } else {
            abort!(span, "Integer overflow.")
        }
    } else if let Ok(lit) = litrs::FloatLit::try_from(lit.clone()) {
        use std::str::FromStr;
        if let Ok(f) = f64::from_str(lit.number_part()) {
            #[cfg(not(feature="unchecked"))]
            if !(0.0..=1.0).contains(&f) {
                abort!(span, "Float has to be in range 0.0..=1.0.")
            }
            return KnownToken::Float(f);
        } else {
            abort!(span, "float overflow.")
        }
    } else {
        abort!(span, "Expected number literal, found {}", lit)
    }
}

pub(crate) fn parse_lit(literal: Literal) -> ColorResult {
    let lit = literal.to_string();
    let lit = lit.as_bytes();
    if lit.get(0..2) == Some(b"\"#") {
        match lit.len() {
            #[cfg(feature="compact")]
            6 => parse_slice(&lit[2..5], literal.span()),
            #[cfg(feature="compact")]
            7 => parse_slice(&lit[2..6], literal.span()),
            9 => parse_slice(&lit[2..8], literal.span()),
            11 => parse_slice(&lit[2..10], literal.span()),
            _ => abort!(literal.span(), "Invalid color syntax, must be of length 6 or 8."),
        }
    } else if lit.first() == Some(&b'"') {
        match lit.len() {
            #[cfg(feature="compact")]
            5 => parse_slice(&lit[1..4], literal.span()),
            #[cfg(feature="compact")]
            6 => parse_slice(&lit[1..5], literal.span()),
            8 => parse_slice(&lit[1..7], literal.span()),
            10 => parse_slice(&lit[1..9], literal.span()),
            _ => abort!(literal.span(), "Invalid color syntax, must be of length 6 or 8."),
        }
    } else if lit.get(0..2) == Some(b"0x") || lit.get(0..2) == Some(b"0X") {
        match lit.len() {
            #[cfg(feature="compact")]
            5 => parse_slice(&lit[2..5], literal.span()),
            #[cfg(feature="compact")]
            6 => parse_slice(&lit[2..6], literal.span()),
            8 => parse_slice(&lit[2..8], literal.span()),
            10 => parse_slice(&lit[2..10], literal.span()),
            _ => abort!(literal.span(), "Invalid color syntax, must be of length 6 or 8."),
        }
    } else {
        abort!(literal.span(), "Invalid color syntax.");
    }
}

macro_rules! iof {
    (_) => {
        KnownToken::Int(_)|KnownToken::Float(_)
    };
    ($i: ident) => {
        $i @ (KnownToken::Int(_)|KnownToken::Float(_))
    };
}

pub(crate) fn parse_numbers(exprs: TokenStream, span: Span) -> ColorResult {
    use KnownToken::*;
    let mut tokens: Vec<_> = exprs.into_iter().map(|tt| parse_tt(tt)).collect();
    if tokens.last() == Some(&KnownToken::Comma){
        tokens.pop();
    }
    for i in 0..tokens.len() - 1 {
        if tokens[i] == KnownToken::Neg {
            tokens[i + 1].make_neg(span)
        }
    }
    tokens.retain(|x| x != &KnownToken::Neg);
    match tokens.as_slice() {
        [Int(a), Comma, Int(b), Comma, Int(c)] => {
            ColorResult::Integers([*a, *b, *c, 255])
        },
        [Int(a), Comma, Int(b), Comma, Int(c), Comma, Int(d)] => {
            ColorResult::Integers([*a, *b, *c, *d])
        },
        [iof!(a), Comma, iof!(b), Comma, iof!(c)] => {
            ColorResult::Floats([a.cast(), b.cast(), c.cast(), 1.0])
        },
        [iof!(a), Comma, iof!(b), Comma, iof!(c), Comma, iof!(d)] => {
            ColorResult::Floats([a.cast(), b.cast(), c.cast(), d.cast()])
        },
        [Int(a), SemiColon, Int(b)] => {
            match b {
                3 => ColorResult::Integers([*a, *a, *a, 255]),
                4 => ColorResult::Integers([*a, *a, *a, *a]),
                _ => abort!(span, "Splat can only have value 3 or 4.")
            }
        },
        [Float(a), SemiColon, Int(b)] => {
            match b {
                3 => ColorResult::Floats([*a, *a, *a, 1.0]),
                4 => ColorResult::Floats([*a, *a, *a, *a]),
                _ => abort!(span, "Splat can only have value 3 or 4.")
            }
        },
        [Int(a), SemiColon, Int(3), Comma, Int(c)] => {
            ColorResult::Integers([*a, *a, *a, *c])
        },
        [iof!(a), SemiColon, Int(3), Comma, iof!(c)] => {
            ColorResult::Floats([a.parse_f64(), a.parse_f64(), a.parse_f64(), c.parse_f64()])
        },
        [iof!(_), SemiColon, _, Comma, iof!(_)] => {
            abort!(span, "Splat with alpha can only have value 3.")
        },
        _ => abort!(span, "Unknown color syntax.")
    }
}

pub(crate) fn parse_exprs(exprs: TokenStream, span: Span) -> ColorResult {
    // Maybe TODO: add support for parsing numbers
    let args = parse_arguments(exprs);
    if args.len() != 3 && args.len() != 4 {
        abort!(span, "Exprect 3 or 4 items.")
    }
    let mut result = [None, None, None, None];
    for (i, v) in args.into_iter().enumerate(){
        result[i] = Some(v);
    }
    ColorResult::Exprs(result)
}

fn u8_to_i64(v: [u8; 4]) -> [i64;4] {
    [
        v[0] as i64,
        v[1] as i64,
        v[2] as i64,
        v[3] as i64,
    ]
}

pub(crate) fn parse_color(tokens: TokenTree) -> ColorResult {
    let span = tokens.span();
    let mut result = match tokens {
        #[cfg(feature="parse-color")]
        TokenTree::Ident(name) => {
            let s = name.to_string();
            #[cfg(feature="tailwind")]
            if let Some(num) = s.find(|x| ('0'..='9').contains(&x)) {
                let (color, right) = s.split_at(num);
                if let Ok(index) = right.parse() {
                    if let Some(color) = parse_color::parse_tailwind(color, index){
                        return ColorResult::Integers(u8_to_i64(color));
                    }
                }
            }
            if let Some(color) = parse_color::parse(&s) {
                ColorResult::Integers(u8_to_i64(color))
            } else {
                abort!(name.span(), "Invalid color name: {}.", name)
            }
        },
        TokenTree::Literal(lit) => parse_lit(lit),
        TokenTree::Group(group) if group.delimiter() == Delimiter::Parenthesis => {
            parse_exprs(group.stream(), group.span())
        },
        TokenTree::Group(group) if group.delimiter() == Delimiter::Bracket => {
            parse_numbers(group.stream(), group.span())
        },
        tt => abort!(tt.span(), "Invalid color syntax: {}.", tt),
    };
    result.check(span);
    result
}
