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

#[allow(unused)]
macro_rules! token_error {
    ($ ($ arg : tt) *) => {{
        let err_msg = format!($($arg)*);
        proc_macro::TokenStream::from(quote::quote! {compile_error!(#err_msg);})
    }}
}

/// A proc macro to define currencies using a csv file.
/// The file has to be in the format `Name,Code,DecimalUnits`:
/// ```csv
/// "US Dollar",USD,2
/// "Imaginary Currency",IMC,4
/// "Swiss Franc",CHF,2
/// ```
#[cfg(feature = "csv")]
#[proc_macro]
pub fn define_currency_csv(input: TokenStream) -> TokenStream {
    let path = syn::parse_macro_input!(input as syn::LitStr);
    let file = std::fs::File::open(path.value()).expect("cannot open file");
    let entries: Result<Vec<_>, TokenStream> = csv::ReaderBuilder::new()
        .flexible(false)
        .has_headers(false)
        .from_reader(file)
        .records()
        .enumerate()
        .map(|(idx, result)| {
            let record = result
                .map_err(|e| token_error!("An error happened while reading the csv file: {}", e))?;

            Ok(Entry {
                name: record
                    .get(0)
                    .ok_or(token_error!("Missing name (index 0) on line {}", idx + 1))?
                    .into(),
                code: record
                    .get(1)
                    .ok_or(token_error!("Missing code (index 1) on line {}", idx + 1))?
                    .into(),
                units: record
                    .get(2)
                    .ok_or(token_error!("Missing units (index 2) on line {}", idx + 1))
                    .and_then(|u| {
                        u.parse().map_err(|e| {
                            token_error!("Malformed units (index 2) on line {}: {}", idx + 1, e)
                        })
                    })?,
            })
        })
        .collect();

    match entries {
        Ok(entries) => define_currency(entries.into_iter()),
        Err(e) => e,
    }
}

#[doc(hidden)]
#[cfg(not(feature = "csv"))]
#[proc_macro]
pub fn define_currency_csv(_: TokenStream) -> TokenStream {
    token_error!("You must enable the \"csv\" feature to use this proc macro!")
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
        array.elems.iter().enumerate().map(|(idx, elem)| {
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
                        _ => Err(token_error!("Tuple at index {} is malformed.\nThe tuple must contain three valid literals: name (str), code (str), units (u8)", idx)),
                    }
                },
                _ => Err(token_error!("The currency array should contain tuples!")),
            }
        }).collect()
    };

    match maybe_entries {
        Ok(entries) => define_currency(entries.into_iter()),
        Err(error) => error,
    }
}

/// Define currencies by loading a TOML file. The file
/// should be in one of the following formats:
///
/// ```toml
/// # Using an array of tables
/// [[currency]]
/// name = "US Dollar"
/// code = "USD"
/// units = 2
///
/// [[currency]]
/// name = "Swiss Franc"
/// code = "CHF"
/// units = 2
/// ```
///
/// ```toml
/// # Using an array of inline tables
/// currency = [
///     { name = "US Dollar", code = "USD", units = 2 },
///     { name = "Swiss Franc", code = "CHF", units = 2 }
/// ]
/// ```
#[cfg(feature = "toml")]
#[proc_macro]
pub fn define_currency_toml(input: TokenStream) -> TokenStream {
    use std::convert::TryFrom;
    use toml::Value;

    let maybe_entries: Result<Vec<Entry>, TokenStream> = {
        let path = syn::parse_macro_input!(input as syn::LitStr);
        let maybe_content = std::fs::read(path.value())
            .map_err(|e| token_error!("Error while reading the path: {}", e));

        let maybe_parsed: Result<Value, _> = maybe_content.and_then(|content| {
            toml::from_slice(&content)
                .map_err(|e| token_error!("Error while parsing TOML file: {}", e))
        });

        maybe_parsed.and_then(|parsed| {
            parsed
                .get("currency")
                .ok_or(token_error!(
                    r#"The TOML file must contain an Array of Tables named "currency""#
                ))
                .and_then(|currencies| {
                    currencies.as_array().ok_or_else(|| {
                        token_error!(
                            "Expected array named \"currency\", found {}",
                            currencies.type_str()
                        )
                    })
                })
                .and_then(|array| {
                    array
                        .iter()
                        .enumerate()
                        .map(|(idx, element)| {
                            element.as_table().map(|table| (idx, table)).ok_or_else(|| {
                                token_error!(
                                    "Expected table in array at index {}, found {}",
                                    idx,
                                    element.type_str()
                                )
                            })
                        })
                        .map(|maybe_table| match maybe_table {
                            Ok((idx, table)) => {
                                let name = table
                                    .get("name")
                                    .ok_or_else(|| {
                                        token_error!("Missing field \"name\" at index {}", idx)
                                    })
                                    .and_then(|name| {
                                        name.as_str().ok_or_else(|| {
                                            token_error!(
                                                "Expected string \"name\" at index {}, found {:?}",
                                                idx,
                                                name.type_str()
                                            )
                                        })
                                    })?;

                                let code = table
                                    .get("code")
                                    .ok_or_else(|| {
                                        token_error!("Missing field \"code\" at index {}", idx)
                                    })
                                    .and_then(|code| {
                                        code.as_str().ok_or_else(|| {
                                            token_error!(
                                                "Expected string \"code\" at index {}, found {:?}",
                                                idx,
                                                code.type_str()
                                            )
                                        })
                                    })?;

                                let units = table
                                    .get("units")
                                    .ok_or_else(|| {
                                        token_error!("Missing field \"units\" at index {}", idx)
                                    })
                                    .and_then(|units| {
                                        units.as_integer().ok_or_else(|| {
                                            token_error!(
                                            "Expected integer \"units\" at index {}, found {:?}",
                                            idx,
                                            units.type_str()
                                        )
                                        })
                                    })
                                    .and_then(|units| {
                                        u8::try_from(units).map_err(|e| {
                                            token_error!(
                                        "Integer \"units\" at index {} cannot be cast to an u8: {}",
                                        idx,
                                        e
                                    )
                                        })
                                    })?;

                                Ok(Entry {
                                    name: name.to_string(),
                                    code: code.to_string(),
                                    units,
                                })
                            }
                            Err(e) => Err(e),
                        })
                        .collect()
                })
        })
    };

    match maybe_entries {
        Ok(entries) => define_currency(entries.into_iter()),
        Err(e) => e,
    }
}

#[doc(hidden)]
#[cfg(not(feature = "toml"))]
#[proc_macro]
pub fn define_currency_toml(_: TokenStream) -> TokenStream {
    token_error!("You must enable the \"toml\" feature to use this proc macro!")
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

            impl monet_traits::Currency for #identifier {
                const UNITS: u8 = #units;

                const CODE: &'static str = #code;

                const NAME: &'static str = #name;
            }
        };

        TokenStream::from(currency)
    })
    .collect()
}
