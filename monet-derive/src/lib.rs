//! A macro that loads currencies from a file at compile time.
//! It creates the module `currency` containing various empty structs (e.g. `pub struct USD;`).
//!
//! # Usage
//!
//! ```ignore
//! use monet_derive::*;
//!
//! define_currency_csv!("path/to/file.csv");
//!
//! define_currency_array!([
//!     ("US Dollar", "USD", 2),
//!     ("Swiss Franc", "CHF" 2),
//! ]);
//! ```
//!
//! It will also define `currency::Unknown`, a variant used only when creating a Money with
//! a currency only known at runtime. It is later possible to cast it into a more specific type.

extern crate proc_macro;
use proc_macro::TokenStream;
use std::fs;

// const DEFAULT_PATH: &str = "default_currencies.json";

mod error;
use error::Error;

#[cfg(feature = "csv")]
#[proc_macro]
pub fn define_currency_csv(input: TokenStream) -> TokenStream {
    let path = syn::parse_macro_input!(input as syn::LitStr);
    let file = fs::File::open(path.value()).expect("cannot open file");
    let entries: Result<Vec<_>, Error> = csv::ReaderBuilder::new()
        .flexible(false)
        .has_headers(false)
        .from_reader(file)
        .records()
        .enumerate()
        .map(|(idx, result)| {
            let record = result.map_err(|e| Error::Other(idx, Box::new(e)))?;

            Ok(Entry {
                name: record.get(0).ok_or(Error::MissingName(idx))?.into(),
                code: record.get(1).ok_or(Error::MissingCode(idx))?.into(),
                units: record
                    .get(2)
                    .ok_or(Error::MissingUnits(idx))
                    .and_then(|u| u.parse().map_err(|e| Error::MalformedUnits(idx, e)))?,
            })
        })
        .collect();

    match entries {
        Ok(entries) => define_currency(entries.into_iter()),
        Err(e) => panic!("{}", e),
    }
}

/// Define currencies here, by providing an array of tuples in the following format:
/// 
/// ```
/// # mod hidden {
/// use monet_derive::*;
/// define_currency_array!([("US Dollar", "USD", 2), ("Swiss Franc", "CHF", 2)]);
/// # }
/// ```
/// 
/// It is good practice to put it in a module called `currency`, but you can really do whatever
/// you want with it, as long as it in the right location. Currently function-like proc-macros cannot
/// be expanded into statements, so you cannot put the first example into a function body, unless you
/// wrap it into a module like this:
/// 
/// ```
/// mod currency {
///     use monet_derive::*;
///     define_currency_array!([("My currency", "CODE", 3)]);
/// }
/// ```
/// 
/// Wrong syntax will produce a compilation error
/// 
/// ```compile_fail
/// use monet_derive::*;
/// define_currency_array!(
///     [
///         ("US Dollar", NOT_A_STRING, 2),
///     ]
/// );
/// ```
#[proc_macro]
pub fn define_currency_array(input: TokenStream) -> TokenStream {
    let array = syn::parse_macro_input!(input as syn::ExprArray);

    let maybe_entries: Result<Vec<_>, TokenStream> = {
        array.elems.iter().map(|elem| {
            match elem {
                syn::Expr::Tuple(tuple) => {
                    let record: Vec<_> = tuple.elems.iter().collect();
                    match (record[0], record[1], record[2]) {
                        (
                            syn::Expr::Lit(syn::ExprLit {lit: syn::Lit::Str(name), ..}),
                            syn::Expr::Lit(syn::ExprLit {lit: syn::Lit::Str(code), ..}),
                            syn::Expr::Lit(syn::ExprLit {lit: syn::Lit::Int(units), ..})
                        ) => {
                            Ok(Entry {name: name.value(), code: code.value(), units: units.base10_digits().parse().expect("malformed units")})
                        },
                        _ => Err(TokenStream::from(quote::quote! {compile_error!("The tuple must contain three valid literals: name (str), code (str), units (u8)")}))?,
                    }
                },
                _ => Err(TokenStream::from(quote::quote! {compile_error!("The currency array should contain tuples!")}))?,
            }
        }).collect()
    };

    match maybe_entries {
        Ok(entries) => define_currency(entries.into_iter()),
        Err(error) => error,
    }
}

#[derive(Debug)]
struct Entry {
    name: String,
    code: String,
    units: u8,
}

fn define_currency<I: Iterator<Item = Entry>>(iter: I) -> TokenStream {
    use proc_macro2::{Ident, Span};

    iter.map(|entry| {
        let Entry { name, units, code } = entry;

        let identifier = Ident::new(&code, Span::call_site());

        let currency = quote::quote! {
            #[derive(Debug, PartialEq, Eq)]
            pub struct #identifier;

            impl monet_traits::Currency<'static> for #identifier {
                const UNITS: u8 = #units;

                const CODE: &'static str = #code;

                const NAME: &'static str = #name;
            }
        };

        TokenStream::from(currency)
    }).collect()
}
