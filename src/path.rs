use proc_macro2::{TokenStream, TokenTree, Span, Spacing, Delimiter};
use proc_macro_error::abort;
use quote::quote;

enum PathSep {
    DoubleColon,
    Dot,
    Break,
}

impl PathSep {
    pub fn write_to(&self, tokens: &mut TokenStream) {
        match &self {
            PathSep::DoubleColon => tokens.extend(quote!(::)),
            PathSep::Dot => tokens.extend(quote!(.)),
            PathSep::Break => (),
        }
    }
}

fn extract_sep(iter: &mut proc_macro2::token_stream::IntoIter) -> PathSep{
    match iter.next() {
        Some(TokenTree::Punct(punct)) if punct.as_char() == ':' && punct.spacing() == Spacing::Joint => {
            match iter.next() {
                Some(TokenTree::Punct(punct)) if punct.as_char() == ':' => PathSep::DoubleColon,
                Some(tt) => abort!(tt.span(), "Expected '::'."),
                None => abort!(punct.span(), "Expected '::'."),
            }
        }
        Some(TokenTree::Punct(punct)) if punct.as_char() == '.' => {
            PathSep::Dot
        },
        Some(TokenTree::Punct(punct)) if punct.as_char() == ',' => {
            PathSep::Break
        },
        Some(tt) => abort!(tt.span(), "Expected one of ',', '.' or '::'."),
        None => abort!(Span::call_site(), "Expected one of ',', '.' or '::'."),
    }
}

/// Separate caller paths like `Rgba::new`, `::colors::rgba::rgba`, `color.set`, etc.
pub fn extract_path(tokens: TokenStream) -> (TokenStream, TokenTree, Option<TokenStream>) {
    let mut tokens = tokens.into_iter();
    let mut path = Vec::new();
    let path = loop {
        match tokens.next() {
            Some(TokenTree::Punct(p)) if p.as_char() == ',' => {
                break path.into_iter().collect();
            }
            Some(other) => {
                path.push(other)
            }
            None => {
                abort!(Span::call_site(), "Expected path.")
            }
        }
    };
    let color = match tokens.next(){
        Some(x) => x,
        None => abort!(Span::call_site(), "Expected color token tree."),
    };
    match (tokens.next(), tokens.next()) {
        (None, _) => return (path, color, None),
        (Some(TokenTree::Punct(a)), Some(TokenTree::Punct(b))) => {
            if a.as_char() != '=' || b.as_char() != '>' || a.spacing() != Spacing::Joint{
                abort!(a.span(), "Expected => , or end.")
            }
        }
        (Some(TokenTree::Punct(p)), None) => {
            if p.as_char() == ',' {
                return (path, color, None)
            } else {
                abort!(p.span(), "Expected => , or end.")
            }
        }
        (Some(tt), _) => abort!(tt.span(), "Expected => , or end.")
    }

    let fields = match tokens.next() {
        Some(TokenTree::Group(g)) if g.delimiter() == Delimiter::Brace => {
            g.stream()
        },
        Some(tt) => abort!(tt.span(), "Expected {fields}."),
        None => abort!(Span::call_site(), "Expected {fields}."),
    };

    match tokens.next() {
        Some(TokenTree::Punct(p)) if p.as_char() == ',' => {
            match tokens.next() {
                Some(t) => abort!(t.span(), "Expected end of arguments."),
                None => (),
            }
        },
        Some(tt) => abort!(tt.span(), "Expected end of arguments."),
        None => (),
    };

    (path, color, Some(fields))
}

#[cfg(test)]
mod test {
    use super::extract_path;
    use quote::quote;
    use std::mem::discriminant;

    #[macro_export]
    macro_rules! tokenstream_eq {
        ($left: expr, $right: expr ) => {
            for (a, b) in $left.into_iter().zip($right.into_iter()){
                assert_eq!(discriminant(&a), discriminant(&b));
                assert_eq!(a.to_string(), b.to_string())
            }
        };
    }

    #[test]
    pub fn test_path_extractor() {
        let (left, right, _) = extract_path(quote!(Color, [1234, 567]));
        tokenstream_eq!(left, quote!(Color));
        assert!(matches!(right, proc_macro2::TokenTree::Group(_)));
        
        let (left, right, _) = extract_path(quote!(rgba::Color, 0xFFFFFF));
        tokenstream_eq!(left, quote!(rgba::Color));
        assert!(matches!(right, proc_macro2::TokenTree::Literal(_)));

        let (left, right, _) = extract_path(quote!(::some::path::func, Red));
        tokenstream_eq!(left, quote!(::some::path::func));
        assert!(matches!(right, proc_macro2::TokenTree::Ident(_)));
    }
}
