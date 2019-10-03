#![recursion_limit = "512"]

extern crate proc_macro;
use proc_macro::TokenStream;
use syn::DeriveInput;

#[proc_macro_derive(SnafuCliDebug)]
pub fn derive_parser(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;

    let impl_block = quote::quote! {
        impl ::std::fmt::Debug for #name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                ::std::write!(f, "{}", self)?;

                let mut e: &dyn ::std::error::Error = self;
                while let Some(source) = e.source() {
                    ::std::write!(f, "\nInfo: caused by {}", source)?;
                    e = source;
                }

                if ::std::env::var("RUSTC_BACKTRACE").is_ok()
                    || ::std::env::var("RUST_BACKTRACE").is_ok() {
                    if let Some(backtrace) = ::snafu::ErrorCompat::backtrace(&self) {
                        ::std::write!(f, "\n{}", backtrace)?;
                    }
                }

                Ok(())
            }
        }
    };

    impl_block.into()
}
