use super::Relation;
use super::{read_as_path, read_as_string};
use crate::errors::Result;
use syn::Ident;
use syn::MetaList;

impl Relation {
    pub(crate) fn build_jointable(list: &MetaList) -> Result<Vec<Self>> {
        let badformat = || FORMAT_ERR_MANUAL.to_owned();

        let inner: Vec<_> = list.nested.iter().collect();
        if inner.len() != 4 {
            return Err(badformat());
        }

        // Read the Rust path to the Related Model out as MetaPath
        let model_a = read_as_path(list, 0).ok_or_else(badformat)?;
        let model_a_key = read_as_string(list, 1).ok_or_else(badformat)?;

        let model_b = read_as_path(list, 2).ok_or_else(badformat)?;
        let model_b_key = read_as_string(list, 3).ok_or_else(badformat)?;

        let span = model_a.segments.last().unwrap().ident.span();
        let field_a = model_a.segments.last().unwrap().ident.to_string();
        let field_a = to_snake_case(&field_a);
        let field_a = Ident::new(&field_a, span);

        let field_b = model_b.segments.last().unwrap().ident.to_string();
        let field_b = to_snake_case(&field_b);
        let field_b = Ident::new(&field_b, span);

        let kind = Ident::new("BelongsTo", span);

        let list = vec![
            Self {
                kind: kind.clone(),
                foreign_struct: model_a,
                field: field_a,
                foreign_key_db: model_a_key,
                self_key_db: None,
                is_jointable: true,
            },
            Self {
                kind: kind.clone(),
                foreign_struct: model_b,
                field: field_b,
                foreign_key_db: model_b_key,
                self_key_db: None,
                is_jointable: true,
            },
        ];

        Ok(list)
    }
}

const FORMAT_ERR_MANUAL: &str = "Invalid Format For JoinTable:
JoinTable should be in for format of
[ welds(JoinTable(struct_a, struct_a_field_key, struct_b, struct_b_field_key) )]
Example:
[ welds(JoinTable(Dog, \"dog_id\", Cat, \"cat_id\") )]
";

pub fn to_snake_case(input: &str) -> String {
    use std::char;

    #[derive(Copy, Clone, PartialEq, Eq)]
    enum Kind {
        Upper,
        Lower,
        Digit,
        Other,
    }

    fn kind(c: char) -> Kind {
        if c.is_uppercase() {
            Kind::Upper
        } else if c.is_lowercase() {
            Kind::Lower
        } else if c.is_numeric() {
            Kind::Digit
        } else {
            Kind::Other
        }
    }

    let mut out = String::with_capacity(input.len());
    let mut chars = input.chars().peekable();

    // Track the kind of the last alphanumeric we emitted.
    let mut prev_kind: Option<Kind> = None;
    // Track whether the last emitted char was an underscore to avoid doubles.
    let mut last_was_us = false;

    while let Some(c) = chars.next() {
        let k = kind(c);

        match k {
            Kind::Other => {
                // Turn any separator/punctuation into a single underscore.
                if !out.is_empty() && !last_was_us {
                    out.push('_');
                    last_was_us = true;
                }
                // Reset alnum state.
                prev_kind = None;
            }
            Kind::Upper | Kind::Lower | Kind::Digit => {
                // Decide if we should insert an underscore before this char.
                let mut need_us = false;

                if let Some(pk) = prev_kind {
                    // lower/digit followed by Upper => boundary (e.g., "fooBar", "v2Alpha")
                    if (pk == Kind::Lower || pk == Kind::Digit) && k == Kind::Upper {
                        need_us = true;
                    }
                    // Upper run that ends before a Lower => split before the last upper
                    // e.g., "HTTPRequest" => "HTTP_Request" (we want "http_request")
                    // When we see the 'R' (upper) and next is lower, add underscore now.
                    if pk == Kind::Upper && k == Kind::Upper {
                        if let Some(&next) = chars.peek() {
                            if kind(next) == Kind::Lower {
                                need_us = true;
                            }
                        }
                    }
                    // Transition Upper/Lower to Digit (or vice versa) => boundary: "ver2Update", "ID3Tag"
                    if (pk == Kind::Digit && (k == Kind::Upper || k == Kind::Lower))
                        || ((pk == Kind::Upper || pk == Kind::Lower) && k == Kind::Digit)
                    {
                        need_us = true;
                    }
                } else {
                    // If previous was a separator we may have pushed an underscore already.
                    // Nothing to do here.
                }

                if need_us && !last_was_us && !out.is_empty() {
                    out.push('_');
                    //last_was_us = true;
                }

                // Emit the lowercase version (to_lowercase may yield multiple chars).
                for lc in c.to_lowercase() {
                    out.push(lc);
                }
                last_was_us = false;

                // Update prev_kind but treat emitted letters as their letter/digit kind.
                prev_kind = Some(k);
            }
        }
    }

    // Trim trailing underscore if any.
    if out.ends_with('_') {
        out.pop();
    }

    // Also trim a leading underscore (could happen if the input starts with punctuation/space).
    while out.starts_with('_') {
        out.remove(0);
    }

    // Collapse any doubles that might slip through (belt-and-suspenders).
    // (Keeps this single-pass; cheap on small strings.)
    let mut cleaned = String::with_capacity(out.len());
    let mut prev_us = false;
    for ch in out.chars() {
        if ch == '_' {
            if !prev_us {
                cleaned.push(ch);
            }
            prev_us = true;
        } else {
            cleaned.push(ch);
            prev_us = false;
        }
    }

    cleaned
}

#[cfg(test)]
mod tests {
    use super::to_snake_case;

    #[test]
    fn basic() {
        assert_eq!(to_snake_case("HelloWorld"), "hello_world");
        assert_eq!(to_snake_case("helloWorld"), "hello_world");
        assert_eq!(to_snake_case("HTTPRequest"), "http_request");
        assert_eq!(to_snake_case("userID"), "user_id");
        assert_eq!(to_snake_case("JSON2XML"), "json_2_xml");
        assert_eq!(to_snake_case("version2Update"), "version_2_update");
    }

    #[test]
    fn separators() {
        assert_eq!(
            to_snake_case(" mixed-CASE  and  spaces "),
            "mixed_case_and_spaces"
        );
        assert_eq!(to_snake_case("__weird__--name!!"), "weird_name");
        assert_eq!(to_snake_case("already_snake"), "already_snake");
    }

    #[test]
    fn unicode() {
        // Works with Unicode letters/digits using the standard library.
        assert_eq!(to_snake_case("Straße"), "straße");
        assert_eq!(to_snake_case("МояСтрока"), "моя_строка");
    }
}
