#![forbid(unsafe_code, future_incompatible, rust_2018_idioms)]
#![deny(missing_debug_implementations, nonstandard_style)]
#![recursion_limit = "512"]

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{quote, quote_spanned};
use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;

/// Enables an async main function.
///
/// # Examples
///
/// ## Dynamic threads
///
/// By default, this spawns as many threads as is in the `SMOL_THREADS` environment variable, or 1
/// if it is not specified.
///
/// ```ignore
/// #[smol_potat::main]
/// async fn main() -> std::io::Result<()> {
///     Ok(())
/// }
/// ```
///
/// ## Automatic Threadpool
///
/// Alternatively, `smol_potat::main` can used to automatically
/// set the number of threads by adding the `auto` feature (off
/// by default).
///
/// ```ignore
/// #[smol_potat::main] // with 'auto' feature enabled
/// async fn main() -> std::io::Result<()> {
///     Ok(())
/// }
/// ```
///
/// ## Manually Configure Threads
///
/// To manually set the number of threads, add this to the attribute:
///
/// ```ignore
/// #[smol_potat::main(threads=3)]
/// async fn main() -> std::io::Result<()> {
///     Ok(())
/// }
/// ```
///
/// ## Set the crate root
///
/// By default `smol-potat` will use `::smol_potat` as its crate root, but you can override this
/// with the `crate` option:
///
/// ```ignore
/// use smol_potat as other_smol_potat;
///
/// #[smol_potat::main(crate = "other_smol_potat")]
/// async fn main() -> std::io::Result<()> {
///     Ok(())
/// }
/// ```
#[proc_macro_attribute]
pub fn main(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::ItemFn);
    let opts = syn::parse_macro_input!(attr as Opts);

    let ret = &input.sig.output;
    let name = &input.sig.ident;
    let body = &input.block;
    let attrs = &input.attrs;

    let crate_root = opts.crate_root;

    if name != "main" {
        return TokenStream::from(quote_spanned! { name.span() =>
            compile_error!("only the main function can be tagged with #[smol::main]"),
        });
    }

    if !input.sig.inputs.is_empty() {
        return TokenStream::from(quote_spanned! { input.sig.paren_token.span =>
            compile_error!("the main function cannot take parameters"),
        });
    }

    if input.sig.asyncness.is_none() {
        return TokenStream::from(quote_spanned! { input.span() =>
            compile_error!("the async keyword is missing from the function declaration"),
        });
    }

    let threads = match opts.threads {
        Some((num, span)) => {
            let num = num.to_string();
            Some(quote_spanned!(span=> #num))
        }
        #[cfg(feature = "auto")]
        None => Some(quote! {
            #crate_root::std::string::ToString::to_string(
                &#crate_root::std::cmp::max(#crate_root::num_cpus::get(), 1)
            )
        }),
        #[cfg(not(feature = "auto"))]
        None => None,
    };

    let set_threads = threads.map(|threads| {
        quote! {
            #crate_root::std::env::set_var(
                "SMOL_THREADS",
                #threads,
            );
        }
    });

    let result = quote! {
        fn main() #ret {
            #(#attrs)*
            async fn main() #ret {
                #body
            }

            #set_threads

            #crate_root::async_io::block_on(main())
        }
    };

    result.into()
}

/// Enables an async test function.
///
/// # Examples
///
/// ```ignore
/// #[smol_potat::test]
/// async fn my_test() -> std::io::Result<()> {
///     assert_eq!(2 * 2, 4);
///     Ok(())
/// }
/// ```
#[proc_macro_attribute]
pub fn test(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::ItemFn);
    let opts = syn::parse_macro_input!(attr as Opts);

    let ret = &input.sig.output;
    let name = &input.sig.ident;
    let body = &input.block;
    let attrs = &input.attrs;

    let crate_root = opts.crate_root;

    if let Some((_, span)) = opts.threads {
        return TokenStream::from(quote_spanned! { span=>
            compile_error!("tests cannot have threads attribute"),
        });
    }
    if !input.sig.inputs.is_empty() {
        return TokenStream::from(quote_spanned! { input.span() =>
            compile_error!("tests cannot take parameters"),
        });
    }
    if input.sig.asyncness.is_none() {
        return TokenStream::from(quote_spanned! { input.span() =>
            compile_error!("the async keyword is missing from the function declaration"),
        });
    }

    let result = quote! {
        #[test]
        #(#attrs)*
        fn #name() #ret {
            #crate_root::async_io::block_on(async { #body })
        }
    };

    result.into()
}

/// Enables an async benchmark function.
///
/// # Examples
///
/// ```ignore
/// #![feature(test)]
/// extern crate test;
///
/// #[smol_potat::bench]
/// async fn bench() {
///     println!("hello world");
/// }
/// ```
#[proc_macro_attribute]
pub fn bench(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::ItemFn);
    let opts = syn::parse_macro_input!(attr as Opts);

    let ret = &input.sig.output;
    let name = &input.sig.ident;
    let body = &input.block;
    let attrs = &input.attrs;

    let crate_root = opts.crate_root;

    if let Some((_, span)) = opts.threads {
        return TokenStream::from(quote_spanned! { span=>
            compile_error!("benchmarks cannot have threads attribute"),
        });
    }
    if !input.sig.inputs.is_empty() {
        return TokenStream::from(quote_spanned! { input.span() =>
            compile_error!("benchmarks cannot take parameters"),
        });
    }
    if input.sig.asyncness.is_none() {
        return TokenStream::from(quote_spanned! { input.span() =>
            compile_error!("the async keyword is missing from the function declaration"),
        });
    }

    let result = quote! {
        #[bench]
        #(#attrs)*
        fn #name(b: &mut ::test::Bencher) #ret {
            let _ = b.iter(|| {
                #crate_root::async_io::block_on(async {
                    #body
                })
            });
        }
    };

    result.into()
}

struct Opts {
    crate_root: syn::Path,
    threads: Option<(u32, Span)>,
}

impl Parse for Opts {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let mut crate_root = None;
        let mut threads = None;

        loop {
            if input.is_empty() {
                break;
            }

            let name_value: syn::MetaNameValue = input.parse()?;
            let ident = match name_value.path.get_ident() {
                Some(ident) => ident,
                None => {
                    return Err(syn::Error::new_spanned(
                        name_value.path,
                        "Must be a single ident",
                    ))
                }
            };
            match &*ident.to_string().to_lowercase() {
                "threads" => match &name_value.lit {
                    syn::Lit::Int(expr) => {
                        if threads.is_some() {
                            return Err(syn::Error::new_spanned(
                                name_value,
                                "multiple threads argments",
                            ));
                        }

                        let num = expr.base10_parse::<std::num::NonZeroU32>()?;
                        threads = Some((num.get(), expr.span()));
                    }
                    _ => {
                        return Err(syn::Error::new_spanned(
                            name_value,
                            "threads argument must be an integer",
                        ))
                    }
                },
                "crate" => match &name_value.lit {
                    syn::Lit::Str(path) => {
                        if crate_root.is_some() {
                            return Err(syn::Error::new_spanned(
                                name_value,
                                "multiple crate arguments",
                            ));
                        }

                        crate_root = Some(path.parse()?);
                    }
                    _ => {
                        return Err(syn::Error::new_spanned(
                            name_value,
                            "crate argument must be a string",
                        ))
                    }
                },
                name => {
                    return Err(syn::Error::new_spanned(
                        name,
                        "unknown attribute {}, expected `threads` or `crate`",
                    ));
                }
            }

            input.parse::<Option<syn::Token![,]>>()?;
        }

        Ok(Self {
            crate_root: crate_root.unwrap_or_else(|| syn::parse2(quote!(::smol_potat)).unwrap()),
            threads,
        })
    }
}
