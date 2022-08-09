/*!
TODO DOC
*/

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[cfg(test)]
mod tests;

pub fn clap_logger(input: TokenStream) -> TokenStream {
	let input: DeriveInput = parse_macro_input!(input as DeriveInput);
	let expanded: quote!();
	TokenStream::from(expanded)
}
