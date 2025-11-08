use syn::{Type, TypeGroup, TypePath};

pub(crate) fn get_clause(ty: &Type, nullable: bool) -> String {
    let base_type = match ty {
        syn::Type::Path(tp) => get_clause_typepath(tp),
        syn::Type::Group(g) => get_clause_typegroup(g),
        _ => None,
    }
    .unwrap_or("Basic")
    .to_owned();
    if nullable {
        return format!("{}Opt", base_type);
    }
    base_type
}

fn get_clause_typegroup(_ty: &TypeGroup) -> Option<&'static str> {
    // If we ever need a special clause for Vec<T>
    None
}

fn get_clause_typepath(ty: &TypePath) -> Option<&'static str> {
    let ident = ty
        .path
        .get_ident()
        .or(ty.path.segments.first().map(|f| &f.ident))?;
    let name = ident.to_string();

    let clause = match name.as_str() {
        "u8" => "Numeric",
        "i8" => "Numeric",
        "u16" => "Numeric",
        "i16" => "Numeric",
        "u32" => "Numeric",
        "i32" => "Numeric",
        "u64" => "Numeric",
        "i64" => "Numeric",
        "f32" => "Numeric",
        "f64" => "Numeric",
        "String" => "Text",
        "chrono" => "Numeric",
        "PgMoney" => "Numeric",
        "DateTime" => "Numeric",
        "Date" => "Numeric",
        //"Time" => "Numeric", // Not time, time wraps every day
        _ => return None,
    };
    Some(clause)
}

pub(crate) fn as_typepath(ty: &syn::Type) -> Option<&syn::TypePath> {
    match ty {
        syn::Type::Path(tp) => Some(tp),
        _ => None,
    }
}
