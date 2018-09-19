extern crate bytecodec;
extern crate proc_macro;
extern crate quote;
extern crate syn;
extern crate trackable;

use proc_macro::TokenStream;

#[proc_macro_derive(Encode)]
pub fn derive_encode(input: TokenStream) -> TokenStream {
    panic!()
}

#[proc_macro_derive(Decode)]
pub fn derive_decode(input: TokenStream) -> TokenStream {
    panic!()
}
