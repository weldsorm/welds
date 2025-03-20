use proc_macro::TokenStream;
use quote::quote;

pub(crate) mod attributes;
pub(crate) mod blocks;
pub(crate) mod column;
pub(crate) mod errors;
pub(crate) mod hook;
pub(crate) mod info;
pub(crate) mod relation;
pub(crate) mod utils;

use info::Info;

#[proc_macro_derive(WeldsModel, attributes(welds, welds_path))]
pub fn model_gen(input: TokenStream) -> TokenStream {
    match model_gen_inner(input) {
        Ok(q) => q,
        Err(err) => quote! { std::compile_error!(#err); }.into(),
    }
}

fn model_gen_inner(input: TokenStream) -> errors::Result<TokenStream> {
    // Gather the Info needed to build all the code snipits
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let info = Info::new(&ast)?;

    // write all the code snipits
    let p1 = blocks::has_schema(&info);
    let p2 = blocks::write_to_args(&info);
    //let p3 = blocks::write_bulk_array_to_args(&info);
    let p4 = blocks::define_schema(&info);
    let p5 = blocks::table_info(&info);
    let p6 = blocks::table_columns(&info);
    let p7 = blocks::relations(&info);
    let p8 = blocks::unique_identifier(&info);
    let p9 = blocks::impl_struct(&info);
    let p10 = blocks::try_from_row(&info);
    let p11 = blocks::update_from_row(&info);
    let p12 = blocks::write_col_default_check(&info);
    let p13 = blocks::write_hooks(&info);
    let p14 = blocks::write_primary_key_value(&info);
    let p15 = blocks::foreign_key_partial_eq(&info);

    let q = quote! {
        #p1
        #p2
        //#p3
        #p4
        #p5
        #p6
        #p7
        #p8
        #p9
        #p10
        #p11
        #p12
        #p13
        #p14
        #p15
    };

    // // Want to see what the macros generate?
    // // this works, or `rustaceanvim :RustLsp expandMacro`
    // let code = q.to_string();
    // std::fs::create_dir_all("/tmp/weldsmacro/");
    // let filename = format!(
    //     "/tmp/weldsmacro/{}.rs",
    //     info.defstruct.to_string().to_lowercase()
    // );
    // std::fs::write(filename, code);

    Ok(q.into())
}
