use crate::errors::Result;
//use syn::Ident;
use syn::{Lit, MetaList};

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
            [ welds(BeforeCreate(fn_to_call_before_create, async = true)) ]".to_owned())
        };

        let inner: Vec<_> = list.nested.iter().collect();

        if inner.len() > 2 {
            return badformat()
        }

        let mut is_async = false;

        if inner.len() == 2 {
            match inner[1] {
                syn::NestedMeta::Meta(syn::Meta::NameValue(option)) => {
                    if &option.path.segments[0].ident.to_string() != "async" {
                        return badformat()
                    }
                    match &option.lit {
                        Lit::Bool(bool) => {
                            is_async = bool.value;
                        },
                        _ => return badformat()
                    }
                },
                _ => return badformat(),
            };
        }

        let callback = match inner[0] {
            syn::NestedMeta::Meta(m) => m,
            _ => return badformat(),
        };
        let callback = match callback {
            syn::Meta::Path(path) => path,
            _ => return badformat(),
        };

        Ok(Self {
            kind,
            callback: callback.clone(),
            is_async
        })
    }
}
