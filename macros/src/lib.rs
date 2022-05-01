use proc_macro::{Delimiter, Group, Literal, TokenStream, TokenTree};

#[proc_macro]
pub fn make_crc_table(input: TokenStream) -> TokenStream {
    let mut input = input.into_iter();

    let func = match input.next() {
        Some(x @ TokenTree::Ident(_)) => x,
        Some(o) => panic!("Expected function name, found {o}"),
        None => panic!("Unexpected end of macro input"),
    };

    let comma = match input.next() {
        Some(x @ TokenTree::Punct(_)) if x.to_string() == "," => x,
        Some(o) => panic!("Expected ',', found {o}"),
        None => panic!("Unexpected end of macro input"),
    };

    let poly = match input.next() {
        Some(x @ TokenTree::Ident(_)) => x,
        Some(o) => panic!("Expected ident, found {o}"),
        None => panic!("Unexpected end of macro input"),
    };

    if input.next().is_some() {
        panic!("Unexpected trailing tokens in macro")
    }

    let mut array = TokenStream::new();
    array.extend(Some(TokenTree::from(Group::new(Delimiter::Bracket, {
        let mut items = TokenStream::new();
        for i in 0..256 {
            items.extend(Some(func.clone()));
            items.extend(Some(TokenTree::from(Group::new(Delimiter::Parenthesis, {
                let mut args = TokenStream::new();
                args.extend([
                    TokenTree::from(Literal::usize_suffixed(i)),
                    comma.clone(),
                    poly.clone(),
                ]);
                args
            }))));
            items.extend(Some(comma.clone()));
        }
        items
    }))));
    array
}
