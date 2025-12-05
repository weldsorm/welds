use proc_macro2::{TokenStream, TokenTree};
use quote::ToTokens;
use crate::errors::Result;
//use syn::Ident;
use syn::{MetaList, Path};

/// User has defined a Hook on the model

#[derive(Debug)]
pub(crate) struct Hook {
    pub(crate) kind: HookKind,
    pub(crate) callback: syn::Path,
    pub(crate) is_async: bool,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub(crate) enum HookKind {
    BeforeCreate,
    BeforeUpdate,
    BeforeDelete,
    AfterCreate,
    AfterUpdate,
    AfterDelete,
}

impl Hook {
    pub(crate) fn new(list: &MetaList, kind: HookKind) -> Result<Self> {
        let badformat = || {
            Err("Expected Hook to be one of the following format(s):\n\
            [ welds(BeforeCreate(fn_to_call_before_create)) ]\n\
            [ welds(BeforeCreate(fn_to_call_before_create, async = true)) ]"
                .to_owned())
        };

        let list= &list.tokens.clone().into_iter().collect::<Vec<_>>();
        if list.len() > 5 {
            return badformat();
        }

        let mut is_async = false;

        if list.len() == 5 {
            match &list[3] {
                TokenTree::Punct(punct)=>
                if punct.as_char() != '=' {
                    return badformat();
                }
                _ => return badformat(),
            }
            match &list[2] {
                TokenTree::Ident(ident)=> {
                    if ident.to_string() != "async" {
                        return badformat();
                    }
                }
                _ => return badformat(),
            }

            match &list[4] {
                TokenTree::Ident(ident) => {
                    if ident.to_string()=="true" {
                        is_async = true;
                    } else if ident.to_string()=="false" {
                        is_async = false;
                    } else {
                        return badformat();
                    }
                },
                _ => return badformat(),
            }
        }

        // Convert the TokenTree to TokenStream
        let token_stream = {
            let mut tokens = TokenStream::new();
            list[0].to_tokens(&mut tokens);
            tokens
        };

        let callback: syn::Result<Path> =syn::parse2(token_stream);

        let callback = match callback {
            Ok(path) => path,
            _ => return badformat(),
        };

        Ok(Self {
            kind,
            callback: callback.clone(),
            is_async,
        })
    }
}
