use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use std::collections::HashSet;
use syn::{parse_macro_input, parse_quote, Data, DeriveInput, Fields, Item, Type};

/// Add an attribute to a struct that sets up and invokes `derive(CppCodegen)` below.
#[proc_macro_attribute]
pub fn cpp_codegen(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as Item);

    let mut item_struct = match item {
        Item::Struct(item_struct) => item_struct,
        _ => {
            return TokenStream::from(quote! {
                compile_error!("cpp_codegen attribute can only be applied to structs");
            });
        }
    };

    item_struct.attrs.push(parse_quote! {
        #[derive(aetools_derive::CppCodegen)]
    });

    Item::Struct(item_struct).into_token_stream().into()
}

/// Create `Type::hpp(…)` method.
#[proc_macro_derive(CppCodegen)]
pub fn derive_cpp_codegen(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    if !(input.generics.lt_token.is_none()
        && input.generics.params.is_empty()
        && input.generics.gt_token.is_none()
        && input.generics.where_clause.is_none())
    {
        return TokenStream::from(quote! {
            compile_error!("Deriving CppCodegen is not implemented for generic types");
        });
    }

    let Data::Struct(data_struct) = input.data else {
        return TokenStream::from(quote! {
            compile_error!("Deriving CppCodegen is only implemented for structs");
        });
    };

    let Fields::Named(fields_named) = data_struct.fields else {
        return TokenStream::from(quote! {
            compile_error!("Deriving CppCodegen is only implemented for named fields");
        });
    };

    let ident = input.ident;
    let vis = input.vis;

    let mut hpp_lines = Vec::<String>::new();

    let namespace = "AtelierEsri";

    let mut system_includes = HashSet::<String>::new();
    let mut user_includes = HashSet::<String>::new();

    let mut fields_lines = Vec::<String>::new();

    for field in fields_named.named {
        let Some(name) = field.ident.as_ref() else {
            return TokenStream::from(quote! {
                compile_error!("Deriving CppCodegen failed because we couldn't get a field's name");
            });
        };
        let Type::Path(type_path) = &field.ty else {
            return TokenStream::from(quote! {
                compile_error!("Deriving CppCodegen failed because we couldn't get a field's type path");
            });
        };
        let Some(last) = type_path.path.segments.iter().last() else {
            return TokenStream::from(quote! {
                compile_error!("Deriving CppCodegen failed because we couldn't get a field's last type path segment");
            });
        };
        let rust_type = last.ident.to_string();
        // TODO: generalize to CppType below
        let cpp_type = match rust_type.as_ref() {
            "i16" => {
                system_includes.insert("cstdint".to_string());
                "std::int16_t".to_string()
            }
            _ => rust_type,
        };
        fields_lines.push(format!("{type} {name};", type = cpp_type, name = name.to_string()));
    }

    for (l_delim, includes, r_delim) in vec![('<', system_includes, '>'), ('"', user_includes, '"')]
    {
        if includes.is_empty() {
            continue;
        }

        let mut includes = includes.into_iter().collect::<Vec<String>>();
        includes.sort();
        for include in includes {
            hpp_lines.push(format!("#include {l_delim}{include}{r_delim}"));
        }
        hpp_lines.push("".to_string());
    }

    hpp_lines.push(format!("namespace {namespace} {{"));
    hpp_lines.push("".to_string());

    hpp_lines.push(format!("struct {ident} {{", ident = ident.to_string()));

    for field_line in fields_lines {
        hpp_lines.push(format!("  {field_line}"));
    }

    hpp_lines.push(format!("}}"));
    hpp_lines.push("".to_string());

    hpp_lines.push(format!("}}  // namespace {namespace}"));
    hpp_lines.push("".to_string());

    // TODO: size_t FooStruct::read(void*, size_t)

    let hpp_text = hpp_lines.join("\n");

    let hpp_doc = "Generate C++ headers for this structure";

    let expanded = quote! {
        #[automatically_derived]
        impl #ident {
            #[doc = #hpp_doc]
            #vis fn hpp() -> &'static str {
                #hpp_text
            }
        }
    };

    TokenStream::from(expanded)
}

// TODO: should be able to handle primitive types, strings, structs, arrays of either primitives or structs
//  and know which types have alignment requirements afterward (strings, bytes, byte arrays)
//  and know which header to include
// TODO: (later) pack adjacent bools into bytes
struct CppType {}
