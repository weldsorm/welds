use crate::hook::HookKind;
use crate::info::Info;
use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn write(info: &Info) -> TokenStream {
    // If this is a readonly model it should NOT impl Save Hooks
    if info.readonly {
        return quote!();
    }

    let before_create = write_before_create(info);
    let after_create = write_after_create(info);

    let before_update = write_before_update(info);
    let after_update = write_after_update(info);

    let before_delete = write_before_delete(info);
    let after_delete = write_after_delete(info);

    quote! {
        #before_create
        #after_create
        #before_update
        #after_update
        #before_delete
        #after_delete
    }
}

// Create

pub(crate) fn write_before_create(info: &Info) -> TokenStream {
    let def = &info.defstruct;
    let wp = &info.welds_path;

    // build the inner content to call all the callback functions
    let hook_calls: Vec<_> = info
        .hooks
        .iter()
        .filter(|h| h.kind == HookKind::BeforeCreate)
        .map(|h| {
            let func = &h.callback;
            if h.is_async {
                quote! { #func(self).await?; }
            } else {
                quote! { #func(self)?; }
            }
        })
        .collect();
    let hook_calls = quote! { #(#hook_calls)* };

    quote! {
        impl #wp::model_traits::hooks::BeforeCreate for #def {
            async fn before(&mut self) -> #wp::errors::Result<()> {
                #hook_calls
                Ok(())
            }
        }
    }
}

pub(crate) fn write_after_create(info: &Info) -> TokenStream {
    let def = &info.defstruct;
    let wp = &info.welds_path;

    // build the inner content to call all the callback functions
    let hook_calls: Vec<_> = info
        .hooks
        .iter()
        .filter(|h| h.kind == HookKind::AfterCreate)
        .map(|h| {
            let func = &h.callback;
            if h.is_async {
                quote! { #func(self).await; }
            } else {
                quote! { #func(self); }
            }
        })
        .collect();
    let hook_calls = quote! { #(#hook_calls)* };

    quote! {
        impl #wp::model_traits::hooks::AfterCreate for #def {
            async fn after(&self) -> #wp::errors::Result<()> {
                #hook_calls
                Ok(())
            }
        }
    }
}

// Update

pub(crate) fn write_before_update(info: &Info) -> TokenStream {
    let def = &info.defstruct;
    let wp = &info.welds_path;

    // build the inner content to call all the callback functions
    let hook_calls: Vec<_> = info
        .hooks
        .iter()
        .filter(|h| h.kind == HookKind::BeforeUpdate)
        .map(|h| {
            let func = &h.callback;
            if h.is_async {
                quote! { #func(self).await?; }
            } else {
                quote! { #func(self)?; }
            }
        })
        .collect();
    let hook_calls = quote! { #(#hook_calls)* };

    quote! {
        impl #wp::model_traits::hooks::BeforeUpdate for #def {
            async fn before(&mut self) -> #wp::errors::Result<()> {
                #hook_calls
                Ok(())
            }
        }
    }
}

pub(crate) fn write_after_update(info: &Info) -> TokenStream {
    let def = &info.defstruct;
    let wp = &info.welds_path;

    // build the inner content to call all the callback functions
    let hook_calls: Vec<_> = info
        .hooks
        .iter()
        .filter(|h| h.kind == HookKind::AfterUpdate)
        .map(|h| {
            let func = &h.callback;
            if h.is_async {
                quote! { #func(self).await; }
            } else {
                quote! { #func(self); }
            }
        })
        .collect();
    let hook_calls = quote! { #(#hook_calls)* };

    quote! {
        impl #wp::model_traits::hooks::AfterUpdate for #def {
            async fn after(&self) -> #wp::errors::Result<()> {
                #hook_calls
                Ok(())
            }
        }
    }
}

// Delete

pub(crate) fn write_before_delete(info: &Info) -> TokenStream {
    let def = &info.defstruct;
    let wp = &info.welds_path;

    // build the inner content to call all the callback functions
    let hook_calls: Vec<_> = info
        .hooks
        .iter()
        .filter(|h| h.kind == HookKind::BeforeDelete)
        .map(|h| {
            let func = &h.callback;
            if h.is_async {
                quote! { #func(self).await?; }
            } else {
                quote! { #func(self)?; }
            }
        })
        .collect();
    let hook_calls = quote! { #(#hook_calls)* };

    quote! {
        impl #wp::model_traits::hooks::BeforeDelete for #def {
            async fn before(&self) -> #wp::errors::Result<()> {
                #hook_calls
                Ok(())
            }
        }
    }
}

pub(crate) fn write_after_delete(info: &Info) -> TokenStream {
    let def = &info.defstruct;
    let wp = &info.welds_path;

    // build the inner content to call all the callback functions
    let hook_calls: Vec<_> = info
        .hooks
        .iter()
        .filter(|h| h.kind == HookKind::AfterDelete)
        .map(|h| {
            let func = &h.callback;
            if h.is_async {
                quote! { #func(self).await; }
            } else {
                quote! { #func(self); }
            }
        })
        .collect();
    let hook_calls = quote! { #(#hook_calls)* };

    quote! {
        impl #wp::model_traits::hooks::AfterDelete for #def {
            async fn after(&self) -> #wp::errors::Result<()> {
                #hook_calls
                Ok(())
            }
        }
    }
}
