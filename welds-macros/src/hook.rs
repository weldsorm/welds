use crate::errors::Result;
//use syn::Ident;
use syn::MetaList;

/// User has defined a Hook on the model

#[derive(Debug)]
pub(crate) struct Hook {
    pub(crate) kind: HookKind,
    pub(crate) callback: syn::Path,
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
            Err("Expected Hook to be in the format\n[ welds(BeforeCreate(fn_to_call_before_create) )]".to_owned())
        };

        let inner: Vec<_> = list.nested.iter().collect();

        if inner.len() != 1 {
            return badformat();
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
        })
    }
}
