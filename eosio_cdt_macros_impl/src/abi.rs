use crate::proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream, Result};
use syn::punctuated::Punctuated;
use syn::{parse_macro_input, Expr, Ident, Token};

struct AbiPair {
    code: Option<Expr>,
    action: Ident,
}

impl Parse for AbiPair {
    fn parse(input: ParseStream) -> Result<Self> {
        let action: Ident = input.parse()?;
        // Ok(AbiPair { code: None, action })
        // TODO
        match input.parse::<Token![@]>() {
            Ok(_) => {
                let code: Expr = input.parse()?;
                Ok(AbiPair {
                    code: Some(code),
                    action,
                })
            }
            Err(_) => Ok(AbiPair { code: None, action }),
        }
    }
}

struct AbiPairs(Vec<AbiPair>);

impl Parse for AbiPairs {
    fn parse(input: ParseStream) -> Result<Self> {
        let parsed =
            Punctuated::<AbiPair, Token![,]>::parse_separated_nonempty(input)?;
        let pairs: Vec<AbiPair> = parsed.into_iter().collect();
        Ok(AbiPairs(pairs))
    }
}

pub fn expand(input: TokenStream) -> TokenStream {
    let pairs = parse_macro_input!(input as AbiPairs);
    let eosio = crate::paths::eosio();
    let actions = pairs.0.into_iter().map(|pair| {
        let code = pair
            .code
            .map(|code| quote!(#eosio::n!(#code)))
            .unwrap_or_else(|| quote!(receiver));
        let action = pair.action;
        quote! {
            else if code == #code && action == #eosio::n!(#action) {
                #action();
            }
        }
    });
    let expanded = quote! {
        #[no_mangle]
        pub extern "C" fn apply(receiver: u64, code: u64, action: u64) {
            std::panic::set_hook(Box::new(|panic_info| {
                let payload = panic_info.payload();
                let message = payload
                    .downcast_ref::<&str>()
                    .map(|s| s.to_string())
                    .or_else(|| payload.downcast_ref::<String>().map(|s| s.to_string()))
                    .unwrap_or_else(|| panic_info.to_string());
                #eosio::check(false, &message);
            }));
            if action == #eosio::n!(onerror) {
                assert!(
                    code == #eosio::n!(eosio),
                    "onerror action's are only valid from the \"eosio\" system account"
                );
            }
            #(#actions)*
            else if code == receiver {
                #eosio::check(false, "unknown action");
            }
        }
    };
    expanded.into()
}